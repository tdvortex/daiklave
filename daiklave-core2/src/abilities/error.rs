use thiserror::Error;

#[derive(Debug, Error)]
pub enum SetAbilityError {
    #[error("Ability ratings must be between 0 and 5, not {0}")]
    InvalidRating(u8),
}

#[derive(Debug, Error)]
pub enum AddSpecialtyError {
    #[error("Specialty already exists")]
    DuplicateSpecialty,
    #[error("Abilities must be rated at 1+ to have specialties")]
    ZeroAbility,
}

#[derive(Debug, Error)]
pub enum RemoveSpecialtyError {
    #[error("Specialty does not exist")]
    NotFound,
}
