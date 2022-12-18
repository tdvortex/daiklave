use serde::{Deserialize, Serialize};
pub(crate) mod create;
pub(crate) mod destroy;
pub(crate) mod tables;
pub use create::create_player;
pub use destroy::destroy_player;

use crate::id::Id;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Player {
    id: Id,
    name: String,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            id: Id::Placeholder(0),
            name: "New Player".to_owned(),
        }
    }
}

impl Player {
    pub fn new(id: Id, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
