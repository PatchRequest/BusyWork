#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Intensity {
    Low,
    Medium,
    High,
    Ultra,
}

pub(crate) struct IntensityParams {
    pub task_count: usize,
    pub iteration_count: usize,
    pub buffer_size: usize,
    pub call_depth: usize,
}

impl Intensity {
    /// Work volume per `run()` call.
    ///
    /// These are intentionally an order-of-magnitude ladder. Earlier values
    /// (Medium = 500 iters / 5 tasks) collapsed to tens of milliseconds of
    /// pure compute and were further flattened by per-task `.min(N)` caps,
    /// so Medium/High/Ultra felt almost identical in wall time.
    ///
    /// Approximate pure-compute wall time on a modern desktop (no I/O):
    ///   Low ~0.2–1s · Medium ~2–8s · High ~10–30s · Ultra ~45–120s
    /// Windows I/O / WinAPI / crypto categories stretch this further.
    /// Kassandra `idle()` runs three bursts, so Medium callbacks land ~6–25s.
    pub(crate) fn base_params(&self) -> IntensityParams {
        match self {
            Intensity::Low => IntensityParams {
                task_count: 5,
                iteration_count: 5_000,
                buffer_size: 16_384,
                call_depth: 3,
            },
            Intensity::Medium => IntensityParams {
                task_count: 10,
                iteration_count: 40_000,
                buffer_size: 131_072,
                call_depth: 8,
            },
            Intensity::High => IntensityParams {
                task_count: 14,
                iteration_count: 120_000,
                buffer_size: 262_144,
                call_depth: 12,
            },
            Intensity::Ultra => IntensityParams {
                task_count: 20,
                iteration_count: 300_000,
                buffer_size: 1_048_576,
                call_depth: 16,
            },
        }
    }
}
