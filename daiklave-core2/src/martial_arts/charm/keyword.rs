use serde::{Deserialize, Serialize};

/// All the keywords that may be associated with a specific Charm or Spell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub enum MartialArtsCharmKeyword {
    /// Aggravated damage-dealing
    Aggravated,
    /// Affects clash attacks
    Clash,
    /// Allows or applies to a Counterattack
    Counterattack,
    /// Only applies to Decisive attacks
    DecisiveOnly,
    /// Dual (different applications to Decisive and Withering)
    Dual,
    /// The key Form ability for a Martial Arts style.
    Form,
    /// Does not increase Anima when used
    Mute,
    /// Mind-affecting Charms
    Psyche,
    /// Cannot be used while crashed
    Perilous,
    /// Can be applied multiple times
    Stackable,
    /// Affects Withering and Decisive attacks equally
    Uniform,
    /// Applies to Withering attacks only
    WitheringOnly,
}
