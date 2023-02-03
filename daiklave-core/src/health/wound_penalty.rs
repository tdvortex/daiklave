/// The possible wound penalty levels for a health box or character
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum WoundPenalty {
    /// -0 wound penalty
    Zero,
    /// -1 wound penalty
    MinusOne,
    /// -2 wound penalty
    MinusTwo,
    /// -4 wound penalty
    MinusFour,
    /// Incapacitated-level wound penalty
    Incapacitated,
}
