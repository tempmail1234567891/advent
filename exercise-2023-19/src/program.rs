use std::collections::HashMap;
use crate::workflow::Workflow;
use crate::shape::Shape;

pub struct Program<'a> {
    workflows: HashMap<String, Workflow<'a>>,
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        Self {
            workflows: HashMap::new(),
        }
    }

    pub fn add_workflow(&mut self, workflow: Workflow<'a>) {
        self.workflows
            .insert(String::from(&workflow.name), workflow);
    }

    pub fn add_success(&mut self, source: &str, success: &str) {
        if self.workflows.contains_key(success) {
            if let Some(workflow) = self.workflows.get_mut(source) {
                workflow.set_success(success);
            }
        }
    }

    pub fn add_failure(&mut self, source: &str, failure: &str) {
        if self.workflows.contains_key(failure) {
            if let Some(workflow) = self.workflows.get_mut(source) {
                workflow.set_failure(failure);
            }
        }
    }

    pub fn add_connection(&mut self, source: &str, success: &str, failure: &str) {
        self.add_failure(source, failure);
        self.add_success(source, success);
    }

    pub fn run(&self, source: &str, shape: &Shape) -> &str {
        println!("Enter: {source}");
        if let Some(workflow) = self.workflows.get(source) {
            if (workflow.check)(shape) {
                if let Some(path) = &workflow.success {
                    self.run(path, shape)
                } else {
                    &workflow.name
                }
            } else {
                if let Some(path) = &workflow.failure {
                    self.run(path, shape)
                } else {
                    &workflow.name
                }
            }
        } else {
            ""
        }
    }
}