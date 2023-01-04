use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(in crate::weapons) enum WeaponRange {
    ContactOnly,
    Throwable(RangeBand),
    Archery(RangeBand),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RangeBand {
    Close,
    Short,
    Medium,
    Long,
    Extreme
}