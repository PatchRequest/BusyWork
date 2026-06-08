#[cfg(feature = "cat-compute")]
pub mod compute;
#[cfg(feature = "cat-crypto")]
pub mod crypto;
#[cfg(feature = "cat-filesystem")]
pub mod filesystem;
#[cfg(feature = "cat-memory")]
pub mod memory;
#[cfg(feature = "cat-network")]
pub mod network;
#[cfg(feature = "cat-registry")]
pub mod registry;
#[cfg(feature = "cat-winapi")]
pub mod winapi_tasks;

use crate::categories::Categories;
use crate::workdata::WorkData;
use rand::rngs::ThreadRng;

pub struct TaskParams {
    pub iterations: usize,
    pub buffer_size: usize,
    pub call_depth: usize,
}

pub type TaskFn = fn(&TaskParams, &mut ThreadRng, &WorkData);

#[allow(dead_code)]
pub struct TaskDescriptor {
    pub name: &'static str,
    pub category: Categories,
    pub func: TaskFn,
}

pub fn all_tasks() -> Vec<TaskDescriptor> {
    let mut tasks = Vec::new();
    #[cfg(feature = "cat-compute")]
    tasks.extend(compute::register());
    #[cfg(feature = "cat-memory")]
    tasks.extend(memory::register());
    #[cfg(feature = "cat-filesystem")]
    tasks.extend(filesystem::register());
    #[cfg(feature = "cat-registry")]
    tasks.extend(registry::register());
    #[cfg(feature = "cat-winapi")]
    tasks.extend(winapi_tasks::register());
    #[cfg(feature = "cat-network")]
    tasks.extend(network::register());
    #[cfg(feature = "cat-crypto")]
    tasks.extend(crypto::register());
    tasks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workdata::WorkData;

    fn non_network_tasks() -> Vec<TaskDescriptor> {
        all_tasks()
            .into_iter()
            .filter(|t| !t.category.contains(Categories::NETWORK))
            .collect()
    }

    fn run_every_task(params: &TaskParams, work: &WorkData) {
        let tasks = non_network_tasks();
        let mut rng = rand::thread_rng();
        for task in &tasks {
            (task.func)(params, &mut rng, work);
        }
    }

    fn make_work(data: &[u8]) -> WorkData {
        let mut w = WorkData::new();
        w.feed(data);
        w
    }

    // ── Registry completeness ──────────────────────────────────────────

    #[test]
    fn registry_compute_count() {
        let n = all_tasks().iter().filter(|t| t.category == Categories::COMPUTE).count();
        assert_eq!(n, 14, "expected 14 COMPUTE tasks");
    }

    #[test]
    fn registry_memory_count() {
        let n = all_tasks().iter().filter(|t| t.category == Categories::MEMORY).count();
        assert_eq!(n, 10, "expected 10 MEMORY tasks");
    }

    #[test]
    fn registry_filesystem_count() {
        let n = all_tasks().iter().filter(|t| t.category == Categories::FILESYSTEM).count();
        assert_eq!(n, 12, "expected 12 FILESYSTEM tasks");
    }

    #[test]
    fn registry_registry_count() {
        let n = all_tasks().iter().filter(|t| t.category == Categories::REGISTRY).count();
        assert_eq!(n, 10, "expected 10 REGISTRY tasks");
    }

    #[test]
    fn registry_winapi_count() {
        let n = all_tasks().iter().filter(|t| t.category == Categories::WINAPI).count();
        assert_eq!(n, 16, "expected 16 WINAPI tasks");
    }

    #[test]
    fn registry_network_count() {
        let n = all_tasks().iter().filter(|t| t.category == Categories::NETWORK).count();
        assert_eq!(n, 7, "expected 7 NETWORK tasks");
    }

    #[test]
    fn registry_crypto_count() {
        let n = all_tasks().iter().filter(|t| t.category == Categories::CRYPTO).count();
        assert_eq!(n, 7, "expected 7 CRYPTO tasks");
    }

    #[test]
    fn registry_total() {
        assert_eq!(all_tasks().len(), 76, "expected 76 total tasks");
    }

    #[test]
    fn registry_no_duplicate_names() {
        let tasks = all_tasks();
        let mut names: Vec<&str> = tasks.iter().map(|t| t.name).collect();
        names.sort();
        let before = names.len();
        names.dedup();
        assert_eq!(names.len(), before, "duplicate task names found");
    }

    // ── Every task runs with every work-data shape ─────────────────────
    // These call every registered task function pointer directly, catching
    // panics from index-out-of-bounds, overflow, or allocation failures.

    #[test]
    fn every_task_empty_work() {
        run_every_task(
            &TaskParams { iterations: 10, buffer_size: 256, call_depth: 2 },
            &WorkData::new(),
        );
    }

    #[test]
    fn every_task_single_byte_work() {
        run_every_task(
            &TaskParams { iterations: 10, buffer_size: 256, call_depth: 2 },
            &make_work(&[0xFF]),
        );
    }

    #[test]
    fn every_task_4_bytes_work() {
        run_every_task(
            &TaskParams { iterations: 10, buffer_size: 256, call_depth: 2 },
            &make_work(&0xDEADBEEFu32.to_ne_bytes()),
        );
    }

    #[test]
    fn every_task_4kb_work() {
        run_every_task(
            &TaskParams { iterations: 10, buffer_size: 256, call_depth: 2 },
            &make_work(&vec![0xAB; 4096]),
        );
    }

    #[test]
    fn every_task_all_zeros_work() {
        run_every_task(
            &TaskParams { iterations: 10, buffer_size: 256, call_depth: 2 },
            &make_work(&[0x00; 64]),
        );
    }

    #[test]
    fn every_task_all_ff_work() {
        run_every_task(
            &TaskParams { iterations: 10, buffer_size: 256, call_depth: 2 },
            &make_work(&[0xFF; 64]),
        );
    }

    #[test]
    fn every_task_alternating_work() {
        let data: Vec<u8> = (0..256).map(|i| if i % 2 == 0 { 0xFF } else { 0x00 }).collect();
        run_every_task(
            &TaskParams { iterations: 10, buffer_size: 256, call_depth: 2 },
            &make_work(&data),
        );
    }

    // ── Parameter edge cases × work data ───────────────────────────────

    #[test]
    fn every_task_zero_params_empty_work() {
        run_every_task(
            &TaskParams { iterations: 0, buffer_size: 0, call_depth: 0 },
            &WorkData::new(),
        );
    }

    #[test]
    fn every_task_zero_params_with_work() {
        run_every_task(
            &TaskParams { iterations: 0, buffer_size: 0, call_depth: 0 },
            &make_work(&0xCAFEBABEu32.to_ne_bytes()),
        );
    }

    #[test]
    fn every_task_min_params_with_work() {
        run_every_task(
            &TaskParams { iterations: 1, buffer_size: 1, call_depth: 1 },
            &make_work(&[42]),
        );
    }

    #[test]
    fn every_task_large_params_empty_work() {
        run_every_task(
            &TaskParams { iterations: 5000, buffer_size: 262144, call_depth: 8 },
            &WorkData::new(),
        );
    }

    #[test]
    fn every_task_large_params_large_work() {
        run_every_task(
            &TaskParams { iterations: 5000, buffer_size: 262144, call_depth: 8 },
            &make_work(&vec![0xCD; 8192]),
        );
    }

    // ── Max-value work data that pushes derive_usize / blend_seed ──────

    #[test]
    fn every_task_max_seed_work() {
        run_every_task(
            &TaskParams { iterations: 10, buffer_size: 256, call_depth: 2 },
            &make_work(&[0xFF; 128]),
        );
    }

    #[test]
    fn every_task_sequential_bytes() {
        let data: Vec<u8> = (0u8..=255).collect();
        run_every_task(
            &TaskParams { iterations: 10, buffer_size: 256, call_depth: 2 },
            &make_work(&data),
        );
    }
}
