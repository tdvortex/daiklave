use serde::{Serialize, Deserialize};

/// Charm keywords that may apply for an Evocation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EvocationKeyword {
    /// Aggravated damage-dealing
    Aggravated,
    /// Affects clash attacks
    Clash,
    /// Allows or applies to a Counterattack
    Counterattack,
    /// Only applies to Decisive attacks
    DecisiveOnly,
    /// Effects are limited when the Exalt's type is dissonant with the Magic
    /// Material of the artifact.
    Dissonant,
    /// Dual (different applications to Decisive and Withering)
    Dual,
    /// Does not increase Anima when used
    Mute,
    /// Mind-affecting Charms
    Psyche,
    /// Has additional effect when the Exalt is resonant with the Magic
    /// Material of the artifact
    Resonant,
    /// Can be applied multiple times
    Stackable,
    /// Affects Withering and Decisive attacks equally
    Uniform,
    /// Applies to Withering attacks only
    WitheringOnly,
}