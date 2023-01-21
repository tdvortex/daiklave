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
    /// The Charm has additional effect when used by Sidereals.
    Enlightenment,
    /// The key Form ability for a Martial Arts style.
    Form,
    /// Charm has additional effect when used by Solars or Sidereals.
    Mastery,
    /// Does not increase Anima when used
    Mute,
    /// Mind-affecting Charms
    Psyche,
    /// Cannot be used while crashed
    Perilous,
    /// Can be applied multiple times
    Stackable,
    /// Charm has reduced effect or increased cost when used by the
    /// Dragon-Blooded.
    Terrestrial,
    /// Affects Withering and Decisive attacks equally
    Uniform,
    /// Applies to Withering attacks only
    WitheringOnly,
}
