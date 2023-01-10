use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HearthstoneKeyword {
    Dependent,
    Linked,
    ManseBorn,
    Steady,
    WildBorn,
}
