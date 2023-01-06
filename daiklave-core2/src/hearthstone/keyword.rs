use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HearthstoneKeyword {
    Linked,
    Steady,
    Dependent,
    ManseBorn,
    WildBorn,
}
