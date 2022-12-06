#[derive(Debug, Clone)]
pub struct Player {
    id: i32,
    name: String,
}

impl Player {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
