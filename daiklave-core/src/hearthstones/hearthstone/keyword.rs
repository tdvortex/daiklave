use serde::{Deserialize, Serialize};

/// Keywords that describe a hearthstone in terms of its origin, stability, or
/// use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HearthstoneKeyword {
    /// The hearthstone does not do anything by itself; instead, it enhances
    /// the effects of other (non-dependent) hearthstones socketed into the
    /// same artifact.
    Dependent,
    /// The hearthstone must be created in a manse and can only exist as long
    /// as the manse exists.
    Linked,
    /// The hearthstone must be created in a manse.
    ManseBorn,
    /// The hearthstone can exist without a manse (if it is destroyed, for
    /// example)
    Steady,
    /// The hearthstone must be created without a manse.
    WildBorn,
}
