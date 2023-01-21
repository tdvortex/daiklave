use serde::{Deserialize, Serialize};

/// A keyword for a Solar charm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub enum SolarCharmKeyword {
    /// Aggravated damage-dealing
    Aggravated,
    /// Non-canonical. Ask your Storyteller before using.
    Apocryphal,
    /// Bridge, alternate purchase costs
    Bridge,
    /// Affects clash attacks
    Clash,
    /// Allows or applies to a Counterattack
    Counterattack,
    /// Only applies to Decisive attacks
    DecisiveOnly,
    /// Effects are limited when the Exalt's type is dissonant with the Magic
    /// Material of its construction.
    Dual,
    /// Does not increase Anima when used
    Mute,
    /// Applies when captaining a ship or boat
    Pilot,
    /// Mind-affecting Charms
    Psyche,
    /// Cannot be used while crashed
    Perilous,
    /// Charm requires spending Crafting experience
    Salient,
    /// Can be applied multiple times
    Stackable,
    /// Affects Withering and Decisive attacks equally
    Uniform,
    /// Applies to Withering attacks only
    WitheringOnly,
    /// Only usable in written form
    WrittenOnly,
}
