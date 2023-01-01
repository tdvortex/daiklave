use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolarBuilderError {
    #[error("Caste and Favored abilities must be unique")]
    UniqueCasteAndFavored,
    #[error("Required field missing: {0}")]
    MissingField(&'static str),
    #[error("Must have 5 Caste and 5 Favored abilities")]
    CasteAndFavoredCount,
    #[error("Martial Arts cannot be a Caste or Favored ability")]
    MartialArts,
    #[error("Must use correct caste abilities")]
    InvalidCasteAbility,
}
