#[derive(Debug)]
pub struct Shape {
    pub x: i32,
    pub m: i32,
    pub a: i32,
    pub s: i32,
}

type CheckFn = Box<dyn Fn(&Shape) -> bool + 'static>;

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    Workflow(String),
    Accepted,
    Regected,
}

impl Target {
    pub fn new(input: &str) -> Self {
        if input == "R" {
            Target::Regected
        } else if input == "A" {
            Target::Accepted
        } else {
            Target::Workflow(input.to_string())
        }
    }
}

pub struct ShapeHandler {
    pub target: Target,
    pub check_fn: CheckFn,
}

impl ShapeHandler {
    pub fn new<F>(target: &str, check_fn: F) -> Self
    where
        F: Fn(&Shape) -> bool + 'static,
    {
        Self {
            target: Target::new(target),
            check_fn: Box::new(check_fn),
        }
    }
}

impl std::fmt::Debug for ShapeHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShapeHandler")
            .field("target", &self.target)
            .finish()
    }
}
