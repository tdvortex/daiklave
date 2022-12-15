use serde::{Deserialize, Serialize};
pub(crate) mod create;
pub(crate) mod destroy;
pub(crate) mod tables;
pub use create::create_player;
pub use destroy::destroy_player;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Player {
    database_id: i32,
    name: String,
}

impl Player {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            database_id: id,
            name,
        }
    }

    pub fn id(&self) -> i32 {
        self.database_id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
