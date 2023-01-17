/// Traits relating to specific Solar castes.
pub mod caste;

/// A builder path for constructing a new Solar.
pub mod builder;
mod error;
mod memo;
mod new;
mod sorcery;

use std::collections::hash_map::Entry;

pub use error::SolarError;
pub(crate) use memo::SolarMemo;
pub use new::NewSolar;
pub(crate) use sorcery::{SolarSorcererMemo, SolarSorcererView};

use crate::{
    abilities::AbilityName,
    exaltation::exalt::Limit,
    merits::merit::MeritError,
    sorcery::{
        circles::terrestrial::sorcerer::TerrestrialCircleSorcerer, CelestialSpell, ShapingRitual,
        ShapingRitualId, SolarSpell, SorceryArchetype, SorceryArchetypeId, SorceryArchetypeMerit,
        SorceryArchetypeMeritId, SorceryError, SpellId, TerrestrialSpell,
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

    pub(crate) fn add_solar_sorcery(
        &mut self,
        archetype_id: SorceryArchetypeId,
        archetype: Option<&'source SorceryArchetype>,
        shaping_ritual_id: ShapingRitualId,
        shaping_ritual: &'source ShapingRitual,
        control_spell_id: SpellId,
        control_spell: &'source SolarSpell,
    ) -> Result<&mut Self, CharacterMutationError> {
        let solar = match &self.sorcery {
            Some(SolarSorcererView::Celestial(celestial)) => celestial.upgrade(
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
        self.sorcery = Some(SolarSorcererView::Solar(solar));
        Ok(self)
    }

    pub(crate) fn check_add_solar_sorcery(
        &self,
        archetype_id: SorceryArchetypeId,
        archetype: Option<&'source SorceryArchetype>,
        _shaping_ritual_id: ShapingRitualId,
        shaping_ritual: &'source ShapingRitual,
        _control_spell_id: SpellId,
        _control_spell: &'source SolarSpell,
    ) -> Result<(), CharacterMutationError> {
        if let Some(SolarSorcererView::Celestial(celestial)) = &self.sorcery {
            if shaping_ritual.archetype_id() != archetype_id {
                Err(CharacterMutationError::SorceryError(
                    SorceryError::MissingArchetype,
                ))
            } else if archetype.is_none() && !celestial.archetypes.contains_key(&archetype_id) {
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

    pub(crate) fn remove_solar_sorcery(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if let Some(SolarSorcererView::Solar(solar)) = &mut self.sorcery {
            self.sorcery = Some(SolarSorcererView::Celestial((&*solar).into()));
            Ok(self)
        } else {
            Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            ))
        }
    }

    pub(crate) fn check_remove_solar_sorcery(&self) -> Result<(), CharacterMutationError> {
        if !matches!(self.sorcery, Some(SolarSorcererView::Solar(_))) {
            Err(CharacterMutationError::SorceryError(
                SorceryError::CircleSequence,
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn add_sorcery_archetype_merit(
        &mut self,
        sorcery_archetype_id: SorceryArchetypeId,
        sorcery_archetype_merit_id: SorceryArchetypeMeritId,
        sorcery_archetype_merit: &'source SorceryArchetypeMerit,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.sorcery {
            Some(SolarSorcererView::Terrestrial(terrestrial)) => {
                if terrestrial.archetype_id != sorcery_archetype_id {
                    Err(CharacterMutationError::SorceryError(
                        SorceryError::MissingArchetype,
                    ))
                } else if let Entry::Vacant(e) = terrestrial
                    .archetype_merits
                    .entry(sorcery_archetype_merit_id)
                {
                    e.insert(sorcery_archetype_merit);
                    Ok(self)
                } else {
                    Err(CharacterMutationError::MeritError(
                        MeritError::DuplicateMerit,
                    ))
                }
            }
            Some(SolarSorcererView::Celestial(celestial)) => {
                if let Some((_, merits)) = celestial.archetypes.get_mut(&sorcery_archetype_id) {
                    if let Entry::Vacant(e) = merits.entry(sorcery_archetype_merit_id) {
                        e.insert(sorcery_archetype_merit);
                        Ok(self)
                    } else {
                        Err(CharacterMutationError::MeritError(
                            MeritError::DuplicateMerit,
                        ))
                    }
                } else {
                    Err(CharacterMutationError::SorceryError(
                        SorceryError::MissingArchetype,
                    ))
                }
            }
            Some(SolarSorcererView::Solar(solar)) => {
                if let Some((_, merits)) = solar.archetypes.get_mut(&sorcery_archetype_id) {
                    if let Entry::Vacant(e) = merits.entry(sorcery_archetype_merit_id) {
                        e.insert(sorcery_archetype_merit);
                        Ok(self)
                    } else {
                        Err(CharacterMutationError::MeritError(
                            MeritError::DuplicateMerit,
                        ))
                    }
                } else {
                    Err(CharacterMutationError::SorceryError(
                        SorceryError::MissingArchetype,
                    ))
                }
            }
            None => Err(CharacterMutationError::SorceryError(
                SorceryError::MissingArchetype,
            )),
        }
    }

    pub(crate) fn check_add_sorcery_archetype_merit(
        &self,
        sorcery_archetype_id: SorceryArchetypeId,
        sorcery_archetype_merit_id: SorceryArchetypeMeritId,
        _sorcery_archetype_merit: &'source SorceryArchetypeMerit,
    ) -> Result<(), CharacterMutationError> {
        match &self.sorcery {
            Some(SolarSorcererView::Terrestrial(terrestrial)) => {
                if terrestrial.archetype_id != sorcery_archetype_id {
                    Err(CharacterMutationError::SorceryError(
                        SorceryError::MissingArchetype,
                    ))
                } else if terrestrial
                    .archetype_merits
                    .contains_key(&sorcery_archetype_merit_id)
                {
                    Err(CharacterMutationError::MeritError(
                        MeritError::DuplicateMerit,
                    ))
                } else {
                    Ok(())
                }
            }
            Some(SolarSorcererView::Celestial(celestial)) => {
                if let Some((_, merits)) = celestial.archetypes.get(&sorcery_archetype_id) {
                    if merits.contains_key(&sorcery_archetype_merit_id) {
                        Err(CharacterMutationError::MeritError(
                            MeritError::DuplicateMerit,
                        ))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(CharacterMutationError::SorceryError(
                        SorceryError::MissingArchetype,
                    ))
                }
            }
            Some(SolarSorcererView::Solar(solar)) => {
                if let Some((_, merits)) = solar.archetypes.get(&sorcery_archetype_id) {
                    if merits.contains_key(&sorcery_archetype_merit_id) {
                        Err(CharacterMutationError::MeritError(
                            MeritError::DuplicateMerit,
                        ))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(CharacterMutationError::SorceryError(
                        SorceryError::MissingArchetype,
                    ))
                }
            }
            None => Err(CharacterMutationError::SorceryError(
                SorceryError::MissingArchetype,
            )),
        }
    }

    pub(crate) fn remove_sorcery_archetype_merit(
        &mut self,
        sorcery_archetype_merit_id: SorceryArchetypeMeritId,
    ) -> Result<&mut Self, CharacterMutationError> {
        match &mut self.sorcery {
            Some(SolarSorcererView::Terrestrial(terrestrial)) => {
                if terrestrial
                    .archetype_merits
                    .remove(&sorcery_archetype_merit_id)
                    .is_none()
                {
                    Err(CharacterMutationError::MeritError(MeritError::NotFound))
                } else {
                    Ok(self)
                }
            }
            Some(SolarSorcererView::Celestial(celestial)) => {
                if !celestial
                    .archetypes
                    .iter_mut()
                    .any(|(_, (_, merits))| merits.remove(&sorcery_archetype_merit_id).is_some())
                {
                    Err(CharacterMutationError::MeritError(MeritError::NotFound))
                } else {
                    Ok(self)
                }
            }
            Some(SolarSorcererView::Solar(solar)) => {
                if !solar
                    .archetypes
                    .iter_mut()
                    .any(|(_, (_, merits))| merits.remove(&sorcery_archetype_merit_id).is_some())
                {
                    Err(CharacterMutationError::MeritError(MeritError::NotFound))
                } else {
                    Ok(self)
                }
            }
            None => Err(CharacterMutationError::MeritError(MeritError::NotFound)),
        }
    }

    pub(crate) fn check_remove_sorcery_archetype_merit(
        &self,
        sorcery_archetype_merit_id: SorceryArchetypeMeritId,
    ) -> Result<(), CharacterMutationError> {
        match &self.sorcery {
            Some(SolarSorcererView::Terrestrial(terrestrial)) => {
                if !terrestrial
                    .archetype_merits
                    .contains_key(&sorcery_archetype_merit_id)
                {
                    Err(CharacterMutationError::MeritError(MeritError::NotFound))
                } else {
                    Ok(())
                }
            }
            Some(SolarSorcererView::Celestial(celestial)) => {
                if !celestial
                    .archetypes
                    .iter()
                    .any(|(_, (_, merits))| merits.contains_key(&sorcery_archetype_merit_id))
                {
                    Err(CharacterMutationError::MeritError(MeritError::NotFound))
                } else {
                    Ok(())
                }
            }
            Some(SolarSorcererView::Solar(solar)) => {
                if !solar
                    .archetypes
                    .iter()
                    .any(|(_, (_, merits))| merits.contains_key(&sorcery_archetype_merit_id))
                {
                    Err(CharacterMutationError::MeritError(MeritError::NotFound))
                } else {
                    Ok(())
                }
            }
            None => Err(CharacterMutationError::MeritError(MeritError::NotFound)),
        }
    }
}

impl<'view, 'source> Solar<'source> {
    pub(crate) fn sorcery(&'view self) -> Option<&'view SolarSorcererView<'source>> {
        self.sorcery.as_ref()
    }
}
