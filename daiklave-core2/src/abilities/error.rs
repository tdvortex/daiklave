use thiserror::Error;

/// An error when trying to set an ability's dot rating.
#[derive(Debug, Error)]
pub enum SetAbilityError {
    /// Ability ratings must be between 0 and 5
    #[error("Ability ratings must be between 0 and 5, not {0}")]
    InvalidRating(u8),
}

/// An error when trying to add a specialty to an ability.
#[derive(Debug, Error)]
pub enum AddSpecialtyError {
    /// Specialties must be unique.
    #[error("Specialty already exists")]
    DuplicateSpecialty,
    /// Specialties can only be added to abilities rated 1+
    #[error("Abilities must be rated at 1+ to have specialties")]
    ZeroAbility,
}

/// An error when trying to remove a specialty from an ability.
#[derive(Debug, Error)]
pub enum RemoveSpecialtyError {
    /// Cannot remove nonexistent specialty.
    #[error("Specialty does not exist")]
    NotFound,
}
