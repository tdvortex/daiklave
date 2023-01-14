use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub enum NightAbility {
    Athletics,
    Awareness,
    Dodge,
    Investigation,
    Larceny,
    Ride,
    Stealth,
    Socialize,
}
