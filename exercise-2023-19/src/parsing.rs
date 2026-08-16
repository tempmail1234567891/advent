use crate::workflow::Workflow;
use crate::step::Handler;

pub fn parse_workflow(line: &str) -> Result<Workflow, String> {
    let (name, body) = line
        .split_once('{')
        .ok_or("missing '{'")?;

    let body = body
        .strip_suffix('}')
        .ok_or("missing '}'")?;

    let mut parts = body.split(',');

    // Last element is the default target.
    let default = parts
        .next_back()
        .ok_or("missing default target")?;

    let mut workflow = Workflow::new(name, default);

    for part in parts.rev() {
        // x>10\:one
        let (condition, target) = part
            .split_once(":")
            .ok_or_else(|| format!("invalid step: {part}"))?;

        let (field, op, value) = parse_condition(condition)?;

        let handler = match (field, op) {
            ('x', '>')
                => Handler::new(target, move |s| s.x > value),

            ('x', '<')
                => Handler::new(target, move |s| s.x < value),

            ('m', '>')
                => Handler::new(target, move |s| s.m > value),

            ('m', '<')
                => Handler::new(target, move |s| s.m < value),

            ('a', '>')
                => Handler::new(target, move |s| s.a > value),

            ('a', '<')
                => Handler::new(target, move |s| s.a < value),

            _ => return Err(format!("unsupported condition: {condition}")),
        };

        workflow.add_step(handler);
    }

    Ok(workflow)
}

fn parse_condition(condition: &str) -> Result<(char, char, i32), String> {
    let field = condition
        .chars()
        .next()
        .ok_or("empty condition")?;

    let op = condition
        .chars()
        .nth(1)
        .ok_or("missing operator")?;

    if !matches!(op, '>' | '<') {
        return Err(format!("invalid operator: {op}"));
    }

    let value = condition[2..]
        .parse::<i32>()
        .map_err(|_| format!("invalid number in: {condition}"))?;

    Ok((field, op, value))
}