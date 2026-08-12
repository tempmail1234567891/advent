#[derive(Debug)]
pub struct Lense {
    pub name: String,
    pub value: u8,
}

impl Lense {
    pub fn new(name: String, value: u8) -> Self {
        Self { name, value }
    }
}
