use crate::{
    abilities::AbilityName,
    martial_arts::MartialArtsStyle,
    sorcery::{
        spell::SpellId, ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId,
        TerrestrialSpell,
    },
    CharacterMutation,
};

use super::exaltation_choice::ExaltationChoice;

/// The operations you can do during a guided character building process.
#[derive(Debug)]
pub enum GuidedMutation {
    /// Apply a standard character mutation (with additional validation).
    CharacterMutation(CharacterMutation),
    /// Move on to the next stage of the builder. Note that because different
    /// Exalt types have different stages, some stages may be skipped or done
    /// in a different order.
    AdvanceStage,
    /// Choose a specific Exalt type (or Mortal), without necessarily setting
    /// all exaltations up-front.
    SetExaltation(ExaltationChoice),
    /// Add a Solar Caste ability to the guided builder.
    AddSolarCasteAbility(AbilityName),
    /// Removes a Solar Caste ability from the guided builder.
    RemoveSolarCasteAbility(AbilityName),
    /// Sets the Solar's Supernal ability.
    SetSolarSupernalAbility(AbilityName),
    /// Add a Solar Favored ability to the guided builder.
    AddSolarFavoredAbility(AbilityName),
    /// Remove a Solar Favored ability from the guided builder.
    RemoveSolarFavoredAbility(AbilityName),
    /// Add a Martial Arts style.
    AddMartialArtsStyle(String, MartialArtsStyle),
    /// Removes a Martial Arts style.
    RemoveMartialArtsStyle(String),
    /// Sets the sorcery archetype for the character. Clears any
    /// previous sorcery archetype and/or shaping ritual.
    SetSorceryArchetype(SorceryArchetypeId, SorceryArchetype),
    /// Sets the shaping ritual for the character.
    SetShapingRitual(ShapingRitualId, ShapingRitual),
    /// Sets the control spell for the character.
    SetControlSpell(SpellId, TerrestrialSpell),
}
