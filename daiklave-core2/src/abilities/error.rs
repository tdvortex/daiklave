use thiserror::Error;

/// An error related to a character's Abilities.
#[derive(Debug, Error)]
pub enum AbilityError {    
    /// Specialties must be unique.
    #[error("Specialty already exists")]
    DuplicateSpecialty,
    /// Ability ratings must be between 0 and 5
    #[error("Ability ratings must be between 0 and 5")]
    InvalidRating,
    /// Cannot remove nonexistent specialty.
    #[error("Specialty does not exist")]
    SpecialtyNotFound,
    /// Specialties can only be added to abilities rated 1+
    #[error("Abilities must be rated at 1+ to have specialties")]
    ZeroAbilitySpecialty,
}