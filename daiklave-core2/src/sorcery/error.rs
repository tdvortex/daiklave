use thiserror::Error;

/// Errors related to adding Sorcery to a character.
#[derive(Debug, Error)]
pub enum SorceryError {
    /// Shaping rituals require specific sorcerous archetypes to use.
    #[error("Missing an archetype for a shaping ritual")]
    MissingArchetype,
    /// Characters must progress through the circles in order, including in
    /// reverse if the player changes their mind.
    #[error("Sorcery must progress as None <-> Terrestrial <-> Celestial <-> Solar only")]
    CircleSequence,
}