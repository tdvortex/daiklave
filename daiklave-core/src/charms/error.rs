use thiserror::Error;

/// An error related to Charms
#[derive(Debug, Error)]
pub enum CharmError {
    /// Can't have the same Charm more than once. If a Charm can be purchased
    /// more than once, should be recorded as "Charm Name", "Charm Name (x2)",
    /// etc.
    #[error("Can't have the same Charm more than once")]
    DuplicateCharm,
    /// Mortals cannot have Charms (except possibly Terrestrial spells)
    #[error("Mortals cannot have non-Spell Charms")]
    Mortal,
    /// Can't remove a Charm that isn't present
    #[error("Charm not found")]
    NotFound,
    /// One or more prerequisites to this Charm have not been met
    #[error("Charm prerequisites are not met")]
    PrerequisitesNotMet,
    /// Charm cannot be purchased by this Exalt type
    #[error("Wrong Exalt type")]
    WrongExaltType,
}
