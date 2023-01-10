use serde::{Serialize, Deserialize};

/// A stability rating for a Hearthstone. 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HearthstoneStability {
    /// The Hearthstone requires a Manse to exist. If the Manse is lost,
    /// the Hearthstone is destroyed.
    Linked,
    /// The Hearthstone must be created in a Manse. Whether it can exist
    /// after the manse is destroyed is up to the Storyteller and player.
    ManseBorn,
    /// The Hearthstone must be created in a Manse, but can survive if the
    /// manse is destroyed.
    ManseBornSteady,
    /// The Hearthstone may be created in a Manse or in the wild. It can
    /// survive without a manse.
    Steady,
    /// The Hearthstone cannot be created in a Manse. It is implicitly also
    /// Steady.
    WildBorn,
    /// The Hearthstone can be manse-born, wild-born, linked, or stable.
    Unspecified,
}