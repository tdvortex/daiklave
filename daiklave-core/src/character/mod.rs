mod default;

mod event;
pub use event::CharacterEvent;
mod event_source;
pub use event_source::CharacterEventSource;

mod memo;
pub use memo::CharacterMemo;

mod methods;

pub mod mutation;
pub use mutation::{CharacterMutation, CharacterMutationError};

mod redo;
mod undo;
pub use redo::Redo;
pub use undo::Undo;

use std::collections::{HashMap, HashSet};

use crate::{
    abilities::AbilitiesVanilla,
    attributes::Attributes,
    book_reference::BookReference,
    craft::Craft,
    exaltation::Exaltation,
    experience::ExperiencePool,
    health::Health,
    hearthstones::{hearthstone::GeomancyLevel, UnslottedHearthstone},
    intimacies::intimacy::{IntimacyLevel, IntimacyTypeMemo},
    languages::language::LanguageMutation,
    merits::merit::{NonStackableMeritInstance, StackableMeritInstance},
    willpower::Willpower,
};

use self::mutation::{
    AttuneArtifact, CommitMotes, EquipArmor, EquipWeapon, GainExaltExperience, GainExperience,
    GainLimit, GainWillpower, HealDamage, RecoverMotes, ReduceLimit, RemoveCharm, RemoveFlaw,
    RemoveMundaneArmor, RemoveMundaneWeapon, SetAttribute, SetConcept, SetEssenceRating,
    SetHealthTrack, SetLimitTrigger, SetName, SetWillpowerRating, SlotHearthstone,
    SpendExaltExperience, SpendExperience, SpendMotes, SpendWillpower, TakeDamage, UnequipWeapon,
    UnslotHearthstone,
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
    pub(crate) hearthstone_inventory: HashMap<&'source str, UnslottedHearthstone<'source>>,
    pub(crate) demenses_no_manse: HashMap<&'source str, GeomancyLevel>,
    pub(crate) stackable_merits:
        HashMap<(&'source str, &'source str), &'source StackableMeritInstance>,
    pub(crate) nonstackable_merits: HashMap<&'source str, &'source NonStackableMeritInstance>,
    pub(crate) flaws: HashMap<&'source str, (Option<BookReference>, &'source str)>,
    pub(crate) native_language: &'source LanguageMutation,
    pub(crate) other_languages: HashSet<&'source LanguageMutation>,
    pub(crate) intimacies: HashMap<&'source IntimacyTypeMemo, IntimacyLevel>,
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
            CharacterMutation::SetConcept(SetConcept(concept)) => {
                self.set_concept(concept.as_str())
            }
            CharacterMutation::RemoveConcept => self.remove_concept(),
            CharacterMutation::SetMortal => self.set_mortal(),
            CharacterMutation::SetSolar(set_solar) => self.set_solar(set_solar),
            CharacterMutation::SpendMotes(SpendMotes { first, amount }) => {
                self.spend_motes(*first, *amount)
            }
            CharacterMutation::CommitMotes(CommitMotes {
                effect_name,
                first,
                amount,
            }) => self.commit_motes(effect_name.as_str(), *first, *amount),
            CharacterMutation::RecoverMotes(RecoverMotes(amount)) => self.recover_motes(*amount),
            CharacterMutation::UncommitMotes(uncommit_motes) => self.uncommit_motes(uncommit_motes),
            CharacterMutation::SetEssenceRating(SetEssenceRating(rating)) => {
                self.set_essence_rating(*rating)
            }
            CharacterMutation::SetWillpowerRating(SetWillpowerRating(dots)) => {
                self.set_willpower_rating(*dots)
            }
            CharacterMutation::TakeDamage(TakeDamage { level, amount }) => {
                self.take_damage(*level, amount.get())
            }
            CharacterMutation::HealDamage(HealDamage(amount)) => self.heal_damage(amount.get()),
            CharacterMutation::SetAttribute(SetAttribute { name, dots }) => {
                self.set_attribute(*name, dots.get())
            }
            CharacterMutation::SetAbility(set_ability) => self.set_ability_dots(set_ability),
            CharacterMutation::AddSpecialty(add_specialty) => self.add_specialty(add_specialty),
            CharacterMutation::RemoveSpecialty(remove_specialty) => {
                self.remove_specialty(remove_specialty)
            }
            CharacterMutation::AddMundaneWeapon(add_mundane_weapon) => {
                self.add_mundane_weapon(add_mundane_weapon)
            }
            CharacterMutation::EquipWeapon(EquipWeapon { weapon_name, hand }) => {
                self.equip_weapon(weapon_name.into(), *hand)
            }
            CharacterMutation::UnequipWeapon(UnequipWeapon { name, equipped }) => {
                self.unequip_weapon(name.into(), *equipped)
            }
            CharacterMutation::RemoveMundaneWeapon(RemoveMundaneWeapon { name, quantity }) => {
                (1..=quantity.get()).try_fold(self, |acc, _| acc.remove_mundane_weapon(name))
            }
            CharacterMutation::AddMundaneArmor(add_mundane_armor) => {
                self.add_mundane_armor(add_mundane_armor)
            }
            CharacterMutation::EquipArmor(EquipArmor(name)) => self.equip_armor(name.into()),
            CharacterMutation::RemoveMundaneArmor(RemoveMundaneArmor(name)) => {
                self.remove_mundane_armor(name.as_str())
            }
            CharacterMutation::UnequipArmor => self.unequip_armor(),
            CharacterMutation::SlotHearthstone(SlotHearthstone {
                artifact_name,
                hearthstone_name,
            }) => self.slot_hearthstone(artifact_name.into(), hearthstone_name),
            CharacterMutation::UnslotHearthstone(UnslotHearthstone(hearthstone_name)) => {
                self.unslot_hearthstone(hearthstone_name)
            }
            CharacterMutation::AttuneArtifact(AttuneArtifact {
                artifact_name,
                first,
            }) => self.attune_artifact(artifact_name.into(), *first),
            CharacterMutation::SetNativeLanguage(language_mutation) => {
                self.set_native_language(language_mutation)
            }
            CharacterMutation::AddCharm(add_charm) => self.add_charm(add_charm),
            CharacterMutation::RemoveCharm(RemoveCharm(charm_name)) => {
                self.remove_charm(charm_name.into())
            }
            CharacterMutation::AddFlaw(add_flaw) => self.add_flaw(add_flaw),
            CharacterMutation::RemoveFlaw(RemoveFlaw(name)) => self.remove_flaw(name),
            CharacterMutation::AddIntimacy(add_intimacy) => self.add_intimacy(add_intimacy),
            CharacterMutation::RemoveIntimacy(remove_intimcay) => {
                self.remove_intimacy(remove_intimcay)
            }
            CharacterMutation::GainLimit(GainLimit(amount)) => self.gain_limit(*amount),
            CharacterMutation::ReduceLimit(ReduceLimit(amount)) => self.reduce_limit(*amount),
            CharacterMutation::SetLimitTrigger(SetLimitTrigger(trigger)) => {
                self.set_limit_trigger(trigger)
            }
            CharacterMutation::GainExperience(GainExperience(amount)) => {
                self.gain_base_experience(*amount)
            }
            CharacterMutation::SpendExperience(SpendExperience(amount)) => {
                self.spend_base_experience(*amount)
            }
            CharacterMutation::GainExaltExperience(GainExaltExperience(amount)) => {
                self.gain_exalt_experience(*amount)
            }
            CharacterMutation::SpendExaltExperience(SpendExaltExperience(amount)) => {
                self.spend_exalt_experience(*amount)
            }
            CharacterMutation::RemoveSorcery => self.remove_sorcery(),
            CharacterMutation::GainWillpower(GainWillpower(amount)) => self.gain_willpower(*amount),
            CharacterMutation::SpendWillpower(SpendWillpower(amount)) => {
                self.spend_willpower(*amount)
            }
            CharacterMutation::SetHealthTrack(SetHealthTrack(hashmap)) => {
                self.set_health_track(hashmap)
            }
            CharacterMutation::AddSorcery(add_sorcery) => self.add_sorcery(add_sorcery),
            CharacterMutation::AddMerit(add_merit) => self.add_merit(add_merit),
            CharacterMutation::RemoveMerit(remove_merit) => self.remove_merit(remove_merit),
            CharacterMutation::AddLanguage(add_language) => self.add_language(add_language),
            CharacterMutation::RemoveLanguage(remove_language) => {
                self.remove_language(remove_language)
            }
        }
    }
}

impl<'source> From<&'source CharacterMemo> for Character<'source> {
    fn from(memo: &'source CharacterMemo) -> Self {
        Self {
            name: &memo.name,
            concept: memo.concept.as_deref(),
            exaltation: (&memo.exaltation).into(),
            willpower: memo.willpower,
            health: memo.health,
            attributes: memo.attributes,
            abilities: (&memo.abilities).into(),
            craft: (&memo.craft).into(),
            hearthstone_inventory: memo
                .hearthstone_inventory
                .iter()
                .map(|(name, hearthstone)| (name.as_str(), hearthstone.into()))
                .collect(),
            demenses_no_manse: memo
                .demenses_no_manse
                .iter()
                .map(|(name, &level)| (name.as_str(), level))
                .collect(),
            stackable_merits: memo
                .stackable_merits
                .iter()
                .map(|((template_name, details), instance)| {
                    ((template_name.as_str(), details.as_str()), instance.into())
                })
                .collect(),
            nonstackable_merits: memo
                .nonstackable_merits
                .iter()
                .map(|(name, instance)| (name.as_str(), instance.into()))
                .collect(),
            flaws: memo
                .flaws
                .iter()
                .map(|(name, (book_reference, description))| {
                    (name.as_str(), (*book_reference, description.as_str()))
                })
                .collect(),
            native_language: &memo.native_language,
            other_languages: memo.other_languages.iter().collect(),
            intimacies: memo
                .intimacies
                .iter()
                .map(|(intimacy_type, &level)| (intimacy_type, level))
                .collect(),
            experience: memo.experience,
        }
    }
}
