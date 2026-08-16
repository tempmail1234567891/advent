pub struct Shape {
    pub x:i32,
    pub m:i32,
    pub a:i32,
    pub s:i32,
}

impl Shape {
    pub fn new(x:i32,m:i32,a:i32,s:i32) -> Self {
        Self { x, m, a, s }
    }
}