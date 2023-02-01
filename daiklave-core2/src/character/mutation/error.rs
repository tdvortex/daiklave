use thiserror::Error;

use crate::{
    abilities::AbilityError,
    armor::{armor_item::artifact::ArtifactError, ArmorError},
    attributes::AttributeError,
    charms::CharmError,
    exaltation::exalt::essence::EssenceError,
    experience::ExperienceError,
    hearthstones::HearthstoneError,
    intimacies::intimacy::IntimacyError,
    languages::LanguageError,
    martial_arts::MartialArtsError,
    merits::merit_new::MeritError,
    sorcery::SorceryError,
    weapons::WeaponError, concept::ConceptError,
};

/// An error representing something that could go wrong with a
/// CharacterMutation.
#[derive(Debug, Error)]
pub enum CharacterMutationError {
    /// Error related to abilities
    #[error("Abilities error")]
    AbilityError(#[from] AbilityError),
    /// Error related to armor
    #[error("Armor error")]
    ArmorError(#[from] ArmorError),
    /// Error related to artifacts
    #[error("Artifacts error")]
    ArtifactError(#[from] ArtifactError),
    /// Error related to Attributes
    #[error("Attribute error")]
    AttributeError(#[from] AttributeError),
    /// Error related to Charms
    #[error("Charm error")]
    CharmError(#[from] CharmError),
    /// Error occurring while trying to modify a character's concept
    #[error("Concept error")]
    ConceptError(#[from] ConceptError),
    /// Error related to Essence rating or mote pools
    #[error("Essence error")]
    EssenceError(#[from] EssenceError),
    /// Error related to character Experience
    #[error("Experience error")]
    ExperienceError(#[from] ExperienceError),
    /// Error related to hearthstones
    #[error("Hearthstone error")]
    HearthstoneError(#[from] HearthstoneError),
    /// Error related to Intimacies
    #[error("Intimacy error")]
    IntimacyError(#[from] IntimacyError),
    /// Error related to languages
    #[error("Language error")]
    LanguageError(#[from] LanguageError),
    /// Error related to Martial Arts
    #[error("Martial Arts error")]
    MartialArtsError(#[from] MartialArtsError),
    /// Error related to merits
    #[error("Merit error")]
    MeritError(#[from] MeritError),
    /// Error related to Sorcery
    #[error("Sorcery error")]
    SorceryError(#[from] SorceryError),
    /// Error related to weapons
    #[error("Weapons error")]
    WeaponError(#[from] WeaponError),
}
