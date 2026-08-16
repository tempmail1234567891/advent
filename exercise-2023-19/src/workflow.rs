use crate::step::Step;
use crate::shape::{Shape, ShapeHandler, Target};

#[derive(Debug)]
pub struct Workflow {
    pub name: String,
    steps: Option<Step>,
    default: Target,
}

impl Workflow {
    pub fn new(name: &str, default: &str) -> Self {
        Self {
            name: name.to_string(),
            steps: None,
            default: Target::new(default),
        }
    }

    pub fn add_step(&mut self, value: ShapeHandler) {
        if let Some(first) = self.steps.as_mut() {
            first.push_left(value);
        } else {
            self.steps = Some(Step::new(value));
        }
    }

    pub fn run(&self, shape: &Shape) -> Target {
        if let Some(steps) = &self.steps
            && let Some(result) = steps.run(shape)
        {
            result
        } else {
            self.default.clone()
        }
    }
}
