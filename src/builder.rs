use crate::categories::Categories;
use crate::dispatch;
use crate::intensity::Intensity;

pub struct BusyWork {
    intensity: Intensity,
    allow: Categories,
    deny: Categories,
    jitter: bool,
}

impl BusyWork {
    pub fn new(intensity: Intensity) -> Self {
        Self {
            intensity,
            allow: Categories::all(),
            deny: Categories::empty(),
            jitter: true,
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

    pub fn run(&self) {
        let effective = (self.allow & Categories::available()) & !self.deny;
        dispatch::execute(self.intensity, effective, self.jitter);
    }
}
