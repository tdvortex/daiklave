/// Traits relating to specific Solar castes.
pub mod caste;

/// A builder path for constructing a new Solar.
pub mod builder;
mod error;
mod memo;
mod new;
mod sorcery;

pub use error::SolarError;
pub(crate) use memo::SolarMemo;
pub use new::NewSolar;
pub(crate) use sorcery::{SolarSorcererMemo, SolarSorcererView};

use crate::{
    abilities::AbilityName,
    exaltation::exalt::Limit,
    sorcery::{
        circles::terrestrial::sorcerer::TerrestrialCircleSorcerer, CelestialSpell, ShapingRitual,
        ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SorceryError, SpellId,
        TerrestrialSpell,
    },
    CharacterMutationError,
};

use self::{builder::SolarBuilder, caste::SolarCaste};

/// Traits which are unique to being a Solar Exalted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solar<'source> {
    caste: SolarCaste,
    favored_abilities: [AbilityName; 5],
    sorcery: Option<SolarSorcererView<'source>>,
    limit: Limit<'source>,
}

impl<'source> Solar<'source> {
    /// Starts building a set of Solar traits
    pub fn builder() -> SolarBuilder {
        SolarBuilder {
            limit_trigger: None,
        }
    }

    pub(crate) fn as_memo(&self) -> SolarMemo {
        SolarMemo {
            caste: self.caste.as_memo(),
            favored_abilities: self.favored_abilities,
            sorcery: self.sorcery.as_ref().map(|sorcery| sorcery.as_memo()),
            limit: self.limit.as_memo(),
        }
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
            Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            ))
        }
    }

    pub(crate) fn check_add_terrestrial_sorcery(
        &self,
        archetype_id: SorceryArchetypeId,
        _archetype: &'source SorceryArchetype,
        _shaping_ritual_id: ShapingRitualId,
        shaping_ritual: &'source ShapingRitual,
        _control_spell_id: SpellId,
        _control_spell: &'source TerrestrialSpell,
    ) -> Result<(), CharacterMutationError> {
        if self.sorcery.is_some() {
            Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            ))
        } else if shaping_ritual.archetype_id() != archetype_id {
            Err(CharacterMutationError::SorceryError(
                SorceryError::MissingArchetype,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn remove_terrestrial_sorcery(
        &mut self,
    ) -> Result<&mut Self, CharacterMutationError> {
        match self.sorcery {
            Some(SolarSorcererView::Terrestrial(_)) => {
                self.sorcery = None;
                Ok(self)
            }
            _ => Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            )),
        }
    }

    pub(crate) fn check_remove_terrestrial_sorcery(&self) -> Result<(), CharacterMutationError> {
        match self.sorcery {
            Some(SolarSorcererView::Terrestrial(_)) => Ok(()),
            _ => Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            )),
        }
    }

    pub(crate) fn add_celestial_sorcery(
        &mut self,
        archetype_id: SorceryArchetypeId,
        archetype: Option<&'source SorceryArchetype>,
        shaping_ritual_id: ShapingRitualId,
        shaping_ritual: &'source ShapingRitual,
        control_spell_id: SpellId,
        control_spell: &'source CelestialSpell,
    ) -> Result<&mut Self, CharacterMutationError> {
        let celestial = match &self.sorcery {
            Some(SolarSorcererView::Terrestrial(terrestrial)) => terrestrial.upgrade(
                archetype_id,
                archetype,
                shaping_ritual_id,
                shaping_ritual,
                control_spell_id,
                control_spell,
            ),
            _ => Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            )),
        }?;
        self.sorcery = Some(SolarSorcererView::Celestial(celestial));
        Ok(self)
    }

    pub(crate) fn check_add_celestial_sorcery(
        &self,
        archetype_id: SorceryArchetypeId,
        archetype: Option<&'source SorceryArchetype>,
        _shaping_ritual_id: ShapingRitualId,
        shaping_ritual: &'source ShapingRitual,
        _control_spell_id: SpellId,
        _control_spell: &'source CelestialSpell,
    ) -> Result<(), CharacterMutationError> {
        if let Some(SolarSorcererView::Terrestrial(terrestrial)) = &self.sorcery {
            if shaping_ritual.archetype_id() != archetype_id {
                Err(CharacterMutationError::SorceryError(
                    SorceryError::MissingArchetype,
                ))
            } else if archetype.is_none() && terrestrial.archetype_id != archetype_id {
                Err(CharacterMutationError::SorceryError(
                    SorceryError::MissingArchetype,
                ))
            } else {
                Ok(())
            }
        } else {
            Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            ))
        }
    }

    pub(crate) fn remove_celestial_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if let Some(SolarSorcererView::Celestial(celestial)) = &mut self.sorcery {
            self.sorcery = Some(SolarSorcererView::Terrestrial((&*celestial).into()));
            Ok(self)
        } else {
            Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            ))
        }
    }

    pub(crate) fn check_remove_celestial_sorcery(&self) -> Result<(), CharacterMutationError> {
        if !matches!(self.sorcery, Some(SolarSorcererView::Celestial(_))) {
            Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            ))
        } else {
            Ok(())
        }
    }
}

impl<'view, 'source> Solar<'source> {
    pub(crate) fn sorcery(&'view self) -> Option<&'view SolarSorcererView<'source>> {
        self.sorcery.as_ref()
    }
}
