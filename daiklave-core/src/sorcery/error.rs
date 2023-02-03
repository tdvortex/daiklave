use thiserror::Error;

/// Errors related to adding Sorcery to a character.
#[derive(Debug, Error)]
pub enum SorceryError {
    /// Characters must progress through the circles in order, including in
    /// reverse if the player changes their mind.
    #[error("Sorcery must progress as None <-> Terrestrial <-> Celestial <-> Solar only")]
    CircleSequence,
    /// Characters with multiple Circles need a different Shaping Ritual for
    /// each.
    #[error("Each Circle needs its own Shaping Ritual")]
    DuplicateShapingRitual,
    /// Shaping rituals require specific sorcerous archetypes to use.
    #[error("Missing an archetype for a shaping ritual")]
    MissingArchetype,
    /// Only Solars can learn Solar sorcery, and only Celestial Exalts can
    /// learn Celestial sorcery.
    #[error("Wrong Exalt type for this sorcery level")]
    WrongExaltType,
    /// Terrestrial Circle requires Occult 3 (or Intelligence 3 for Lunars).
    /// Celestial Circle requires Essence 3 and Occult 4 (or Intelligence 4 for
    /// Lunars.) Solar Circle requires Essence 5 and Occult 5.
    #[error("Insufficient Occult and/or Essence")]
    PrerequisitesNotMet,
    /// Cannot remove a Control Spell.
    #[error("Cannot remove a Control Spell.")]
    RemoveControlSpell,
}
