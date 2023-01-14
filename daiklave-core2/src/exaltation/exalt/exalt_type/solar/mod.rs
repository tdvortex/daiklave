/// Traits relating to specific Solar castes.
pub mod caste;

mod builder;
mod builder_error;
mod error;
mod memo;
mod new;
mod sorcery;

pub use memo::SolarMemo;
pub(crate) use sorcery::{SolarSorcererView, SolarSorcererMemo};
pub use new::NewSolar;
pub use error::SolarError;

use crate::{
    abilities::AbilityName,
    sorcery::{
        circles::terrestrial::sorcerer::TerrestrialCircleSorcerer, ShapingRitual, ShapingRitualId,
        SorceryArchetype, SorceryArchetypeId, SorceryError, SpellId, TerrestrialSpell,
    },
    CharacterMutationError,
};

use self::{builder::SolarBuilder, caste::SolarCaste};

/// Traits which are unique to being a Solar Exalted, with &str
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solar<'source> {
    caste: SolarCaste,
    favored_abilities: [AbilityName; 5],
    sorcery: Option<SolarSorcererView<'source>>,
}

impl<'source> Solar<'source> {
    pub(crate) fn new(
        caste: SolarCaste,
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
    pub fn builder() -> SolarBuilder {
        SolarBuilder {
            limit_trigger: None,
        }
    }

    /// Converts a borrowed Solar object and clones it into an owned memo struct.
    pub fn as_memo(&self) -> SolarMemo {
        SolarMemo::new(
            self.caste.as_memo(),
            self.favored_abilities,
            self.sorcery.as_ref().map(|sorcery| sorcery.as_memo()),
        )
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
        let search_ability = if ability == AbilityName::MartialArts {
            AbilityName::Brawl
        } else {
            ability
        };

        self.favored_abilities.iter().any(|&a| a == search_ability)
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
                TerrestrialCircleSorcerer::new(
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
}

impl<'view, 'source> Solar<'source> {
    pub(crate) fn sorcery(&'view self) -> Option<&'view SolarSorcererView<'source>> {
        self.sorcery.as_ref()
    }
}
