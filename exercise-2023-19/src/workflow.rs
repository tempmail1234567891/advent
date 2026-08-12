use crate::shape::Shape;

pub struct Workflow<'a> {
    pub name: String,
    pub check: Box<dyn Fn(&Shape) -> bool + 'a>,
    pub success: Option<String>,
    pub failure: Option<String>,
}

impl<'a> Workflow<'a> {
    pub fn new<F>(name: &str, check: F) -> Self
    where
        F: Fn(&Shape) -> bool + 'a,
    {
        Self {
            name: String::from(name),
            check: Box::new(check),
            success: None,
            failure: None,
        }
    }

    pub fn set_success(&mut self, success: &str) {
        self.success = Some(String::from(success));
    }

    pub fn set_failure(&mut self, failure: &str) {
        self.failure = Some(String::from(failure));
    }
}
