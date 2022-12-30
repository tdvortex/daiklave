use crate::{
    willpower::Willpower,
    CharacterMutation, CharacterMutationError, exalt_state::ExaltStateView, health::Health, abilities::AbilitiesView, attributes::Attributes,
};

/// A borrowed instance of a Character which references a CharacterEventSource
/// object, using &str instead of String.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterView<'source> {
    pub(crate) name: &'source str,
    pub(crate) concept: Option<&'source str>,
    pub(crate) exalt_state: ExaltStateView<'source>,
    pub(crate) willpower: Willpower,
    pub(crate) health: Health,
    pub(crate) attributes: Attributes,
    pub(crate) abilities: AbilitiesView<'source>,
}

impl<'source> Default for CharacterView<'source> {
    fn default() -> Self {
        Self {
            name: "New Character",
            concept: Default::default(),
            exalt_state: Default::default(),
            willpower: Default::default(),
            health: Default::default(),
            attributes: Default::default(),
            abilities: Default::default(),
        }
    }
}

impl<'source> CharacterView<'source> {
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
            CharacterMutation::RemoveMartialArtsStyle(_) => todo!(),
        }
    }

    /// Applies a specific CharacterMutation or returns an error.
    pub fn apply_mutation(
        &mut self,
        mutation: &'source CharacterMutation,
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
            CharacterMutation::RemoveMartialArtsStyle(_) => todo!(),
        }
    }
}