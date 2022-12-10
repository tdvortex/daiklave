use serde::{Deserialize, Serialize};
pub(crate) mod create;
pub(crate) mod destroy;
pub(crate) mod tables;
pub use create::create_player;
pub use destroy::destroy_player;

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Player {
    id: i32,
    name: String,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
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
