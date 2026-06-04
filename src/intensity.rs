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
    pub(crate) fn base_params(&self) -> IntensityParams {
        match self {
            Intensity::Low => IntensityParams {
                task_count: 2,
                iteration_count: 50,
                buffer_size: 1024,
                call_depth: 2,
            },
            Intensity::Medium => IntensityParams {
                task_count: 5,
                iteration_count: 500,
                buffer_size: 16384,
                call_depth: 4,
            },
            Intensity::High => IntensityParams {
                task_count: 10,
                iteration_count: 5000,
                buffer_size: 262144,
                call_depth: 8,
            },
            Intensity::Ultra => IntensityParams {
                task_count: 20,
                iteration_count: 50000,
                buffer_size: 1048576,
                call_depth: 16,
            },
        }
    }
}
