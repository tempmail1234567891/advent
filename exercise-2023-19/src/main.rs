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

    let mut accepted_shapes = vec![];

    for shape in shapes {
        match program.run(&start, &shape) {
            Ok(result) => {
                if result == shape::Target::Accepted {
                    accepted_shapes.push(shape);
                }
            },
            Err(error) => println!("Error: {:?}", error),
        }
    }
    let sum = accepted_shapes.iter().map(|s| s.x+ s.a+s.m+s.s).sum::<i32>();
    println!("{:?}", sum);
}
