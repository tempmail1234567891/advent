use std::collections::HashMap;
use crate::workflow::Workflow;
use crate::parsing;
use crate::shape::Shape;

pub struct Program {
    workflows: HashMap<String, Workflow>
}

impl Program {
    pub fn new() -> Self{
        Self {workflows: HashMap::new()}
    }

    pub fn add_workflow(&mut self, line: &str) {
        if let Ok(workflow) = parsing::parse_workflow(line) {
            self.workflows.insert(workflow.name.clone(), workflow);
        }
        else {
            println!("Invalid workflow found: {line}");
        }
    }

    pub fn run(&self, start: &String, shape: &Shape) -> Result<String, &str> {
        if let Some(workflow) = self.workflows.get(start){
            let result = workflow.run(shape);
            if result == "R" || result == "A" {
                Ok(result)
            }
            else {
                self.run(&result, shape)
            }
        }
        else {
            Err("workflow {start} does not exists")
        }
    }
}