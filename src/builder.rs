use crate::categories::Categories;
use crate::dispatch;
use crate::intensity::Intensity;
use crate::workdata::{FeedWork, WorkData};

pub struct BusyWork {
    intensity: Intensity,
    allow: Categories,
    deny: Categories,
    jitter: bool,
    work_data: WorkData,
}

impl BusyWork {
    pub fn new(intensity: Intensity) -> Self {
        Self {
            intensity,
            allow: Categories::all(),
            deny: Categories::empty(),
            jitter: true,
            work_data: WorkData::new(),
        }
    }

    pub fn allow(mut self, cats: Categories) -> Self {
        self.allow = cats;
        self
    }

    pub fn deny(mut self, cats: Categories) -> Self {
        self.deny = cats;
        self
    }

    pub fn jitter(mut self, enabled: bool) -> Self {
        self.jitter = enabled;
        self
    }

    /// Feed a variable from the surrounding code into the busywork block.
    /// The value is cloned immediately — the original is never touched.
    /// Tasks will weave this data into their control flow, making the block
    /// indistinguishable from real data processing.
    pub fn feed(mut self, data: &(impl FeedWork + ?Sized)) -> Self {
        self.work_data.feed(data);
        self
    }

    pub fn run(&self) {
        let effective = (self.allow & Categories::available()) & !self.deny;
        dispatch::execute(self.intensity, effective, self.jitter, &self.work_data);
    }
}
