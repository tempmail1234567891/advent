use crate::step::{Step,Handler};

#[derive(Debug)]
pub struct Workflow {
    pub name: String,
    steps: Option<Step>,
    default: String,
}

impl Workflow {
    pub fn new(name: &str, default: &str) -> Self {
        Self {
            name: String::from(name),
            steps: None,
            default: String::from(default),
        }
    }

    pub fn add_step(&mut self, value: Handler) {
        if let Some(first) = self.steps.as_mut() {
            first.push_left(value);
        }
        else {
            self.steps = Some(Step::new(value));
        }
    }
}
