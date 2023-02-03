use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// One of the major languages spoken in Creation. Most are actually a set of
/// local dialects that are mutually intelligible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MajorLanguage {
    /// A fusion of High Realm with Old Realm for poetic purposes.
    Dragontongue,
    /// The language group of the South.
    Flametongue,
    /// The language group of the far East.
    ForestTongue,
    /// A coded language spoken by the Guild.
    GuildCant,
    /// The language of the Dragon-Blooded of the Realm.
    HighRealm,
    /// The language of the common folk of the Realm.
    LowRealm,
    /// The language of spirits and sorcery. **Note**: This
    /// language requires either Lore 1+ or Occult 1+ to take.
    OldRealm,
    /// The language group of the Scavenger Lands.
    Riverspeak,
    /// The language group of the West.
    Seatongue,
    /// The language group of the North.
    Skytongue,
}

impl Default for MajorLanguage {
    fn default() -> Self {
        Self::LowRealm
    }
}

impl Display for MajorLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MajorLanguage::Dragontongue => write!(f, "Dragontongue"),
            MajorLanguage::Flametongue => write!(f, "Flametongue"),
            MajorLanguage::ForestTongue => write!(f, "Forest-tongue"),
            MajorLanguage::GuildCant => write!(f, "Guild Cant"),
            MajorLanguage::HighRealm => write!(f, "High Realm"),
            MajorLanguage::LowRealm => write!(f, "Low Realm"),
            MajorLanguage::OldRealm => write!(f, "Old Realm"),
            MajorLanguage::Riverspeak => write!(f, "Riverspeak"),
            MajorLanguage::Seatongue => write!(f, "Seatongue"),
            MajorLanguage::Skytongue => write!(f, "Skytongue"),
        }
    }
}
