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
    artifact::{ArtifactId, ArtifactName},
    attributes::Attributes,
    book_reference::BookReference,
    charms::charm::{CharmId, CharmMutation},
    craft::Craft,
    exaltation::Exaltation,
    experience::ExperiencePool,
    health::Health,
    hearthstones::{hearthstone::GeomancyLevel, HearthstoneId, UnslottedHearthstone},
    intimacies::intimacy::{IntimacyLevel, IntimacyType},
    languages::Languages,
    merits::merit::{
        NonStackableMeritId, NonStackableMeritView, StackableMeritId, StackableMeritView,
    },
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
    pub(crate) demenses_no_manse: HashMap<&'source str, GeomancyLevel>,
    pub(crate) stackable_merits: HashMap<StackableMeritId, StackableMeritView<'source>>,
    pub(crate) nonstackable_merits: HashMap<NonStackableMeritId, NonStackableMeritView<'source>>,
    pub(crate) flaws: HashMap<&'source str, (Option<BookReference>, &'source str)>,
    pub(crate) languages: Languages<'source>,
    pub(crate) intimacies: HashMap<IntimacyType<'source>, IntimacyLevel>,
    pub(crate) experience: ExperiencePool,
}

impl<'source> Character<'source> {
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
                .map(|(k, v)| ((*k).to_owned(), (*v)))
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
            flaws: self
                .flaws
                .iter()
                .map(|(name, (book_reference, description))| {
                    (
                        (*name).to_owned(),
                        (*book_reference, (*description).to_owned()),
                    )
                })
                .collect(),
            languages: self.languages.as_memo(),
            intimacies: self
                .intimacies
                .iter()
                .map(|(intimacy_type, level)| (intimacy_type.as_memo(), *level))
                .collect(),
            experience: self.experience,
        }
    }

    /// Applies a specific CharacterMutation or returns an error.
    pub fn apply_mutation(
        &mut self,
        mutation: &'source CharacterMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        match mutation {
            CharacterMutation::SetName(name) => self.set_name(name.as_str()),
            CharacterMutation::SetConcept(concept) => self.set_concept(concept.as_str()),
            CharacterMutation::RemoveConcept => self.remove_concept(),
            CharacterMutation::SetMortal => self.set_mortal(),
            CharacterMutation::SetSolar(solar_traits) => self.set_solar(solar_traits),
            CharacterMutation::SpendMotes(first, amount) => self.spend_motes(*first, *amount),
            CharacterMutation::CommitMotes(name, first, amount) => {
                self.commit_motes(name, *first, *amount)
            }
            CharacterMutation::RecoverMotes(amount) => self.recover_motes(*amount),
            CharacterMutation::UncommitMotes(name) => self.uncommit_motes(name),
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
            CharacterMutation::AddMartialArtsStyle(name, style) => {
                self.add_martial_arts_style(name.as_str(), style)
            }
            CharacterMutation::RemoveMartialArtsStyle(name) => {
                self.remove_martial_arts_style(name.as_str())
            }
            CharacterMutation::SetMartialArtsDots(name, dots) => {
                self.set_martial_arts_dots(name.as_str(), *dots)
            }
            CharacterMutation::SetCraftDots(focus, dots) => {
                self.set_craft_dots(focus.as_str(), *dots)
            }
            CharacterMutation::AddMundaneWeapon((name, mundane_weapon)) => {
                self.add_mundane_weapon(name.as_str(), mundane_weapon)
            }
            CharacterMutation::EquipWeapon(name, equip_hand) => {
                self.equip_weapon(name.as_ref(), *equip_hand)
            }
            CharacterMutation::UnequipWeapon(name, equipped) => {
                self.unequip_weapon(name.as_ref(), *equipped)
            }
            CharacterMutation::AddArtifact(artifact) => self.add_artifact(artifact),
            CharacterMutation::RemoveMundaneWeapon(name) => {
                self.remove_mundane_weapon(name.as_str())
            }
            CharacterMutation::RemoveArtifact(artifact_name) => self.remove_artifact(artifact_name),
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
            CharacterMutation::SlotHearthstone(artifact_name, hearthstone_id) => {
                self.slot_hearthstone(artifact_name, *hearthstone_id)
            }
            CharacterMutation::UnslotHearthstone(hearthstone_id) => {
                self.unslot_hearthstone(*hearthstone_id)
            }
            CharacterMutation::RemoveHearthstone(hearthstone_id) => {
                self.remove_hearthstone(*hearthstone_id)
            }
            CharacterMutation::AttuneArtifact(artifact_name, first) => {
                let artifact_id = match artifact_name {
                    ArtifactName::Weapon(weapon_name) => ArtifactId::Weapon(weapon_name.as_str()),
                    ArtifactName::Armor(armor_id) => ArtifactId::Armor(*armor_id),
                    ArtifactName::Wonder(wonder_id) => ArtifactId::Wonder(*wonder_id),
                };

                self.attune_artifact(artifact_id, *first)
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
            CharacterMutation::AddTerrestrialSorcery(add_terrestrial) => {
                self.add_terrestrial_sorcery(add_terrestrial)
            }
            CharacterMutation::RemoveTerrestrialSorcery => self.remove_terrestrial_sorcery(),
            CharacterMutation::AddCelestialSorcery(add_celestial) => {
                self.add_celestial_sorcery(add_celestial)
            }
            CharacterMutation::RemoveCelestialSorcery => self.remove_celestial_sorcery(),
            CharacterMutation::AddSolarSorcery(add_solar) => self.add_solar_sorcery(add_solar),
            CharacterMutation::RemoveSolarSorcery => self.remove_solar_sorcery(),
            CharacterMutation::AddSorceryArchetypeMerit(
                sorcery_archetype_id,
                sorcery_archetype_merit_id,
                sorcery_archetype_merit,
            ) => self.add_sorcery_archetype_merit(
                *sorcery_archetype_id,
                *sorcery_archetype_merit_id,
                sorcery_archetype_merit,
            ),
            CharacterMutation::RemoveSorceryArchetypeMerit(sorcery_archetype_merit_id) => {
                self.remove_sorcery_archetype_merit(*sorcery_archetype_merit_id)
            }
            CharacterMutation::AddDemense(name, geomancy_level) => {
                self.add_demense(name.as_str(), *geomancy_level)
            }
            CharacterMutation::AddExaltedHealing => self.add_exalted_healing(),
            CharacterMutation::RemoveStackableMerit(stackable_merit_id) => {
                self.remove_stackable_merit(*stackable_merit_id)
            }
            CharacterMutation::RemoveNonStackableMerit(nonstackable_merit_id) => {
                self.remove_nonstackable_merit(*nonstackable_merit_id)
            }
            CharacterMutation::RemoveExaltedHealing => self.remove_exalted_healing(),
            CharacterMutation::RemoveDemense(name) => self.remove_demense(name.as_str()),
            CharacterMutation::AddCharm(charm) => match charm {
                CharmMutation::Eclipse(spirit_charm_id, eclipse_charm) => {
                    self.add_eclipse_charm(*spirit_charm_id, eclipse_charm)
                }
                CharmMutation::Evocation(evocation_id, evocation) => {
                    self.add_evocation(*evocation_id, evocation)
                }
                CharmMutation::MartialArts(ma_charm_id, ma_charm) => {
                    self.add_martial_arts_charm(*ma_charm_id, ma_charm)
                }
                CharmMutation::Solar(solar_charm_id, solar_charm) => {
                    self.add_solar_charm(*solar_charm_id, solar_charm)
                }
                CharmMutation::Spell(spell_id, spell) => self.add_spell(*spell_id, spell),
            },
            CharacterMutation::RemoveCharm(charm_id) => match charm_id {
                CharmId::Spirit(spirit_charm_id) => self.remove_eclipse_charm(*spirit_charm_id),
                CharmId::Evocation(evocation_id) => self.remove_evocation(*evocation_id),
                CharmId::MartialArts(ma_charm_id) => self.remove_martial_arts_charm(*ma_charm_id),
                CharmId::Solar(solar_charm_id) => self.remove_solar_charm(*solar_charm_id),
                CharmId::Spell(spell_id) => self.remove_spell(*spell_id),
            },
            CharacterMutation::AddFlaw(flaw_mutation) => self.add_flaw(flaw_mutation),
            CharacterMutation::RemoveFlaw(name) => self.remove_flaw(name.as_str()),
            CharacterMutation::AddIntimacy(intimacy) => self.add_intimacy(intimacy),
            CharacterMutation::RemoveIntimacy(intimacy) => self.remove_intimacy(intimacy),
            CharacterMutation::GainLimit(amount) => self.gain_limit(*amount),
            CharacterMutation::ReduceLimit(amount) => self.reduce_limit(*amount),
            CharacterMutation::SetLimitTrigger(trigger) => self.set_limit_trigger(trigger.as_str()),
            CharacterMutation::GainExperience(amount) => self.gain_base_experience(*amount),
            CharacterMutation::SpendExperience(amount) => self.spend_base_experience(*amount),
            CharacterMutation::GainExaltExperience(amount) => self.gain_exalt_experience(*amount),
            CharacterMutation::SpendExaltExperince(amount) => self.spend_exalt_experience(*amount),
        }
    }
}
