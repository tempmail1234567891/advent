use crate::shape::{Shape, ShapeHandler, Target};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Step {
    pub value: ShapeHandler,
    pub next: Option<Box<Step>>,
}

impl Step {
    pub fn new(value: ShapeHandler) -> Self {
        Self {
            value: value,
            next: None,
        }
    }

    pub fn push_left(&mut self, value: ShapeHandler) {
        let old = std::mem::replace(self, Step::new(value));
        self.next = Some(Box::new(old));
    }

    pub fn run(&self, shape: &Shape) -> Option<Target> {
        if (self.value.check_fn)(shape) {
            Some(self.value.target.clone())
        } else if let Some(next) = &self.next {
            next.run(shape)
        } else {
            None
        }
    }
}
