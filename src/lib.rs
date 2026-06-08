mod builder;
mod categories;
mod dispatch;
mod intensity;
mod jitter;
mod tasks;
mod workdata;

pub use builder::BusyWork;
pub use categories::Categories;
pub use intensity::Intensity;
pub use workdata::{FeedWork, WorkData};

pub fn busywork(intensity: Intensity) {
    BusyWork::new(intensity).run();
}

pub fn busywork_with(intensity: Intensity, categories: Categories) {
    BusyWork::new(intensity).allow(categories).run();
}
