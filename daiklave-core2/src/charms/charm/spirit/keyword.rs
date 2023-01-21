use serde::{Serialize, Deserialize};

/// A keyword for a Spirit charm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub enum SpiritCharmKeyword {
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
    /// Only usable in written form
    WrittenOnly,
}