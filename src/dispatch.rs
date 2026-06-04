use crate::categories::Categories;
use crate::intensity::Intensity;
use crate::jitter;
use crate::tasks::{self, TaskParams};
use rand::seq::SliceRandom;

pub fn execute(intensity: Intensity, effective: Categories, jitter_enabled: bool) {
    let mut rng = rand::thread_rng();
    let all = tasks::all_tasks();
    let eligible: Vec<_> = all
        .iter()
        .filter(|t| effective.contains(t.category))
        .collect();

    if eligible.is_empty() {
        return;
    }

    let base = intensity.base_params();
    let count = if jitter_enabled {
        jitter::apply(base.task_count, &mut rng)
    } else {
        base.task_count
    };

    for _ in 0..count {
        let task = eligible.choose(&mut rng).unwrap();
        let params = TaskParams {
            iterations: if jitter_enabled {
                jitter::apply(base.iteration_count, &mut rng)
            } else {
                base.iteration_count
            },
            buffer_size: if jitter_enabled {
                jitter::apply(base.buffer_size, &mut rng)
            } else {
                base.buffer_size
            },
            call_depth: if jitter_enabled {
                jitter::apply(base.call_depth, &mut rng)
            } else {
                base.call_depth
            },
        };
        (task.func)(&params, &mut rng);
    }
}
