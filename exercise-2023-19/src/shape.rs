pub struct Shape {
    pub x:u32,
    pub m:u32,
    pub a:u32,
    pub s:u32,
}

impl Shape {
    pub fn new(x:u32,m:u32,a:u32,s:u32) -> Self {
        Self { x, m, a, s }
    }
}