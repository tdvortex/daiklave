use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::{CharacterMutationError, Character};

/// This is used to identify all ability ratings that must exist for a
/// character. It excludes all Craft abilities and MartialArts styles.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AbilityNameVanilla {
    /// Archery
    Archery,
    /// Athletics
    Athletics,
    /// Awareness
    Awareness,
    /// Brawl
    Brawl,
    /// Bureaucracy
    Bureaucracy,
    /// Dodge
    Dodge,
    /// Integrity
    Integrity,
    /// Investigation
    Investigation,
    /// Larceny
    Larceny,
    /// Linguistics
    Linguistics,
    /// Lore
    Lore,
    /// Medicine
    Medicine,
    /// Melee
    Melee,
    /// Occult
    Occult,
    /// Performance
    Performance,
    /// Presence
    Presence,
    /// Resistance
    Resistance,
    /// Ride
    Ride,
    /// Sail
    Sail,
    /// Socialize
    Socialize,
    /// Stealth
    Stealth,
    /// Survival
    Survival,
    /// Thrown
    Thrown,
    /// War
    War,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Ability {
    Zero,
    NonZero(u8, HashSet<String>)
}

impl Default for Ability {
    fn default() -> Self {
        Self::Zero
    }
}

#[derive(Debug, Error)]
pub enum SetAbilityError {
    #[error("Ability ratings must be between 0 and 5, not {0}")]
    InvalidRating(u8),
}

#[derive(Debug, Error)]
pub enum AddSpecialtyError {
    #[error("Specialty already exists")]
    DuplicateSpecialty,
    #[error("Abilities must be rated at 1+ to have specialties")]
    ZeroAbility,
}

#[derive(Debug, Error)]
pub enum RemoveSpecialtyError {
    #[error("Specialty does not exist")]
    NotFound,
}

impl Ability {
    fn dots(&self) -> u8 {
        match self {
            Ability::Zero => 0,
            Ability::NonZero(dots, _) => *dots,
        }
    }

    fn set_dots(&mut self, new_dots: u8) -> Result<&mut Self, CharacterMutationError> {
        if new_dots > 5 {
            Err(CharacterMutationError::SetAbilityError(SetAbilityError::InvalidRating(new_dots)))
        } else {
            if new_dots == 0 {
                *self = Ability::Zero;
                Ok(self)
            } else if let Ability::NonZero(dots, _) = self {
                *dots = new_dots;
                Ok(self)
            } else { // Was zero, now is non zero
                *self = Ability::NonZero(new_dots, HashSet::new());
                Ok(self)
            }
        }
    }

    fn specialties(&self) -> impl Iterator<Item = &str> {
        match self {
            Ability::Zero => vec![],
            Ability::NonZero(_, specialties) => specialties.iter().map(|s| s.as_str()).collect()
        }.into_iter()
    }

    fn add_specialty(&mut self, new_specialty: &str) -> Result<&mut Self, CharacterMutationError> {
        if let Ability::NonZero(_, specialties) = self {
            if specialties.contains(new_specialty) {
                Err(CharacterMutationError::AddSpecialtyError(AddSpecialtyError::DuplicateSpecialty))
            } else {
                specialties.insert(new_specialty.to_owned());
                Ok(self)
            }
        } else {
            Err(CharacterMutationError::AddSpecialtyError(AddSpecialtyError::ZeroAbility))
        }
    }

    fn remove_specialty(&mut self, specialty: &str) -> Result<&mut Self, CharacterMutationError> {
        if let Ability::NonZero(_, specialties) = self {
            if !specialties.remove(specialty) {
                Err(CharacterMutationError::RemoveSpecialtyError(RemoveSpecialtyError::NotFound))
            } else {
                Ok(self)
            }
        } else {
            Err(CharacterMutationError::RemoveSpecialtyError(RemoveSpecialtyError::NotFound))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Abilities {
    archery: Ability,
    athletics: Ability,
    awareness: Ability,
    brawl: Ability,
    bureaucracy: Ability,
    dodge: Ability,
    integrity: Ability,
    investigation: Ability,
    larceny: Ability,
    linguistics: Ability,
    lore: Ability,
    medicine: Ability,
    melee: Ability,
    occult: Ability,
    performance: Ability,
    presence: Ability,
    resistance: Ability,
    ride: Ability,
    sail: Ability,
    socialize: Ability,
    stealth: Ability,
    survival: Ability,
    thrown: Ability,
    war: Ability,
}

impl Abilities {
    fn ability(&self, ability_name: AbilityNameVanilla) -> &Ability {
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

    fn ability_mut(&mut self, ability_name: AbilityNameVanilla) -> &mut Ability {
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

    pub fn dots(&self, ability_name: AbilityNameVanilla) -> u8 {
        self.ability(ability_name).dots()
    }

    pub fn specialties(&self, ability_name: AbilityNameVanilla) -> impl Iterator<Item = &str> {
        self.ability(ability_name).specialties()
    }
}

impl Character {
    pub fn abilities(&self) -> &Abilities {
        &self.abilities
    }

    pub fn check_set_ability_dots(&self, _ability_name: AbilityNameVanilla, dots: u8) -> Result<(), CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::SetAbilityError(SetAbilityError::InvalidRating(dots)))
        } else {
            Ok(())
        }
    }

    pub fn set_ability_dots(&mut self, ability_name: AbilityNameVanilla, dots: u8) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_ability_dots(ability_name, dots)?;
        self.abilities.ability_mut(ability_name).set_dots(dots)?;
        Ok(self)
    }

    pub fn check_add_specialty(&self, ability_name: AbilityNameVanilla, specialty: &str) -> Result<(), CharacterMutationError> {
        if let Ability::NonZero(_, specialties) = self.abilities().ability(ability_name) {
            if specialties.contains(specialty) {
                Err(CharacterMutationError::AddSpecialtyError(AddSpecialtyError::DuplicateSpecialty))
            } else {
                Ok(())
            }
        } else {
            Err(CharacterMutationError::AddSpecialtyError(AddSpecialtyError::ZeroAbility))
        }
    }

    pub fn add_specialty(&mut self, ability_name: AbilityNameVanilla, specialty: &str) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_specialty(ability_name, specialty)?;
        self.abilities.ability_mut(ability_name).add_specialty(specialty)?;
        Ok(self)
    }

    pub fn check_remove_specialty(&self, ability_name: AbilityNameVanilla, specialty: &str) -> Result<(), CharacterMutationError> {
        if let Ability::NonZero(_, specialties) = self.abilities().ability(ability_name) {
            if !specialties.contains(specialty) {
                Err(CharacterMutationError::RemoveSpecialtyError(RemoveSpecialtyError::NotFound))
            } else {
                Ok(())
            }
        } else {
            Err(CharacterMutationError::RemoveSpecialtyError(RemoveSpecialtyError::NotFound))
        }
    }

    pub fn remove_specialty(&mut self, ability_name: AbilityNameVanilla, specialty: &str) -> Result<&mut Self, CharacterMutationError> {
        self.check_remove_specialty(ability_name, specialty)?;
        self.abilities.ability_mut(ability_name).remove_specialty(specialty)?;
        Ok(self)
    }
}