pub struct Shape {
    pub x: i32,
    pub m: i32,
    pub a: i32,
    pub s: i32,
}

type CheckFn = Box<dyn Fn(&Shape) -> bool + 'static>;
pub struct ShapeHandler {
    pub target: String,
    pub check_fn: CheckFn,
}

impl ShapeHandler {
    pub fn new<F>(target: &str, check_fn: F) -> Self
    where
        F: Fn(&Shape) -> bool + 'static,
    {
        Self {
            target: target.to_string(),
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
