use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum ArmorWeight {
    Light,
    Medium,
    Heavy,
}