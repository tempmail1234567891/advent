mod parsing;
mod program;
mod shape;
mod step;
mod workflow;

fn main() {
    let mut program = program::Program::new();
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (workflows, shapes) = parsing::parse_input(&input);

    for workflow in workflows {
        program.add_workflow(workflow);
    }

    let start = String::from("in");

    for shape in shapes {
        match program.run(&start, &shape) {
            Ok(result) => println!("{:?}", result),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}
