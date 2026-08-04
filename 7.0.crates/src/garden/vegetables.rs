#[derive(Debug)]
pub struct Asparagus {
    name: String,
}

impl Asparagus {
    pub fn constructor(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }

    pub fn get_name(&self) -> String {
        let name_clone = self.name.clone();
        name_clone
    }
}
