use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub(in crate::weapons::hearthstone) enum GeomancyLevel {
    Standard,
    Greater,
}
