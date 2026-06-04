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
use rand::rngs::ThreadRng;

pub struct TaskParams {
    pub iterations: usize,
    pub buffer_size: usize,
    pub call_depth: usize,
}

pub type TaskFn = fn(&TaskParams, &mut ThreadRng);

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
