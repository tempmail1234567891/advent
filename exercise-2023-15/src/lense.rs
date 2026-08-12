#[derive(Debug)]
pub struct Lense {
    pub name: String,
    pub value: u32,
}

impl Lense {
    pub fn new(name: String, value: u32) -> Self {
        Self { name, value }
    }
}
