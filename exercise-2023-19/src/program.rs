use crate::shape::{Shape, Target};
use crate::workflow::Workflow;
use std::collections::HashMap;

pub struct Program {
    workflows: HashMap<String, Workflow>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            workflows: HashMap::new(),
        }
    }

    pub fn add_workflow(&mut self, workflow: Workflow) {
        self.workflows.insert(workflow.name.clone(), workflow);
    }

    pub fn run(&self, start: &String, shape: &Shape) -> Result<Target, &str> {
        if let Some(workflow) = self.workflows.get(start) {
            match workflow.run(shape) {
                Target::Accepted => Ok(Target::Accepted),
                Target::Regected => Ok(Target::Regected),
                Target::Workflow(next) => self.run(&next, shape),
            }
        } else {
            Err("workflow {start} does not exists")
        }
    }
}
