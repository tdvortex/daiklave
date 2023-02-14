use thiserror::Error;

use crate::{
    abilities::AbilityError,
    armor::{armor_item::artifact::ArtifactError, ArmorError},
    attributes::AttributeError,
    charms::CharmError,
    concept::ConceptError,
    exaltation::exalt::{essence::EssenceError, exalt_type::solar::SolarError},
    experience::ExperienceError,
    hearthstones::HearthstoneError,
    intimacies::intimacy::IntimacyError,
    languages::LanguageError,
    martial_arts::MartialArtsError,
    merits::merit::MeritError,
    sorcery::SorceryError,
    weapons::WeaponError,
};

/// An error representing something that could go wrong with a
/// CharacterMutation.
#[derive(Debug, Error)]
pub enum CharacterMutationError {
    /// Error related to abilities
    #[error("Abilities error: {0:?}")]
    AbilityError(#[from] AbilityError),
    /// Error related to armor
    #[error("Armor error: {0:?}")]
    ArmorError(#[from] ArmorError),
    /// Error related to artifacts
    #[error("Artifacts error: {0:?}")]
    ArtifactError(#[from] ArtifactError),
    /// Error related to Attributes
    #[error("Attribute error: {0:?}")]
    AttributeError(#[from] AttributeError),
    /// Error related to Charms
    #[error("Charm error: {0:?}")]
    CharmError(#[from] CharmError),
    /// Error occurring while trying to modify a character's concept
    #[error("Concept error: {0:?}")]
    ConceptError(#[from] ConceptError),
    /// Error related to Essence rating or mote pools
    #[error("Essence error: {0:?}")]
    EssenceError(#[from] EssenceError),
    /// Error related to character Experience
    #[error("Experience error: {0:?}")]
    ExperienceError(#[from] ExperienceError),
    /// Error related to hearthstones
    #[error("Hearthstone error: {0:?}")]
    HearthstoneError(#[from] HearthstoneError),
    /// Error related to Intimacies
    #[error("Intimacy error: {0:?}")]
    IntimacyError(#[from] IntimacyError),
    /// Error related to languages
    #[error("Language error: {0:?}")]
    LanguageError(#[from] LanguageError),
    /// Error related to Martial Arts
    #[error("Martial Arts error: {0:?}")]
    MartialArtsError(#[from] MartialArtsError),
    /// Error related to merits
    #[error("Merit error: {0:?}")]
    MeritError(#[from] MeritError),
    /// Error specific to the Solar Exalted
    #[error("Solar error: {0:?}")]
    SolarError(#[from] SolarError),
    /// Error related to Sorcery
    #[error("Sorcery error: {0:?}")]
    SorceryError(#[from] SorceryError),
    /// Error related to weapons
    #[error("Weapons error: {0:?}")]
    WeaponError(#[from] WeaponError),
}
