use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub(crate) enum TwilightAbility {
    Bureaucracy,
    Craft,
    Integrity,
    Investigation,
    Linguistics,
    Lore,
    Medicine,
    Occult,
}
