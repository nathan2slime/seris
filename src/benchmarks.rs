//! Lightweight benchmark helpers for critical paths.

use std::{
    fs,
    time::{Duration, Instant},
};

/// A simple timing sample.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimingSample {
    /// Elapsed time spent in the benchmarked operation.
    pub elapsed: Duration,
    /// How many iterations were executed.
    pub iterations: usize,
}

/// Measures a closure over a fixed number of iterations.
pub fn measure<F>(iterations: usize, mut f: F) -> TimingSample
where
    F: FnMut(),
{
    let started_at = Instant::now();

    for _ in 0..iterations {
        f();
    }

    TimingSample {
        elapsed: started_at.elapsed(),
        iterations,
    }
}

/// Returns the average duration per iteration.
pub fn average_duration(sample: TimingSample) -> Duration {
    if sample.iterations == 0 {
        return Duration::from_nanos(0);
    }

    Duration::from_nanos(sample.elapsed.as_nanos() as u64 / sample.iterations as u64)
}

/// A simple memory sample.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemorySample {
    /// Resident set size before the operation.
    pub rss_before: Option<u64>,
    /// Resident set size after the operation.
    pub rss_after: Option<u64>,
    /// How many iterations were executed.
    pub iterations: usize,
}

/// Measures resident memory before and after running a closure repeatedly.
pub fn measure_memory<F>(iterations: usize, mut f: F) -> MemorySample
where
    F: FnMut(),
{
    let rss_before = current_rss_bytes();

    for _ in 0..iterations {
        f();
    }

    MemorySample {
        rss_before,
        rss_after: current_rss_bytes(),
        iterations,
    }
}

/// Returns the current resident memory usage on Linux.
pub fn current_rss_bytes() -> Option<u64> {
    let status = fs::read_to_string("/proc/self/status").ok()?;
    parse_rss_kb(&status).map(|kb| kb * 1024)
}

fn parse_rss_kb(status: &str) -> Option<u64> {
    status
        .lines()
        .find(|line| line.starts_with("VmRSS:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|value| value.parse::<u64>().ok())
}

#[cfg(test)]
mod tests {
    use super::{average_duration, current_rss_bytes, measure, measure_memory, parse_rss_kb};
    use std::time::Duration;

    #[test]
    fn measures_iterations() {
        let sample = measure(4, || {});

        assert_eq!(sample.iterations, 4);
    }

    #[test]
    fn average_duration_handles_zero_iterations() {
        assert_eq!(
            average_duration(super::TimingSample {
                elapsed: Duration::from_secs(1),
                iterations: 0
            }),
            Duration::from_nanos(0)
        );
    }

    #[test]
    fn parses_rss_kb_from_proc_status() {
        let status = "Name:\tseris\nVmRSS:\t1234 kB\n";

        assert_eq!(parse_rss_kb(status), Some(1234));
    }

    #[test]
    #[ignore]
    fn samples_current_memory_usage() {
        let sample = measure_memory(1_000, || {
            let _ = vec![0_u8; 128];
        });

        assert_eq!(sample.iterations, 1_000);
        assert!(current_rss_bytes().is_some());
        let _ = (sample.rss_before, sample.rss_after);
    }
}
