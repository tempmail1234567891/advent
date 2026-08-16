use std::fmt::Debug;
use crate::shape::Shape;

type CheckFn = Box<dyn Fn(&Shape) -> bool + 'static>;
pub struct Handler {
    pub target: String,
    pub check_fn: CheckFn,
}

impl Handler {
    pub fn new<F>(target: &str, check_fn: F) -> Self
    where
        F: Fn(&Shape) -> bool + 'static,
    {
        Self {
            target: String::from(target),
            check_fn: Box::new(check_fn),
        }
    }
}

impl std::fmt::Debug for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handler")
            .field("target", &self.target)
            .finish()
    }
}

#[derive(Debug)]
pub struct Step {
    pub value: Handler,
    pub next: Option<Box<Step>>,
}

impl Step {
    pub fn new(value: Handler) -> Self {
        Self {
            value: value,
            next: None,
        }
    }

    pub fn push_left(&mut self, value: Handler) {
        let old = std::mem::replace(self, Step::new(value));
        self.next = Some(Box::new(old));
    }

    pub fn push_right(&mut self, value: Handler) {
        let mut node = Step::new(value);
        node.next = self.next.take();
        self.next = Some(Box::new(node));
    }

    pub fn run(self, shape: &Shape)-> Option<String> {
        if (self.value.check_fn)(shape) {
            Some(self.value.target)
        }
        else if let Some(next) = self.next {
            next.run(shape)
        }
        else {
            None
        }
    }
}
