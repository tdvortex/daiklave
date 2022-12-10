use serde::{Serialize, Deserialize};
pub(crate) mod tables;
pub(crate) mod create;
pub(crate) mod destroy;
pub use create::create_player;
pub use destroy::destroy_player;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
