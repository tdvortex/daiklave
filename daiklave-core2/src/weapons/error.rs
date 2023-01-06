use thiserror::Error;

/// An error related to character weapons.
#[derive(Debug, Error)]
pub enum WeaponError {
    /// Characters cannot have duplicate natural weapons.
    #[error("Natural weapons must be unique")]
    DuplicateNatural,
}
