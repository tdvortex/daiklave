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
    book_reference::BookReference,
    charms::charm::{AddCharm, CharmNameMutation},
    craft::Craft,
    exaltation::Exaltation,
    experience::ExperiencePool,
    health::Health,
    hearthstones::{hearthstone::GeomancyLevel, UnslottedHearthstone},
    intimacies::intimacy::{IntimacyLevel, IntimacyType},
    languages::Languages,
    merits::merit::{
        NonStackableMeritView, StackableMeritView,
    },
    sorcery::{SorceryCircle, SorceryError},
    willpower::Willpower,
};

use self::mutation::{SpendMotes, SetName, SetConcept, CommitMotes, TakeDamage, RecoverMotes, SetEssenceRating, SetWillpowerRating, HealDamage, SetAttribute, SetAbility};

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
    pub(crate) hearthstone_inventory: HashMap<&'source str, UnslottedHearthstone<'source>>,
    pub(crate) demenses_no_manse: HashMap<&'source str, GeomancyLevel>,
    pub(crate) stackable_merits: HashMap<(&'source str, &'source str), StackableMeritView<'source>>, // Keyed by the 
    pub(crate) nonstackable_merits: HashMap<&'source str, NonStackableMeritView<'source>>, // Keyed by the template name
    pub(crate) flaws: HashMap<&'source str, (Option<BookReference>, &'source str)>,
    pub(crate) languages: Languages<'source>,
    pub(crate) intimacies: HashMap<IntimacyType<'source>, IntimacyLevel>,
    pub(crate) experience: ExperiencePool,
}

impl<'source> Character<'source> {
    /// Applies a specific CharacterMutation or returns an error.
    pub fn apply_mutation(
        &mut self,
        mutation: &'source CharacterMutation,
    ) -> Result<&mut Self, CharacterMutationError> {
        match mutation {
            CharacterMutation::SetName(SetName(name)) => self.set_name(name.as_str()),
            CharacterMutation::SetConcept(SetConcept(concept)) => self.set_concept(concept.as_str()),
            CharacterMutation::RemoveConcept => self.remove_concept(),
            CharacterMutation::SetMortal => self.set_mortal(),
            CharacterMutation::SetSolar(set_solar) => self.set_solar(set_solar),
            CharacterMutation::SpendMotes(SpendMotes {
                first,
                amount,
            }) => self.spend_motes(*first, amount.get()),
            CharacterMutation::CommitMotes(CommitMotes {
                effect_name,
                first,
                amount,
            }) => {
                self.commit_motes(effect_name.as_str(), *first, amount.get())
            }
            CharacterMutation::RecoverMotes(RecoverMotes(amount)) => self.recover_motes(amount.get()),
            CharacterMutation::UncommitMotes(uncommit_motes) => self.uncommit_motes(uncommit_motes),
            CharacterMutation::SetEssenceRating(SetEssenceRating(rating)) => self.set_essence_rating(rating.get()),
            CharacterMutation::SetWillpowerRating(SetWillpowerRating(dots)) => self.set_willpower_rating(*dots),
            CharacterMutation::TakeDamage(TakeDamage{
                level,
                amount,
            }) => {
                self.take_damage(*level, amount.get())
            }
            CharacterMutation::HealDamage(HealDamage(amount)) => self.heal_damage(amount.get()),
            CharacterMutation::SetAttribute(SetAttribute {
                name,
                dots,
            }) => {
                self.set_attribute(*name, dots.get())
            }
            CharacterMutation::SetAbility(SetAbility{
                name,
                dots,
            }) => {
                self.set_ability_dots(*name, *dots)
            }
            CharacterMutation::AddSpecialty(ability_name, specialty) => {
                self.add_specialty(*ability_name, specialty.as_str())
            }
            CharacterMutation::RemoveSpecialty(ability_name, specialty) => {
                self.remove_specialty(*ability_name, specialty.as_str())
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
            CharacterMutation::RemoveMundaneWeapon(name) => {
                self.remove_mundane_weapon(name.as_str())
            }
            CharacterMutation::AddMundaneArmor((name, armor_item)) => {
                self.add_mundane_armor(name.as_str(), armor_item)
            }
            CharacterMutation::EquipArmor(name) => self.equip_armor(name.as_ref()),
            CharacterMutation::RemoveMundaneArmor(name) => self.remove_mundane_armor(name.as_str()),
            CharacterMutation::UnequipArmor => self.unequip_armor(),
            CharacterMutation::SlotHearthstone(artifact_name, hearthstone_name) => {
                self.slot_hearthstone(artifact_name.as_ref(), hearthstone_name.as_str())
            }
            CharacterMutation::UnslotHearthstone(hearthstone_name) => {
                self.unslot_hearthstone(hearthstone_name.as_str())
            }
            CharacterMutation::AttuneArtifact(artifact_name, first) => {
                self.attune_artifact(artifact_name.as_ref(), *first)
            }
            CharacterMutation::SetNativeLanguage(language_mutation) => {
                self.set_native_language(language_mutation)
            }
            CharacterMutation::AddExaltedHealing => self.add_exalted_healing(),
            CharacterMutation::RemoveExaltedHealing => self.remove_exalted_healing(),
            CharacterMutation::AddCharm(charm) => match charm {
                AddCharm::Eclipse((spirit_charm_name, eclipse_charm)) => {
                    self.add_eclipse_charm(spirit_charm_name.as_str(), eclipse_charm)
                }
                AddCharm::Evocation((evocation_name, evocation)) => {
                    self.add_evocation(evocation_name.as_str(), evocation)
                }
                AddCharm::MartialArts((ma_charm_name, ma_charm)) => {
                    self.add_martial_arts_charm(ma_charm_name.as_str(), ma_charm)
                }
                AddCharm::Solar((solar_charm_name, solar_charm)) => {
                    self.add_solar_charm(solar_charm_name.as_str(), solar_charm)
                }
                AddCharm::Spell((spell_name, spell)) => self.add_spell(spell_name.as_str(), spell),
            },
            CharacterMutation::RemoveCharm(charm_name) => match charm_name {
                CharmNameMutation::Spirit(spirit_charm_id) => {
                    self.remove_eclipse_charm(spirit_charm_id.as_str())
                }
                CharmNameMutation::Evocation(evocation_name) => {
                    self.remove_evocation(evocation_name.as_str())
                }
                CharmNameMutation::MartialArts(ma_charm_name) => {
                    self.remove_martial_arts_charm(ma_charm_name.as_str())
                }
                CharmNameMutation::Solar(solar_charm_name) => {
                    self.remove_solar_charm(solar_charm_name.as_str())
                }
                CharmNameMutation::Spell(spell_name) => self.remove_spell(spell_name.as_str()),
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
            CharacterMutation::RemoveSorcery => {
                let sorcery = self.sorcery().ok_or(CharacterMutationError::SorceryError(
                    SorceryError::CircleSequence,
                ))?;

                if sorcery.control_spell(SorceryCircle::Solar).is_some() {
                    self.remove_solar_sorcery()
                } else if sorcery.control_spell(SorceryCircle::Celestial).is_some() {
                    self.remove_celestial_sorcery()
                } else {
                    self.remove_terrestrial_sorcery()
                }
            }
        }
    }
}
