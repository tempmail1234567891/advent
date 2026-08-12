use crate::lense::Lense;

#[derive(Debug)]
pub struct Box {
    index: u8,
    values: Vec<Lense>,
}

impl Box {
    pub fn new(index: u8) -> Self {
        Self { values: vec![], index }
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
