use crate::{
    abilities::AbilityName,
    sorcery::{
        ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId,
        SorceryError, SpellId, TerrestrialSpell, circles::terrestrial::sorcerer_view::TerrestrialCircleSorcererView,
    },
    CharacterMutationError,
};

use super::{caste::SolarCasteView, sorcery::SolarSorcererView, builder::SolarBuilder, SolarMemo};

/// Traits which are unique to being a Solar Exalted, with &str
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolarView<'source> {
    caste: SolarCasteView,
    favored_abilities: [AbilityName; 5],
    sorcery: Option<SolarSorcererView<'source>>,
}

impl<'source> SolarView<'source> {
    pub(crate) fn new(
        caste: SolarCasteView,
        favored_abilities: [AbilityName; 5],
        sorcery: Option<SolarSorcererView<'source>>,
    ) -> Self {
        Self {
            caste,
            favored_abilities,
            sorcery,
        }
    }

    /// Starts building a set of Solar traits
    pub fn builder() -> SolarBuilder<'source> {
        SolarBuilder::default()
    }

    /// Returns True if the ability is a caste ability for the charcter. Note
    /// that MartialArts is a caste ability if and only if Brawl is a caste
    /// ability.
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        self.caste.has_caste_ability(ability)
    }

    /// Returns the Solar's supernal ability.
    pub fn supernal_ability(&self) -> AbilityName {
        self.caste.supernal_ability()
    }

    /// Returns True if the ability is a favored ability for the charcter. Note
    /// that MartialArts is a favored ability if and only if Brawl is a favored
    /// ability.
    pub fn has_favored_ability(&self, ability: AbilityName) -> bool {
        self.favored_abilities.iter().any(|&a| a == ability)
    }

    pub(crate) fn add_terrestrial_sorcery(
        &mut self,
        archetype_id: SorceryArchetypeId,
        archetype: &'source SorceryArchetype,
        shaping_ritual_id: ShapingRitualId,
        shaping_ritual: &'source ShapingRitual,
        control_spell_id: SpellId,
        control_spell: &'source TerrestrialSpell,
    ) -> Result<&mut Self, CharacterMutationError> {
        if self.sorcery.is_none() {
            self.sorcery = Some(SolarSorcererView::Terrestrial(
                TerrestrialCircleSorcererView::new(
                    archetype_id,
                    archetype,
                    shaping_ritual_id,
                    shaping_ritual,
                    control_spell_id,
                    control_spell,
                )?,
            ));
            Ok(self)
        } else {
            Err(CharacterMutationError::AddSorceryCircleError(
                SorceryError::CircleSequence,
            ))
        }
    }

    pub fn as_memo(&self) -> SolarMemo {
        todo!()
    }
}

impl<'view, 'source> SolarView<'source> {
    pub(crate) fn sorcery(&'view self) -> Option<&'view SolarSorcererView<'source>> {
        self.sorcery.as_ref()
    }
}
