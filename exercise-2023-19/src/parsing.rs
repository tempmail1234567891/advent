use crate::shape::{Shape, ShapeHandler};
use crate::workflow::Workflow;

pub fn parse_workflow(line: &str) -> Result<Workflow, String> {
    let (name, body) = line.split_once('{').ok_or("missing '{'")?;

    let body = body.strip_suffix('}').ok_or("missing '}'")?;

    let mut parts = body.split(',');

    // Last element is the default target.
    let default = parts.next_back().ok_or("missing default target")?;

    let mut workflow = Workflow::new(name, default);

    for part in parts.rev() {
        // x>10\:one
        let (condition, target) = part
            .split_once(":")
            .ok_or_else(|| format!("invalid step: {part}"))?;

        let (field, op, value) = parse_condition(condition)?;

        let handler = match (field, op) {
            ('x', '>') => ShapeHandler::new(target, move |s| s.x > value),
            ('x', '<') => ShapeHandler::new(target, move |s| s.x < value),
            ('m', '>') => ShapeHandler::new(target, move |s| s.m > value),
            ('m', '<') => ShapeHandler::new(target, move |s| s.m < value),
            ('a', '>') => ShapeHandler::new(target, move |s| s.a > value),
            ('a', '<') => ShapeHandler::new(target, move |s| s.a < value),
            ('s', '>') => ShapeHandler::new(target, move |s| s.s > value),
            ('s', '<') => ShapeHandler::new(target, move |s| s.s < value),

            _ => return Err(format!("unsupported condition: {condition}")),
        };

        workflow.add_step(handler);
    }

    Ok(workflow)
}

fn parse_condition(condition: &str) -> Result<(char, char, i32), String> {
    let field = condition.chars().next().ok_or("empty condition")?;

    let op = condition.chars().nth(1).ok_or("missing operator")?;

    if !matches!(op, '>' | '<') {
        return Err(format!("invalid operator: {op}"));
    }

    let value = condition[2..]
        .parse::<i32>()
        .map_err(|_| format!("invalid number in: {condition}"))?;

    Ok((field, op, value))
}

fn parse_value(line: &mut std::str::Split<'_, char>, key: &str) -> i32 {
    line.next()
        .unwrap()
        .strip_prefix(key)
        .unwrap()
        .parse()
        .unwrap()
}
pub fn parse_shape(line: &str) -> Shape {
    let line = line
        .trim()
        .strip_prefix('{')
        .unwrap()
        .strip_suffix('}')
        .unwrap();

    let mut values = line.split(',');

    Shape {
        x: parse_value(&mut values, "x="),
        m: parse_value(&mut values, "m="),
        a: parse_value(&mut values, "a="),
        s: parse_value(&mut values, "s="),
    }
}

pub fn parse_input(input: &str) -> (Vec<Workflow>, Vec<Shape>) {
    let (workflow_text, shape_text) = input
        .split_once("\n\n")
        .expect("input must contain a blank line");

    // Parse workflows
    let workflows: Vec<Workflow> = workflow_text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_workflow(line).expect("invalid workflow"))
        .collect();

    // Parse shapes
    let shapes: Vec<Shape> = shape_text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_shape)
        .collect();

    (workflows, shapes)
}
