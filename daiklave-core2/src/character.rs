use serde::{Deserialize, Serialize};

use crate::{
    abilities::{
        Abilities, Ability, AbilityNameVanilla, AddSpecialtyError, RemoveSpecialtyError,
        SetAbilityError,
    },
    attributes::{AttributeName, Attributes, SetAttributesError},
    craft::Craft,
    exaltation::{
        exalt::{
            essence::{Essence, MoteCommitmentId, MotePoolName},
            exalt_type::solar::Solar,
        },
        Exaltation,
    },
    health::{DamageLevel, Health, WoundPenalty},
    martial_arts::{AddMartialArtsStyleError, MartialArtsStyle, MartialArtsStyleId},
    name_and_concept::RemoveConceptError,
    willpower::Willpower,
    CharacterMutation, CharacterMutationError,
};

/// An owned instance of a full (player) character. This is the format used in
/// serialization and deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    pub(crate) name: String,
    pub(crate) concept: Option<String>,
    pub(crate) exalt_state: Exaltation,
    pub(crate) willpower: Willpower,
    pub(crate) health: Health,
    pub(crate) attributes: Attributes,
    pub(crate) abilities: Abilities,
    pub(crate) craft: Craft,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "New Character".to_owned(),
            concept: Default::default(),
            exalt_state: Default::default(),
            willpower: Default::default(),
            health: Default::default(),
            attributes: Default::default(),
            abilities: Default::default(),
            craft: Default::default(),
        }
    }
}

impl Character {
    /// Checks if a specific CharacterMutation can be safely applied.
    pub fn check_mutation(
        &self,
        mutation: &CharacterMutation,
    ) -> Result<(), CharacterMutationError> {
        match mutation {
            CharacterMutation::SetName(name) => self.check_set_name(name.as_str()),
            CharacterMutation::SetConcept(concept) => self.check_set_concept(concept.as_str()),
            CharacterMutation::RemoveConcept => self.check_remove_concept(),
            CharacterMutation::SetMortal => self.check_set_mortal(),
            CharacterMutation::SetSolar(solar_traits) => self.check_set_solar(solar_traits),
            CharacterMutation::SpendMotes(first, amount) => self.check_spend_motes(*first, *amount),
            CharacterMutation::CommitMotes(id, name, first, amount) => {
                self.check_commit_motes(id, name, *first, *amount)
            }
            CharacterMutation::RecoverMotes(amount) => self.check_recover_motes(*amount),
            CharacterMutation::UncommitMotes(id) => self.check_uncommit_motes(id),
            CharacterMutation::SetEssenceRating(rating) => self.check_set_essence_rating(*rating),
            CharacterMutation::SetCurrentWillpower(amount) => {
                self.check_set_current_willpower(*amount)
            }
            CharacterMutation::SetWillpowerRating(dots) => self.check_set_willpower_rating(*dots),
            CharacterMutation::SetWoundPenalties(wound_penalties) => {
                self.check_set_wound_penalties(wound_penalties)
            }
            CharacterMutation::TakeDamage(damage_level, amount) => {
                self.check_take_damage(*damage_level, *amount)
            }
            CharacterMutation::HealDamage(amount) => self.check_heal_damage(*amount),
            CharacterMutation::SetAttribute(attribute_name, dots) => {
                self.check_set_attribute(*attribute_name, *dots)
            }
            CharacterMutation::SetAbilityDots(ability_name, dots) => {
                self.check_set_ability_dots(*ability_name, *dots)
            }
            CharacterMutation::AddSpecialty(ability_name, specialty) => {
                self.check_add_specialty(*ability_name, specialty.as_str())
            }
            CharacterMutation::RemoveSpecialty(ability_name, specialty) => {
                self.check_remove_specialty(*ability_name, specialty.as_str())
            }
            CharacterMutation::AddMartialArtsStyle(id, style) => {
                self.check_add_martial_arts_style(*id, style)
            }
            CharacterMutation::RemoveMartialArtsStyle(id) => {
                self.check_remove_martial_arts_style(*id)
            }
            CharacterMutation::SetMartialArtsDots(id, dots) => {
                self.check_set_martial_arts_dots(*id, *dots)
            }
            CharacterMutation::SetCraftDots(focus, dots) => {
                self.check_set_craft_dots(focus.as_str(), *dots)
            }
        }
    }

    /// Applies a specific CharacterMutation or returns an error.
    pub fn apply_mutation(
        &mut self,
        mutation: &CharacterMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_mutation(mutation)?;
        match mutation {
            CharacterMutation::SetName(name) => self.set_name(name.as_str()),
            CharacterMutation::SetConcept(concept) => self.set_concept(concept.as_str()),
            CharacterMutation::RemoveConcept => self.remove_concept(),
            CharacterMutation::SetMortal => self.set_mortal(),
            CharacterMutation::SetSolar(solar_traits) => self.set_solar(solar_traits),
            CharacterMutation::SpendMotes(first, amount) => self.spend_motes(*first, *amount),
            CharacterMutation::CommitMotes(id, name, first, amount) => {
                self.commit_motes(id, name, *first, *amount)
            }
            CharacterMutation::RecoverMotes(amount) => self.recover_motes(*amount),
            CharacterMutation::UncommitMotes(id) => self.uncommit_motes(id),
            CharacterMutation::SetEssenceRating(rating) => self.set_essence_rating(*rating),
            CharacterMutation::SetCurrentWillpower(amount) => self.set_current_willpower(*amount),
            CharacterMutation::SetWillpowerRating(dots) => self.set_willpower_rating(*dots),
            CharacterMutation::SetWoundPenalties(wound_penalties) => {
                self.set_wound_penalties(wound_penalties)
            }
            CharacterMutation::TakeDamage(damage_level, amount) => {
                self.take_damage(*damage_level, *amount)
            }
            CharacterMutation::HealDamage(amount) => self.heal_damage(*amount),
            CharacterMutation::SetAttribute(attribute_name, dots) => {
                self.set_attribute(*attribute_name, *dots)
            }
            CharacterMutation::SetAbilityDots(ability_name, dots) => {
                self.set_ability_dots(*ability_name, *dots)
            }
            CharacterMutation::AddSpecialty(ability_name, specialty) => {
                self.add_specialty(*ability_name, specialty.as_str())
            }
            CharacterMutation::RemoveSpecialty(ability_name, specialty) => {
                self.remove_specialty(*ability_name, specialty.as_str())
            }
            CharacterMutation::AddMartialArtsStyle(id, style) => {
                self.add_martial_arts_style(*id, style)
            }
            CharacterMutation::RemoveMartialArtsStyle(id) => self.remove_martial_arts_style(*id),
            CharacterMutation::SetMartialArtsDots(id, dots) => {
                self.set_martial_arts_dots(*id, *dots)
            }
            CharacterMutation::SetCraftDots(focus, dots) => {
                self.set_craft_dots(focus.as_str(), *dots)
            }
        }
    }

    /// Returns the character's current willpower amount and permanent rating.
    pub fn willpower(&self) -> &Willpower {
        &self.willpower
    }

    /// Checks if the character's current willpower can be set to the specified
    /// amount.
    pub fn check_set_current_willpower(&self, _amount: u8) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Sets the character's willpower to the specified amount. This is allowed
    /// to exceed their ordinary rating.
    pub fn set_current_willpower(
        &mut self,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.willpower.current = amount;
        Ok(self)
    }

    /// Checks if the character's permanent willpower can be set to the
    /// specified dot level.
    pub fn check_set_willpower_rating(&self, _dots: u8) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Sets the character's permanent willpower rating to the specified dots
    /// amount. This will also reset their current willpower amount to be the
    /// same amount.
    pub fn set_willpower_rating(&mut self, dots: u8) -> Result<&mut Self, CharacterMutationError> {
        self.willpower.rating = dots;
        self.willpower.current = dots;
        Ok(self)
    }

    /// Get read-only access to a character's Abilities.
    pub fn abilities(&self) -> &Abilities {
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
        if let Ability::NonZero(_, specialties) = self.abilities().ability(ability_name) {
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
        specialty: &str,
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
        if let Ability::NonZero(_, specialties) = self.abilities().ability(ability_name) {
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

    /// None for mortals.
    pub fn essence(&self) -> Option<&Essence> {
        self.exalt_state.essence()
    }

    /// Checks if the requested amount of motes can be spent.
    pub fn check_spend_motes(
        &self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_spend_motes(first, amount)
    }

    /// Spends motes, starting with the specified pool first.
    pub fn spend_motes(
        &mut self,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.spend_motes(first, amount)?;
        Ok(self)
    }

    /// Checks if the requested mote commitment would be possible.
    pub fn check_commit_motes(
        &self,
        id: &MoteCommitmentId,
        name: &str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_commit_motes(id, name, first, amount)
    }

    /// Removes available motes, starting with the specified pool, and
    /// packages them into a commitment package to be later uncommitted.
    pub fn commit_motes(
        &mut self,
        id: &MoteCommitmentId,
        name: &str,
        first: MotePoolName,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.commit_motes(id, name, first, amount)?;
        Ok(self)
    }

    /// Checks if mote recovery is possible.
    pub fn check_recover_motes(&self, amount: u8) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_recover_motes(amount)
    }

    /// Recovers motes, moving them from spent to available. Will not uncommit
    /// motes.
    pub fn recover_motes(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.recover_motes(amount)?;
        Ok(self)
    }

    /// Checks if a committed mote effect can be uncommitted.
    pub fn check_uncommit_motes(
        &self,
        id: &MoteCommitmentId,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_uncommit_motes(id)
    }

    /// Uncommits a mote effect, returning the committed motes to their pool(s)
    /// as spent motes to be later recovered.
    pub fn uncommit_motes(
        &mut self,
        id: &MoteCommitmentId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.uncommit_motes(id)?;
        Ok(self)
    }

    /// Checks if essence can be set to the specified value.
    pub fn check_set_essence_rating(&self, rating: u8) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_essence_rating(rating)
    }

    /// Changes the essence rating of the character to the specified value.
    /// This also uncommits all active effects and recovers all motes.
    pub fn set_essence_rating(&mut self, rating: u8) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.set_essence_rating(rating)?;
        Ok(self)
    }

    /// Returns true if character is a Solar.
    pub fn is_solar(&self) -> bool {
        self.exalt_state.is_solar()
    }

    /// Returns the character's Solar-specific traits, or None if not a Solar.
    pub fn solar_traits(&self) -> Option<&Solar> {
        self.exalt_state.solar_traits()
    }

    /// Checks if character can be turned into a Solar Exalted with given
    /// traits.
    pub fn check_set_solar(&self, solar_traits: &Solar) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_solar(solar_traits)
    }

    /// Sets a character's Exaltation to be the given Solar exaltation. If the
    /// character was previously mortal, permanent willpower rating is
    /// increased by 2 (reflecting the difference between mortal default and
    /// Exalt default).
    pub fn set_solar(&mut self, solar_traits: &Solar) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_solar(solar_traits)?;
        if self.is_mortal() {
            let new_willpower_rating = self.willpower().rating() + 2;
            self.set_willpower_rating(new_willpower_rating)?;
        }
        self.exalt_state.set_solar(solar_traits)?;
        Ok(self)
    }

    /// Checks if a Martial Arts style can be added to the character.
    pub fn check_add_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<(), CharacterMutationError> {
        if self.abilities().dots(AbilityNameVanilla::Brawl) < 1 {
            return Err(CharacterMutationError::AddMartialArtsStyleError(
                AddMartialArtsStyleError::PrerequsitesNotMet(
                    "Brawl must be 1+ to take Martial Artist merit".to_owned(),
                ),
            ));
        }

        self.exalt_state.check_add_martial_arts_style(id, style)
    }

    /// Adds a Martial Arts style to the character.
    pub fn add_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
        style: &MartialArtsStyle,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_add_martial_arts_style(id, style)?;
        self.exalt_state.add_martial_arts_style(id, style)?;

        Ok(self)
    }

    /// Checks if a Martial Arts style can be removed from the character.
    pub fn check_remove_martial_arts_style(
        &self,
        id: MartialArtsStyleId,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_remove_martial_arts_style(id)
    }

    /// Removes a Martial Arts style from the character.
    pub fn remove_martial_arts_style(
        &mut self,
        id: MartialArtsStyleId,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.remove_martial_arts_style(id)?;
        Ok(self)
    }

    /// Checks if the ability dots for the specified Martial Arts style
    /// can be set to a given value.
    pub fn check_set_martial_arts_dots(
        &self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_martial_arts_dots(id, dots)
    }

    /// Sets the ability dots for a specific Martial Arts style.
    pub fn set_martial_arts_dots(
        &mut self,
        id: MartialArtsStyleId,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.exalt_state.set_martial_arts_dots(id, dots)?;
        Ok(self)
    }

    /// Returns the character's name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the character's concept (if any).
    pub fn concept(&self) -> Option<&str> {
        self.concept.as_deref()
    }

    /// Checks if the character's name can be changed.
    pub fn check_set_name(&self, _name: &str) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Checks if the character's concept can be set.
    pub fn check_set_concept(&self, _concept: &str) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Checks if the character's concept can be removed.
    pub fn check_remove_concept(&self) -> Result<(), CharacterMutationError> {
        if self.concept().is_none() {
            Err(CharacterMutationError::RemoveConceptError(
                RemoveConceptError::NoConcept,
            ))
        } else {
            Ok(())
        }
    }

    /// Sets the character's name.
    pub fn set_name(&mut self, name: &str) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_name(name)?;
        self.name = name.to_owned();
        Ok(self)
    }

    /// Sets the character to the given concept.
    pub fn set_concept(&mut self, concept: &str) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_concept(concept)?;
        self.concept = Some(concept.to_owned());
        Ok(self)
    }

    /// Removes the character's concept.
    pub fn remove_concept(&mut self) -> Result<&mut Self, CharacterMutationError> {
        self.check_remove_concept()?;
        self.concept = None;
        Ok(self)
    }

    /// Returns true if character is not Exalted.
    pub fn is_mortal(&self) -> bool {
        self.exalt_state.is_mortal()
    }

    /// Returns true if character is an Exalt.
    pub fn is_exalted(&self) -> bool {
        self.exalt_state.is_exalted()
    }

    /// Checks if character can be de-Exalted and set to be mortal.
    pub fn check_set_mortal(&self) -> Result<(), CharacterMutationError> {
        self.exalt_state.check_set_mortal()
    }

    /// De-Exalts character, setting them to be mortal. This also reduces their
    /// permanent willpower rating by 2 (reflecting the difference between
    /// mortal default and Exalt default).
    pub fn set_mortal(&mut self) -> Result<&mut Self, CharacterMutationError> {
        if self.is_mortal() {
            return Ok(self);
        }
        self.exalt_state.set_mortal()?;
        let new_willpower_rating = self.willpower().rating().max(2) - 2;
        self.set_willpower_rating(new_willpower_rating)?;
        Ok(self)
    }

    /// Gets the character's health state (read-only).
    pub fn health(&self) -> &Health {
        &self.health
    }

    /// Checks if wound penalties can be set to a specific level.
    pub fn check_set_wound_penalties(
        &self,
        _new_wound_penalties: &[WoundPenalty],
    ) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Sets a character's health track to be the specified set of wound
    /// penalies. Additionally heals all damage.
    pub fn set_wound_penalties(
        &mut self,
        new_wound_penalties: &[WoundPenalty],
    ) -> Result<&mut Self, CharacterMutationError> {
        self.health.set_wound_penalties(new_wound_penalties)?;
        Ok(self)
    }

    /// Checks if character can be assigned an amount and type of damage.
    pub fn check_take_damage(
        &self,
        _damage_level: DamageLevel,
        _amount: u8,
    ) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Adds damage to character (including overflow rollovers). Caps out at
    /// being full up with aggravated.
    pub fn take_damage(
        &mut self,
        damage_level: DamageLevel,
        amount: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.health.take_damage(damage_level, amount)?;
        Ok(self)
    }

    /// Checks if the character can heal the specified amount of damage.
    pub fn check_heal_damage(&self, _amount: u8) -> Result<(), CharacterMutationError> {
        Ok(())
    }

    /// Heals a character for the specified amount of damage (capped at the
    /// amount of damage they actually have). Bashing heals before lethal which
    /// heals before aggravated.
    pub fn heal_damage(&mut self, amount: u8) -> Result<&mut Self, CharacterMutationError> {
        self.health.heal_damage(amount)?;
        Ok(self)
    }

    /// Gets a struct reference for the character's attributes.
    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    /// Validates that the requested dot level is an appropriate attribute
    /// rating. Attributes must be between 1 and 5 for all player characters.
    pub fn check_set_attribute(
        &self,
        _attribute_name: AttributeName,
        dots: u8,
    ) -> Result<(), CharacterMutationError> {
        if !(1..=5).contains(&dots) {
            Err(CharacterMutationError::SetAttributesError(
                SetAttributesError::InvalidRating(dots),
            ))
        } else {
            Ok(())
        }
    }

    /// Sets the specified attribute name to the specified dot rating.
    pub fn set_attribute(
        &mut self,
        attribute_name: AttributeName,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        self.check_set_attribute(attribute_name, dots)?;
        self.attributes.set_dots(attribute_name, dots)?;
        Ok(self)
    }

    /// The character's Craft abilities and specialties.
    pub fn craft(&self) -> &Craft {
        &self.craft
    }

    /// Checks if a Craft ability can be set to the specified dots.
    pub fn check_set_craft_dots(
        &self,
        _focus: &str,
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

    /// Sets a specific Craft focus area to the specified dots.
    pub fn set_craft_dots(
        &mut self,
        focus: &str,
        dots: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        if dots > 5 {
            Err(CharacterMutationError::SetAbilityError(
                SetAbilityError::InvalidRating(dots),
            ))
        } else {
            self.craft.set_dots(focus, dots)?;
            Ok(self)
        }
    }
}
