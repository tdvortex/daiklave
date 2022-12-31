use thiserror::Error;

use crate::CharacterMutationError;

/// An error trying to set or remove a Solar caste, supernal, or favored
/// ability
#[derive(Debug, Error)]
pub enum SolarAbilityError {
    /// Solar caste and favored abilities must be unique.
    #[error("Cannot have duplicate Caste or Favored abilities")]
    UniqueCasteAndFavored,
    /// Referencing an absent ability
    #[error("Could not find ability")]
    NotFound,
    /// Supernal abilities must first be selected as Caste abilities, unless
    /// MartialArts is Supernal, in which case Brawl must be a Caste ability.
    #[error("Supernal ability must be a selected Caste ability")]
    SupernalIsCaste,
    /// Must use correct abilities for the chosen Caste
    #[error("Not a caste ability")]
    InvalidCasteAbility,
    /// Must have exactly 5 Caste abilities and 5 Favored abilities.
    #[error("Incorrect number of Caste and Favored abilities")]
    CasteAndFavoredCount,
    /// Martial Arts cannot be either a Caste or Favored ability (implied by
    /// having Brawl as Caste/Favored).
    #[error("MartialArts cannot be Caster or Favored")]
    MartialArts,
}

#[derive(Debug, Error)]
pub enum SorceryError {
    #[error("Must have the correct archetype for the shaping ritual")]
    MissingArchetype,
}

/// The possible errors occurring in the guided character builder.
#[derive(Debug, Error)]
pub enum GuidedError {
    /// An error in applying the mutation to the base character
    #[error("Could not apply mutation to base character")]
    CharacterMutationError(#[from] CharacterMutationError),
    /// An error in trying to move stages in the wrong order
    #[error("Cannot move stages out of order")]
    StageOrderError,
    /// An error trying to move because previous stage is not complete
    #[error("Cannot move to the next stage while previous is incomplete")]
    StageIncompleteError,
    /// An error in trying to spend more bonus points than are available
    #[error("Cannot spend more bonus points than are available")]
    InsufficientBonusPoints,
    /// An error trying to set or remove a Solar caste, supernal, or favored
    /// ability
    #[error("Could not add a Solar ability")]
    SolarAbilityError(#[from] SolarAbilityError),
    #[error("Could not set a Sorcery value")]
    SorceryError(#[from] SorceryError),
}
