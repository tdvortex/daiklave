mod default;

mod event_source;
pub use event_source::CharacterEventSource;

mod memo;
pub use memo::CharacterMemo;

mod methods;

mod mutation;
pub use mutation::{CharacterMutation, CharacterMutationError};

use std::collections::HashMap;

use crate::{
    abilities::AbilitiesVanilla,
    attributes::Attributes,
    craft::Craft,
    exaltation::Exaltation,
    health::Health,
    hearthstones::{hearthstone::GeomancyLevel, HearthstoneId, UnslottedHearthstone},
    languages::Languages,
    merits::merit::{
        NonStackableMeritId, NonStackableMeritView, StackableMeritId, StackableMeritView,
    },
    unique_id::UniqueId,
    willpower::Willpower,
};

/// A borrowed instance of a Character which references a CharacterEventSource
/// object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Character<'source> {
    pub(crate) name: &'source str,
    pub(crate) concept: Option<&'source str>,
    pub(crate) exaltation: Exaltation<'source>,
    pub(crate) willpower: Willpower,
    pub(crate) health: Health,
    pub(crate) attributes: Attributes,
    pub(crate) abilities: AbilitiesVanilla<'source>,
    pub(crate) craft: Craft<'source>,
    pub(crate) hearthstone_inventory: HashMap<HearthstoneId, UnslottedHearthstone<'source>>,
    pub(crate) demenses_no_manse: HashMap<UniqueId, (&'source str, GeomancyLevel)>,
    pub(crate) stackable_merits: HashMap<StackableMeritId, StackableMeritView<'source>>,
    pub(crate) nonstackable_merits: HashMap<NonStackableMeritId, NonStackableMeritView<'source>>,
    pub(crate) languages: Languages<'source>,
}

impl<'view, 'source> Character<'source> {
    /// Clones the character and all contained values into an owned struct.
    pub fn as_memo(&self) -> CharacterMemo {
        CharacterMemo {
            name: self.name.to_string(),
            concept: self.concept.map(|s| s.to_string()),
            exalt_state: self.exaltation.as_memo(),
            willpower: self.willpower,
            health: self.health,
            attributes: self.attributes,
            abilities: self.abilities.as_memo(),
            craft: self.craft.as_memo(),
            hearthstone_inventory: self
                .hearthstone_inventory
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
            demenses_no_manse: self
                .demenses_no_manse
                .iter()
                .map(|(k, (s, g))| (*k, (s.to_string(), *g)))
                .collect(),
            nonstackable_merits: self
                .nonstackable_merits
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
            stackable_merits: self
                .stackable_merits
                .iter()
                .map(|(k, v)| (*k, v.as_memo()))
                .collect(),
            languages: self.languages.as_memo(),
        }
    }

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
            CharacterMutation::AddMundaneWeapon(weapon_id, weapon) => {
                self.check_add_mundane_weapon(*weapon_id, weapon)
            }
            CharacterMutation::EquipWeapon(weapon_id, hand) => {
                self.check_equip_weapon(*weapon_id, *hand)
            }
            CharacterMutation::UnequipWeapon(weapon_id, equipped) => {
                self.check_unequip_weapon(*weapon_id, *equipped)
            }
            CharacterMutation::AddArtifact(artifact) => self.check_add_artifact(artifact),
            CharacterMutation::RemoveMundaneWeapon(weapon_id) => {
                self.check_remove_mundane_weapon(*weapon_id)
            }
            CharacterMutation::RemoveArtifact(artifact_id) => {
                self.check_remove_artifact(*artifact_id)
            }
            CharacterMutation::AddMundaneArmor(armor_id, armor_item) => {
                self.check_add_mundane_armor(*armor_id, armor_item)
            }
            CharacterMutation::EquipArmor(armor_id) => self.check_equip_armor(*armor_id),
            CharacterMutation::RemoveMundaneArmor(armor_id) => {
                self.check_remove_mundane_armor(*armor_id)
            }
            CharacterMutation::UnequipArmor => self.check_unequip_armor(),
            CharacterMutation::AddManse(manse_name, demense_name, hearthstone_id, template) => {
                self.check_add_manse(manse_name, demense_name, *hearthstone_id, template)
            }
            CharacterMutation::AddHearthstone(hearthstone_id, template) => {
                self.check_add_hearthstone(*hearthstone_id, template)
            }
            CharacterMutation::SlotHearthstone(artifact_id, hearthstone_id) => {
                self.check_slot_hearthstone(*artifact_id, *hearthstone_id)
            }
            CharacterMutation::UnslotHearthstone(hearthstone_id) => {
                self.check_unslot_hearthstone(*hearthstone_id)
            }
            CharacterMutation::RemoveHearthstone(hearthstone_id) => {
                self.check_remove_hearthstone(*hearthstone_id)
            }
            CharacterMutation::AttuneArtifact(artifact_id, first) => {
                self.check_attune_artifact(*artifact_id, *first)
            }
            CharacterMutation::AddStackableMerit(stackable_merit_id, stackable_merit) => {
                self.check_add_stackable_merit(*stackable_merit_id, stackable_merit)
            }
            CharacterMutation::AddNonStackableMerit(nonstackable_merit_id, nonstackable_merit) => {
                self.check_add_nonstackable_merit(*nonstackable_merit_id, nonstackable_merit)
            }
            CharacterMutation::AddLanguage(language_mutation) => {
                self.check_add_language(language_mutation)
            }
            CharacterMutation::SetNativeLanguage(language_mutation) => {
                self.check_set_native_language(language_mutation)
            }
            CharacterMutation::RemoveLanguage(language_mutation) => {
                self.check_remove_language(language_mutation)
            }
            CharacterMutation::AddTerrestrialSorcery(add_terrestrial) => self
                .check_add_terrestrial_sorcery(
                    add_terrestrial.0,
                    &add_terrestrial.1,
                    add_terrestrial.2,
                    &add_terrestrial.3,
                    add_terrestrial.4,
                    &add_terrestrial.5,
                ),
            CharacterMutation::RemoveTerrestrialSorcery => self.check_remove_terrestrial_sorcery(),
            CharacterMutation::AddCelestialSorcery(add_celestial) => self
                .check_add_celestial_sorcery(
                    add_celestial.0,
                    add_celestial.1.as_ref(),
                    add_celestial.2,
                    &add_celestial.3,
                    add_celestial.4,
                    &add_celestial.5,
                ),
            CharacterMutation::RemoveCelestialSorcery => self.check_remove_celestial_sorcery(),
            CharacterMutation::AddSolarSorcery(_) => todo!(),
            CharacterMutation::RemoveSolarSorcery => todo!(),
            CharacterMutation::AddSorceryArchetypeMerit(_, _, _) => todo!(),
            CharacterMutation::RemoveSorceryArchetypeMerit(_) => todo!(),
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
            CharacterMutation::RemoveMartialArtsStyle(id) => self.remove_martial_arts_style(*id),
            CharacterMutation::SetMartialArtsDots(id, dots) => {
                self.set_martial_arts_dots(*id, *dots)
            }
            CharacterMutation::SetCraftDots(focus, dots) => {
                self.set_craft_dots(focus.as_str(), *dots)
            }
            CharacterMutation::AddMundaneWeapon(weapon_id, mundane_weapon) => {
                self.add_mundane_weapon(*weapon_id, mundane_weapon)
            }
            CharacterMutation::EquipWeapon(weapon_id, equip_hand) => {
                self.equip_weapon(*weapon_id, *equip_hand)
            }
            CharacterMutation::UnequipWeapon(weapon_id, equipped) => {
                self.unequip_weapon(*weapon_id, *equipped)
            }
            CharacterMutation::AddArtifact(artifact) => self.add_artifact(artifact),
            CharacterMutation::RemoveMundaneWeapon(weapon_id) => {
                self.remove_mundane_weapon(*weapon_id)
            }
            CharacterMutation::RemoveArtifact(artifact_id) => self.remove_artifact(*artifact_id),
            CharacterMutation::AddMundaneArmor(armor_id, armor_item) => {
                self.add_mundane_armor(*armor_id, armor_item)
            }
            CharacterMutation::EquipArmor(armor_id) => self.equip_armor(*armor_id),
            CharacterMutation::RemoveMundaneArmor(armor_id) => self.remove_mundane_armor(*armor_id),
            CharacterMutation::UnequipArmor => self.unequip_armor(),
            CharacterMutation::AddManse(manse_name, demense_name, hearthstone_id, template) => {
                self.add_manse(manse_name, demense_name, *hearthstone_id, template)
            }
            CharacterMutation::AddHearthstone(hearthstone_id, template) => {
                self.add_hearthstone(*hearthstone_id, template)
            }
            CharacterMutation::SlotHearthstone(artifact_id, hearthstone_id) => {
                self.slot_hearthstone(*artifact_id, *hearthstone_id)
            }
            CharacterMutation::UnslotHearthstone(hearthstone_id) => {
                self.unslot_hearthstone(*hearthstone_id)
            }
            CharacterMutation::RemoveHearthstone(hearthstone_id) => {
                self.remove_hearthstone(*hearthstone_id)
            }
            CharacterMutation::AttuneArtifact(artifact_id, first) => {
                self.attune_artifact(*artifact_id, *first)
            }
            CharacterMutation::AddStackableMerit(stackable_merit_id, stackable_merit) => {
                self.add_stackable_merit(*stackable_merit_id, stackable_merit)
            }
            CharacterMutation::AddNonStackableMerit(nonstackable_merit_id, nonstackable_merit) => {
                self.add_nonstackable_merit(*nonstackable_merit_id, nonstackable_merit)
            }
            CharacterMutation::AddLanguage(language_mutation) => {
                self.add_language(language_mutation)
            }
            CharacterMutation::SetNativeLanguage(language_mutation) => {
                self.set_native_language(language_mutation)
            }
            CharacterMutation::RemoveLanguage(language_mutation) => {
                self.remove_language(language_mutation)
            }
            CharacterMutation::AddTerrestrialSorcery(add_terrestrial) => self
                .add_terrestrial_sorcery(
                    add_terrestrial.0,
                    &add_terrestrial.1,
                    add_terrestrial.2,
                    &add_terrestrial.3,
                    add_terrestrial.4,
                    &add_terrestrial.5,
                ),
            CharacterMutation::RemoveTerrestrialSorcery => self.remove_terrestrial_sorcery(),
            CharacterMutation::AddCelestialSorcery(add_celestial) => self.add_celestial_sorcery(
                add_celestial.0,
                add_celestial.1.as_ref(),
                add_celestial.2,
                &add_celestial.3,
                add_celestial.4,
                &add_celestial.5,
            ),
            CharacterMutation::RemoveCelestialSorcery => self.remove_celestial_sorcery(),
            CharacterMutation::AddSolarSorcery(_) => todo!(),
            CharacterMutation::RemoveSolarSorcery => todo!(),
            CharacterMutation::AddSorceryArchetypeMerit(_, _, _) => todo!(),
            CharacterMutation::RemoveSorceryArchetypeMerit(_) => todo!(),
        }
    }
}