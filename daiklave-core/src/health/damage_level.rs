use serde::{Serialize, Deserialize};

/// The three levels of damage severity
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub enum DamageLevel {
    /// Bashing damage \[/\]
    Bashing,
    /// Lethal damage \[X\]
    Lethal,
    /// Aggravated damage \[∗\]
    Aggravated,
}
