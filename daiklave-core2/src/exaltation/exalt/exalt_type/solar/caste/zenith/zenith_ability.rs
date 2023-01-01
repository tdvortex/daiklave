use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub(crate) enum ZenithAbility {
    Athletics,
    Integrity,
    Performance,
    Lore,
    Presence,
    Resistance,
    Survival,
    War,
}
