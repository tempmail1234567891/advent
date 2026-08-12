mod shape;
mod workflow;
mod program;

fn main() {
    let mut program = program::Program::new();

    program.add_workflow(workflow::Workflow::new("ex", |s| s.x > 10));
    program.add_workflow(workflow::Workflow::new("ex_success", |s| s.m < 20));
    program.add_workflow(workflow::Workflow::new("ex_failed", |s| s.m < 20));
    program.add_workflow(workflow::Workflow::new("ex_failed2", |s| s.a > 30));
    program.add_workflow(workflow::Workflow::new("accept", |s| true));
    program.add_workflow(workflow::Workflow::new("reject", |s| true));

    program.add_connection("ex", "ex_success", "ex_failed");
    program.add_failure("ex_failed", "ex_failed2");
    program.add_connection("ex_failed2", "accept", "reject");

    let shape = shape::Shape {
        x: 1,
        a: 31,
        m: 21,
        s: 1,
    };
    println!("{}", program.run("ex", &shape));
}
