mod error;
use crate::{Character};
pub use crate::abilities::{SetAbility, AddSpecialty, RemoveSpecialty};
pub use crate::armor::armor_item::{EquipArmor, UnequipArmor};
pub use crate::armor::armor_item::mundane::{AddMundaneArmor, RemoveMundaneArmor};
pub use crate::artifact::AttuneArtifact;
pub use crate::attributes::SetAttribute;
pub use crate::charms::charm::{AddCharm, RemoveCharm};
pub use crate::exaltation::exalt::essence::{SetEssenceRating, SpendMotes, CommitMotes, RecoverMotes, UncommitMotes};
pub use crate::exaltation::exalt::limit::{GainLimit, ReduceLimit, SetLimitTrigger};
pub use crate::experience::{GainExperience, GainExaltExperience, SpendExperience, SpendExaltExperience};
pub use crate::flaws::flaw::RemoveFlaw;
pub use crate::health::{HealDamage, SetHealthTrack, TakeDamage};
pub use crate::hearthstones::hearthstone::{SlotHearthstone, UnslotHearthstone};
pub use crate::intimacies::intimacy::{AddIntimacy, RemoveIntimacy};
use crate::languages::AddLanguages;
use crate::languages::language::RemoveLanguage;
pub use crate::languages::language::SetNativeLanguage;
use crate::merits::merit::AddMerit;
pub use crate::name::SetName;
pub use crate::concept::{RemoveConcept, SetConcept};
pub use crate::exaltation::mortal::SetMortal;
pub use crate::exaltation::exalt::exalt_type::solar::SetSolar;
pub use crate::sorcery::{AddSorcery, RemoveSorcery};
pub use crate::weapons::weapon::{EquipWeapon, UnequipWeapon};
pub use crate::weapons::weapon::mundane::{AddMundaneWeapon, RemoveMundaneWeapon};
pub use crate::willpower::{GainWillpower, SpendWillpower, SetWillpowerRating};

pub use error::CharacterMutationError;

use crate::{
    flaws::flaw::AddFlaw,
};

/// The API for the character, expressed as an owned struct. Each mutation has
/// an associated pub method on Character and CharacterEventSource which
/// returns Result<&mut Self, CharacterMutationError>. All API events also has
///  a "check_" variant which returns Result<(), CharacterMutationError>.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterMutation {
    /// Set the Character's name
    SetName(SetName),
    /// Set the Character's concept
    SetConcept(SetConcept),
    /// Remove the Character's concept
    RemoveConcept,
    /// Set character to be mortal
    SetMortal,
    /// Set character to be Solar
    SetSolar(SetSolar),
    /// Spend motes, starting with one pool
    SpendMotes(SpendMotes),
    /// Commit motes into a persistent effect, starting with one pool
    CommitMotes(CommitMotes),
    /// Recover motes, always starting from peripheral
    RecoverMotes(RecoverMotes),
    /// Uncommit motes from a peristent effect
    UncommitMotes(UncommitMotes),
    /// Set the Essence rating of the character. Note: also ends all mote
    /// commitments and recovers all motes.
    SetEssenceRating(SetEssenceRating),
    GainWillpower(GainWillpower),
    SpendWillpower(SpendWillpower),
    /// Sets the permanent willpower rating of the character. Also resets
    /// current willpower to permanent rating.
    SetWillpowerRating(SetWillpowerRating),
    /// Changes the character's health track to have the specified wound
    /// penalties.
    SetHealthTrack(SetHealthTrack),
    /// Adds the specified amount and type of damage to the character's
    /// health track, accounting for overflows.
    TakeDamage(TakeDamage),
    /// Heals the specified amount of damage, always bashing then lethal then
    /// aggravated.
    HealDamage(HealDamage),
    /// Sets an attribute to a specific rating.
    SetAttribute(SetAttribute),
    /// Sets an ability (other than Craft or Martial Arts) to a dot rating.
    SetAbility(SetAbility),
    /// Adds a specialty to a non-zero, non-Craft, non-Martial Arts ability.
    AddSpecialty(AddSpecialty),
    /// Removes a specialty from a non-Craft, non-Martial Arts ability.
    RemoveSpecialty(RemoveSpecialty),
    /// Adds a mundane weapon to the character.
    AddMundaneWeapon(AddMundaneWeapon),
    /// Removes a mundane weapon from the character.
    RemoveMundaneWeapon(RemoveMundaneWeapon),
    /// Equips the specific weapon. For a OneHanded weapon, will equip into
    /// the specified hand, otherwise the parameter is ignored.
    EquipWeapon(EquipWeapon),
    /// Unequips the specific weapon at the specified equipped position.
    UnequipWeapon(UnequipWeapon),
    /// Adds a piece of mundane armor.
    AddMundaneArmor(AddMundaneArmor),
    /// Removes a piece of mundane armor from the character.
    RemoveMundaneArmor(RemoveMundaneArmor),
    /// Equip a specific piece of armor.
    EquipArmor(EquipArmor),
    /// Unequip any armor currently worn.
    UnequipArmor,
    /// Slot a hearthstone into an artifact.
    SlotHearthstone(SlotHearthstone),
    /// Unslot a hearthstone from its current position.
    UnslotHearthstone(UnslotHearthstone),
    /// Attune to an artifact, committing motes to its ongoing use.
    AttuneArtifact(AttuneArtifact),
    /// Set the character's native language.
    SetNativeLanguage(SetNativeLanguage),
    /// Adds a (non-native) language to the character.
    AddLanguage(AddLanguage),
    /// Removes a language from the character.
    RemoveLanguage(RemoveLanguage),
    /// Adds a circle of Sorcery to a character. The circle, archetype, shaping
    /// ritual, and control spell must be provided. Circles must be provided in
    /// order: Terrestrial, Celestial, Solar.
    AddSorcery(AddSorcery),
    /// Removes the currently highest-known level of sorcery from the
    /// character.
    RemoveSorcery,
    /// Adds a Charm to the character.
    AddCharm(AddCharm),
    /// Removes a Charm from the character. Note that this may cause cascading
    /// drops due to Charm tree dependencies.
    RemoveCharm(RemoveCharm),
    /// Adds a Flaw to the character.
    AddFlaw(AddFlaw),
    /// Removes a Flaw from the character.
    RemoveFlaw(RemoveFlaw),
    /// Adds an Intimacy to the character.
    AddIntimacy(AddIntimacy),
    /// Removes an Intimacy from a character
    RemoveIntimacy(RemoveIntimacy),
    /// Increases the Exalt's Limit track.
    GainLimit(GainLimit),
    /// Reduces the Exalt's Limit track.
    ReduceLimit(ReduceLimit),
    /// Sets the Exalt's Limit trigger.
    SetLimitTrigger(SetLimitTrigger),
    /// Adds normal, non-Exalt experience
    GainExperience(GainExperience),
    /// Spends normal, non-Exalt experience
    SpendExperience(SpendExperience),
    /// Adds Exalt experience (Solar Experience, for example)
    GainExaltExperience(GainExaltExperience),
    /// Spends Exalt experience
    SpendExaltExperience(SpendExaltExperience),
    AddMerit(AddMerit),
    RemoveMerit(RemoveMerit),
}

impl<'source> CharacterMutation {
    pub fn apply_mutation(&'source self, character: &mut Character<'source>) -> Result<&mut Character, CharacterMutationError> {
        character.apply_mutation(self)
    }
}

