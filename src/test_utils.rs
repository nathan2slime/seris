use std::{sync::Arc, time::Duration};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub struct TestResponse {
    pub status: u16,
    pub body: &'static str,
    pub delay: Duration,
}

impl TestResponse {
    pub fn new(status: u16, body: &'static str) -> Self {
        Self {
            status,
            body,
            delay: Duration::from_millis(0),
        }
    }

    pub fn delayed(status: u16, body: &'static str, delay: Duration) -> Self {
        Self {
            status,
            body,
            delay,
        }
    }
}

pub struct TestHttpServer {
    pub url: String,
    request_line: Arc<Mutex<Option<String>>>,
    request_count: Arc<Mutex<usize>>,
}

impl TestHttpServer {
    pub async fn request_line(&self) -> Option<String> {
        self.request_line.lock().await.clone()
    }

    pub async fn request_count(&self) -> usize {
        *self.request_count.lock().await
    }
}

pub async fn spawn_json_server(body: &'static str) -> TestHttpServer {
    spawn_scripted_server(vec![TestResponse::new(200, body)]).await
}

pub async fn spawn_scripted_server(responses: Vec<TestResponse>) -> TestHttpServer {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test server");
    let addr = listener.local_addr().expect("server address");
    let request_line = Arc::new(Mutex::new(None));
    let request_count = Arc::new(Mutex::new(0usize));
    let request_line_writer = Arc::clone(&request_line);
    let request_count_writer = Arc::clone(&request_count);

    tokio::spawn(async move {
        for response in responses {
            if let Ok((mut socket, _)) = listener.accept().await {
                let mut buffer = [0u8; 4096];
                if let Ok(bytes_read) = socket.read(&mut buffer).await {
                    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                    let first_line = request.lines().next().unwrap_or_default().to_string();
                    *request_line_writer.lock().await = Some(first_line);
                    *request_count_writer.lock().await += 1;

                    if !response.delay.is_zero() {
                        tokio::time::sleep(response.delay).await;
                    }

                    let response = format!(
                        "HTTP/1.1 {} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        response.status,
                        response.body.len(),
                        response.body
                    );
                    let _ = socket.write_all(response.as_bytes()).await;
                }
            }
        }
    });

    TestHttpServer {
        url: format!("http://{}", addr),
        request_line,
        request_count,
    }
}
