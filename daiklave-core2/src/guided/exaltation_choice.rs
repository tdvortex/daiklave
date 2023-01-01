/// The supported options for Exaltations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExaltationChoice {
    /// No exaltation, just a heroic mortal.
    Mortal,
    /// Dawn caste Solar.
    Dawn,
    /// Zenith caste Solar.
    Zenith,
    /// Twilight caste Solar.
    Twilight,
    /// Night caste Solar.
    Night,
    /// Eclipse caste Solar.
    Eclipse,
}
