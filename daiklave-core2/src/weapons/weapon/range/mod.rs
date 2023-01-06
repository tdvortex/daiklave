mod attack_range;
mod band;

use serde::{Deserialize, Serialize};

pub use self::band::RangeBand;
pub use attack_range::AttackRange;

/// The range at which
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponRange {
    ContactOnly,
    Throwable(RangeBand),
    Archery(RangeBand),
}
