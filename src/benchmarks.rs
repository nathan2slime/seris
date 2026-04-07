//! Lightweight benchmark helpers for critical paths.

use std::time::{Duration, Instant};

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

#[cfg(test)]
mod tests {
    use super::{average_duration, measure};
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
}
