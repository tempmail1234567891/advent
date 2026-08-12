#[derive(Debug)]
pub struct Lense {
    name: String,
    value: u32,
}

impl Lense {
    pub fn new(name: String, value: u32) -> Self {
        Self { name, value }
    }
}

#[derive(Debug)]
pub struct Box {
    values: Vec<Lense>,
}

impl Box {
    pub fn new() -> Self {
        Self { values: vec![] }
    }
    pub fn add(&mut self, lense: Lense) {
        if let Some(found) = self.values.iter_mut().find(|l| l.name == lense.name) {
            found.value = lense.value;
        } else {
            self.values.push(lense);
        }
    }

    pub fn remove(&mut self, lense: Lense) {
        if let Some(index) = self.values.iter_mut().position(|l| l.name == lense.name) {
            self.values.remove(index);
        }
    }
}
