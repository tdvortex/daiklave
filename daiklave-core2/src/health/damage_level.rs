/// The three levels of damage severity
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum DamageLevel {
    /// Bashing damage \[/\]
    Bashing,
    /// Lethal damage \[X\]
    Lethal,
    /// Aggravated damage \[∗\]
    Aggravated,
}
