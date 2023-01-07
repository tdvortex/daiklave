use serde::{Serialize, Deserialize};

/// A piece of mundane armor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MundaneArmorMemo {
    name: String,
}