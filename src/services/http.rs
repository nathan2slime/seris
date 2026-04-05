use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
    time::{Duration, Instant},
};

use reqwest::{RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use tokio::time::{sleep, timeout};

use crate::types::{Error, SerisError};

const MAX_ATTEMPTS: u32 = 3;
const INITIAL_BACKOFF: Duration = Duration::from_millis(200);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(3);
const CIRCUIT_FAILURE_THRESHOLD: u32 = 3;
const CIRCUIT_OPEN_DURATION: Duration = Duration::from_secs(30);

#[derive(Clone, Copy)]
pub(crate) struct RequestPolicy {
    pub attempts: u32,
    pub timeout: Duration,
    pub initial_backoff: Duration,
}

impl Default for RequestPolicy {
    fn default() -> Self {
        Self {
            attempts: MAX_ATTEMPTS,
            timeout: REQUEST_TIMEOUT,
            initial_backoff: INITIAL_BACKOFF,
        }
    }
}

struct CircuitState {
    failures: u32,
    open_until: Option<Instant>,
}

fn circuits() -> &'static Mutex<HashMap<&'static str, CircuitState>> {
    static CIRCUITS: OnceLock<Mutex<HashMap<&'static str, CircuitState>>> = OnceLock::new();
    CIRCUITS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) async fn get_json<T, F>(service: &'static str, build_request: F) -> Result<T, Error>
where
    T: DeserializeOwned,
    F: Fn() -> RequestBuilder,
{
    fetch_json_with_policy(service, build_request, RequestPolicy::default()).await
}

async fn fetch_json_with_policy<T, F>(
    service: &'static str,
    build_request: F,
    policy: RequestPolicy,
) -> Result<T, Error>
where
    T: DeserializeOwned,
    F: Fn() -> RequestBuilder,
{
    if circuit_is_open(service) {
        return Err(SerisError::CircuitOpen { service });
    }

    let mut backoff = policy.initial_backoff;

    for attempt in 0..policy.attempts {
        let response = timeout(policy.timeout, build_request().send()).await;

        match response {
            Ok(Ok(response)) => {
                if response.status().is_success() {
                    record_success(service);
                    return Ok(response.json::<T>().await?);
                }

                if is_retryable_status(response.status()) && attempt + 1 < policy.attempts {
                    record_failure(service);
                    sleep(backoff).await;
                    backoff = backoff.saturating_mul(2);
                    continue;
                }

                let status = response.status();
                record_failure(service);
                return Err(SerisError::HttpStatus { service, status });
            }
            Ok(Err(err)) => {
                if attempt + 1 < policy.attempts && is_retryable_error(&err) {
                    record_failure(service);
                    sleep(backoff).await;
                    backoff = backoff.saturating_mul(2);
                    continue;
                }

                record_failure(service);
                return Err(err.into());
            }
            Err(_) => {
                if attempt + 1 < policy.attempts {
                    record_failure(service);
                    sleep(backoff).await;
                    backoff = backoff.saturating_mul(2);
                    continue;
                }

                record_failure(service);
                return Err(SerisError::Timeout { service });
            }
        }
    }

    Err(SerisError::Timeout { service })
}

fn is_retryable_status(status: StatusCode) -> bool {
    status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
}

fn is_retryable_error(err: &reqwest::Error) -> bool {
    err.is_timeout() || err.is_connect() || err.is_request() || err.is_body()
}

fn circuit_is_open(service: &'static str) -> bool {
    let mut circuits = circuits().lock().expect("circuit mutex");

    if let Some(state) = circuits.get_mut(service) {
        if let Some(open_until) = state.open_until {
            if Instant::now() < open_until {
                return true;
            }

            state.open_until = None;
            state.failures = 0;
        }
    }

    false
}

fn record_success(service: &'static str) {
    circuits().lock().expect("circuit mutex").remove(service);
}

fn record_failure(service: &'static str) {
    let mut circuits = circuits().lock().expect("circuit mutex");
    let state = circuits.entry(service).or_insert(CircuitState {
        failures: 0,
        open_until: None,
    });

    state.failures = state.failures.saturating_add(1);
    if state.failures >= CIRCUIT_FAILURE_THRESHOLD {
        state.open_until = Some(Instant::now() + CIRCUIT_OPEN_DURATION);
    }
}

#[cfg(test)]
mod tests {
    use super::{fetch_json_with_policy, RequestPolicy};
    use crate::test_utils::{spawn_scripted_server, TestResponse};
    use serde::Deserialize;
    use std::time::Duration;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Payload {
        value: String,
    }

    #[tokio::test]
    async fn retries_until_success() {
        let server = spawn_scripted_server(vec![
            TestResponse::new(500, "fail"),
            TestResponse::new(200, r#"{"value":"ok"}"#),
        ])
        .await;

        let policy = RequestPolicy {
            attempts: 2,
            timeout: Duration::from_secs(1),
            initial_backoff: Duration::from_millis(1),
        };

        let payload = fetch_json_with_policy::<Payload, _>(
            "test-retry",
            || reqwest::Client::new().get(&server.url),
            policy,
        )
        .await
        .expect("payload");

        assert_eq!(
            payload,
            Payload {
                value: "ok".to_string()
            }
        );
        assert_eq!(server.request_count().await, 2);
    }

    #[tokio::test]
    async fn times_out_when_server_is_slow() {
        let server = spawn_scripted_server(vec![TestResponse::delayed(
            200,
            r#"{"value":"ok"}"#,
            Duration::from_millis(150),
        )])
        .await;

        let policy = RequestPolicy {
            attempts: 1,
            timeout: Duration::from_millis(20),
            initial_backoff: Duration::from_millis(1),
        };

        let err = fetch_json_with_policy::<Payload, _>(
            "test-timeout",
            || reqwest::Client::new().get(&server.url),
            policy,
        )
        .await
        .expect_err("timeout error");

        assert!(err.to_string().contains("timed out"));
    }

    #[tokio::test]
    async fn opens_circuit_after_repeated_failures() {
        let server = spawn_scripted_server(vec![
            TestResponse::new(500, "fail"),
            TestResponse::new(500, "fail"),
            TestResponse::new(500, "fail"),
        ])
        .await;

        let policy = RequestPolicy {
            attempts: 1,
            timeout: Duration::from_secs(1),
            initial_backoff: Duration::from_millis(1),
        };

        for _ in 0..3 {
            let _ = fetch_json_with_policy::<Payload, _>(
                "test-circuit",
                || reqwest::Client::new().get(&server.url),
                policy,
            )
            .await;
        }

        let error = fetch_json_with_policy::<Payload, _>(
            "test-circuit",
            || reqwest::Client::new().get(&server.url),
            policy,
        )
        .await
        .expect_err("circuit open");

        assert!(error.to_string().contains("temporarily unavailable"));
    }
}
