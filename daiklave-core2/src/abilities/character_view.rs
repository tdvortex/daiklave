use std::collections::HashSet;

use crate::{AbilityNameVanilla, CharacterMutationError, CharacterView};

use super::error::{AddSpecialtyError, RemoveSpecialtyError, SetAbilityError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AbilityView<'source> {
    Zero,
    NonZero(u8, HashSet<&'source str>),
}

impl<'source> Default for AbilityView<'source> {
    fn default() -> Self {
        Self::Zero
    }
}

impl<'source> AbilityView<'source> {
    fn dots(&self) -> u8 {
        match self {
            AbilityView::Zero => 0,
            AbilityView::NonZero(dots, _) => *dots,
        }
    }

    fn set_dots(&mut self, new_dots: u8) -> Result<&mut Self, CharacterMutationError> {
        if new_dots > 5 {
            Err(CharacterMutationError::SetAbilityError(
                SetAbilityError::InvalidRating(new_dots),
            ))
        } else if new_dots == 0 {
            *self = AbilityView::Zero;
            Ok(self)
        } else if let AbilityView::NonZero(dots, _) = self {
            *dots = new_dots;
            Ok(self)
        } else {
            // Was zero, now is non zero
            *self = AbilityView::NonZero(new_dots, HashSet::new());
            Ok(self)
        }
    }

    fn specialties(&self) -> impl Iterator<Item = &str> {
        match self {
            AbilityView::Zero => vec![],
            AbilityView::NonZero(_, specialties) => specialties.iter().copied().collect(),
        }
        .into_iter()
    }

    fn add_specialty(
        &mut self,
        new_specialty: &'source str,
    ) -> Result<&mut Self, CharacterMutationError> {
        if let AbilityView::NonZero(_, specialties) = self {
            if specialties.contains(new_specialty) {
                Err(CharacterMutationError::AddSpecialtyError(
                    AddSpecialtyError::DuplicateSpecialty,
                ))
            } else {
                specialties.insert(new_specialty);
                Ok(self)
            }
        } else {
            Err(CharacterMutationError::AddSpecialtyError(
                AddSpecialtyError::ZeroAbility,
            ))
        }
    }

    fn remove_specialty(&mut self, specialty: &str) -> Result<&mut Self, CharacterMutationError> {
        if let AbilityView::NonZero(_, specialties) = self {
            if !specialties.remove(specialty) {
                Err(CharacterMutationError::RemoveSpecialtyError(
                    RemoveSpecialtyError::NotFound,
                ))
            } else {
                Ok(self)
            }
        } else {
            Err(CharacterMutationError::RemoveSpecialtyError(
                RemoveSpecialtyError::NotFound,
            ))
        }
    }
}

/// A struct representing all non-Craft, non-Martial Arts abilities, including
/// any specialties.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AbilitiesView<'source> {
    archery: AbilityView<'source>,
    athletics: AbilityView<'source>,
    awareness: AbilityView<'source>,
    brawl: AbilityView<'source>,
    bureaucracy: AbilityView<'source>,
    dodge: AbilityView<'source>,
    integrity: AbilityView<'source>,
    investigation: AbilityView<'source>,
    larceny: AbilityView<'source>,
    linguistics: AbilityView<'source>,
    lore: AbilityView<'source>,
    medicine: AbilityView<'source>,
    melee: AbilityView<'source>,
    occult: AbilityView<'source>,
    performance: AbilityView<'source>,
    presence: AbilityView<'source>,
    resistance: AbilityView<'source>,
    ride: AbilityView<'source>,
    sail: AbilityView<'source>,
    socialize: AbilityView<'source>,
    stealth: AbilityView<'source>,
    survival: AbilityView<'source>,
    thrown: AbilityView<'source>,
    war: AbilityView<'source>,
}

impl<'source> AbilitiesView<'source> {
    fn ability(&self, ability_name: AbilityNameVanilla) -> &AbilityView {
        match ability_name {
            AbilityNameVanilla::Archery => &self.archery,
            AbilityNameVanilla::Athletics => &self.athletics,
            AbilityNameVanilla::Awareness => &self.awareness,
            AbilityNameVanilla::Brawl => &self.brawl,
            AbilityNameVanilla::Bureaucracy => &self.bureaucracy,
            AbilityNameVanilla::Dodge => &self.dodge,
            AbilityNameVanilla::Integrity => &self.integrity,
            AbilityNameVanilla::Investigation => &self.investigation,
            AbilityNameVanilla::Larceny => &self.larceny,
            AbilityNameVanilla::Linguistics => &self.linguistics,
            AbilityNameVanilla::Lore => &self.lore,
            AbilityNameVanilla::Medicine => &self.medicine,
            AbilityNameVanilla::Melee => &self.melee,
            AbilityNameVanilla::Occult => &self.occult,
            AbilityNameVanilla::Performance => &self.performance,
            AbilityNameVanilla::Presence => &self.presence,
            AbilityNameVanilla::Resistance => &self.resistance,
            AbilityNameVanilla::Ride => &self.ride,
            AbilityNameVanilla::Sail => &self.sail,
            AbilityNameVanilla::Socialize => &self.socialize,
            AbilityNameVanilla::Stealth => &self.stealth,
            AbilityNameVanilla::Survival => &self.survival,
            AbilityNameVanilla::Thrown => &self.thrown,
            AbilityNameVanilla::War => &self.war,
        }
    }

    fn ability_mut(&mut self, ability_name: AbilityNameVanilla) -> &mut AbilityView<'source> {
        match ability_name {
            AbilityNameVanilla::Archery => &mut self.archery,
            AbilityNameVanilla::Athletics => &mut self.athletics,
            AbilityNameVanilla::Awareness => &mut self.awareness,
            AbilityNameVanilla::Brawl => &mut self.brawl,
            AbilityNameVanilla::Bureaucracy => &mut self.bureaucracy,
            AbilityNameVanilla::Dodge => &mut self.dodge,
            AbilityNameVanilla::Integrity => &mut self.integrity,
            AbilityNameVanilla::Investigation => &mut self.investigation,
            AbilityNameVanilla::Larceny => &mut self.larceny,
            AbilityNameVanilla::Linguistics => &mut self.linguistics,
            AbilityNameVanilla::Lore => &mut self.lore,
            AbilityNameVanilla::Medicine => &mut self.medicine,
            AbilityNameVanilla::Melee => &mut self.melee,
            AbilityNameVanilla::Occult => &mut self.occult,
            AbilityNameVanilla::Performance => &mut self.performance,
            AbilityNameVanilla::Presence => &mut self.presence,
            AbilityNameVanilla::Resistance => &mut self.resistance,
            AbilityNameVanilla::Ride => &mut self.ride,
            AbilityNameVanilla::Sail => &mut self.sail,
            AbilityNameVanilla::Socialize => &mut self.socialize,
            AbilityNameVanilla::Stealth => &mut self.stealth,
            AbilityNameVanilla::Survival => &mut self.survival,
            AbilityNameVanilla::Thrown => &mut self.thrown,
            AbilityNameVanilla::War => &mut self.war,
        }
    }

    /// Get the dot rating for a specific (non-Craft, non-MA) ability.
    pub fn dots(&self, ability_name: AbilityNameVanilla) -> u8 {
        self.ability(ability_name).dots()
    }

    /// Get an iterator for all specialties associated with a specific ability.
    pub fn specialties(&self, ability_name: AbilityNameVanilla) -> impl Iterator<Item = &str> {
        self.ability(ability_name).specialties()
    }
}

impl<'source> CharacterView<'source> {
    /// Get read-only access to a character's Abilities.
    pub fn abilities(&self) -> &AbilitiesView {
        &self.abilities
    }

    /// Check if an ability's dots can be set to a specific level.
    pub fn check_set_ability_dots(
        &self,
        _ability_name: AbilityNameVanilla,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::SetAbilityError(
                SetAbilityError::InvalidRating(dots),
            ))
        } else {
            Ok(())
        }
    }

    /// Set an ability's dots to a specific dot value. If this sets the ability
    /// to 0 dots, will erase all specialties.
    pub fn set_ability_dots(
        &mut self,
        ability_name: AbilityNameVanilla,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_ability_dots(ability_name, dots)?;
        self.abilities.ability_mut(ability_name).set_dots(dots)?;
        Ok(self)
    }

    /// Checks if a specialty can be added to an ability. Errors if ability is
    /// 0 dots or specialty is not unique.
    pub fn check_add_specialty(
        &self,
        ability_name: AbilityNameVanilla,
        specialty: &str,
    ) -> Result<(), CharacterMutationError> {
        if let AbilityView::NonZero(_, specialties) = self.abilities().ability(ability_name) {
            if specialties.contains(specialty) {
                Err(CharacterMutationError::AddSpecialtyError(
                    AddSpecialtyError::DuplicateSpecialty,
                ))
            } else {
                Ok(())
            }
        } else {
            Err(CharacterMutationError::AddSpecialtyError(
                AddSpecialtyError::ZeroAbility,
            ))
        }
    }

    /// Adds a specialty to an ability.
    pub fn add_specialty(
        &mut self,
        ability_name: AbilityNameVanilla,
        specialty: &'source str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_specialty(ability_name, specialty)?;
        self.abilities
            .ability_mut(ability_name)
            .add_specialty(specialty)?;
        Ok(self)
    }

    /// Checks if a specialty can be removed from an ability. Returns an error
    /// if specialty does not exist.
    pub fn check_remove_specialty(
        &self,
        ability_name: AbilityNameVanilla,
        specialty: &str,
    ) -> Result<(), CharacterMutationError> {
        if let AbilityView::NonZero(_, specialties) = self.abilities().ability(ability_name) {
            if !specialties.contains(specialty) {
                Err(CharacterMutationError::RemoveSpecialtyError(
                    RemoveSpecialtyError::NotFound,
                ))
            } else {
                Ok(())
            }
        } else {
            Err(CharacterMutationError::RemoveSpecialtyError(
                RemoveSpecialtyError::NotFound,
            ))
        }
    }

    /// Removes a specialty from an ability.
    pub fn remove_specialty(
        &mut self,
        ability_name: AbilityNameVanilla,
        specialty: &str,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_remove_specialty(ability_name, specialty)?;
        self.abilities
            .ability_mut(ability_name)
            .remove_specialty(specialty)?;
        Ok(self)
    }
}
