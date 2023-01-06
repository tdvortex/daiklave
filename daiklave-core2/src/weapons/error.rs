use thiserror::Error;

#[derive(Debug, Error)]
pub enum WeaponError {
    #[error("Natural weapons must be unique")]
    DuplicateNatural,
}