use crate::{shape::Shape, step::{Handler}};
use crate::workflow::Workflow;

mod shape;
mod workflow;
mod step;

fn main() {
    let mut workflow = Workflow::new("ex", "A");

    workflow.add_step(Handler::new("R", |s| s.a > 30));
    workflow.add_step(Handler::new("two", |s| s.m < 20));
    workflow.add_step(Handler::new("one", |s| s.x > 10));

    let shape = Shape::new(1,22,31,1);
    println!("{:?}", workflow);
}
