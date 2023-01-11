use serde::{Deserialize, Serialize};

/// The category of hearthstone, based on its affinity.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HearthstoneCategory {
    /// The hearthstone comes from an Air-aligned demense.
    Air,
    /// The hearthstone comes from an Earth-aligned demense.
    Earth,
    /// The hearthstone comes from a Fire-aligned demense.
    Fire,
    /// The hearthstone comes from a Water-aligned demense.
    Water,
    /// The hearthstone comes from a Wood-aligned demense.
    Wood,
    /// The hearthstone comes from a Solar manse.
    Solar,
    /// The hearthstone comes from a Sidereal manse.
    Sidereal,
    /// The hearthstone comes from a Lunar manse.
    Lunar,
    /// The hearthstone comes from an Abyssal manse, or is otherwise heavily
    /// influenced by the Underworld.
    Abyssal,
}
