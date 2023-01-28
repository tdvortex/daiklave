mod error;
use crate::abilities::SetAbility;
pub use crate::attributes::SetAttribute;
pub use crate::exaltation::exalt::essence::{SetEssenceRating, SpendMotes, CommitMotes, RecoverMotes, UncommitMotes};
pub use crate::health::{HealDamage, SetHealthTrack, TakeDamage};
pub use crate::name::SetName;
pub use crate::concept::{RemoveConcept, SetConcept};
pub use crate::exaltation::mortal::SetMortal;
pub use crate::exaltation::exalt::exalt_type::solar::SetSolar;
pub use crate::willpower::{GainWillpower, SpendWillpower, SetWillpowerRating};
use std::num::{NonZeroU16, NonZeroU8};

pub use error::CharacterMutationError;

use crate::{
    abilities::AbilityNameVanilla,
    armor::armor_item::{mundane::AddMundaneArmor, ArmorNameMutation},
    artifact::{AddArtifact, ArtifactNameMutation},
    charms::charm::{AddCharm, CharmNameMutation},
    exaltation::exalt::{
        essence::{MotePoolName},
    },
    flaws::flaw::FlawMutation,
    hearthstones::hearthstone::{AddHearthstone, AddManse, GeomancyLevel},
    intimacies::intimacy::IntimacyMutation,
    languages::language::LanguageMutation,
    martial_arts::style::MartialArtsStyle,
    merits::merit::{NonStackableMerit, NonStackableMeritId, StackableMerit, StackableMeritId},
    sorcery::{AddSorcery, SorceryArchetypeMerit, SorceryArchetypeName},
    weapons::weapon::{mundane::AddMundaneWeapon, EquipHand, Equipped, WeaponNameMutation},
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
    AddSpecialty(AbilityNameVanilla, String),
    /// Removes a specialty from a non-Craft, non-Martial Arts ability.
    RemoveSpecialty(AbilityNameVanilla, String),
    /// Adds a Martial Arts style to a character. This purchases the
    /// MartialArtist merit for the style, but does not grant any Martial Arts
    /// dots or Martial Arts charms.
    AddMartialArtsStyle(String, MartialArtsStyle),
    /// Removes a Martial Arts style from a character, including the merit,
    /// associated ability dots, specialties, and Charms.
    RemoveMartialArtsStyle(String),
    /// Sets the Ability dots for a specific Martial Arts style.
    SetMartialArtsDots(String, u8),
    /// Sets the Craft dots for a particular focus area.
    SetCraftDots(String, u8),
    /// Adds a mundane weapon to the character.
    AddMundaneWeapon(AddMundaneWeapon),
    /// Removes a mundane weapon from the character.
    RemoveMundaneWeapon(String),
    /// Equips the specific weapon. For a OneHanded weapon, will equip into
    /// the specified hand, otherwise the parameter is ignored.
    EquipWeapon(WeaponNameMutation, Option<EquipHand>),
    /// Unequips the specific weapon at the specified equipped position.
    UnequipWeapon(WeaponNameMutation, Equipped),
    /// Add an artifact to the character, which may be a weapon, armor item,
    /// warstrider, or wonder.
    AddArtifact(AddArtifact),
    /// Removes an artifact from the character.
    RemoveArtifact(ArtifactNameMutation),
    /// Adds a piece of mundane armor.
    AddMundaneArmor(AddMundaneArmor),
    /// Removes a piece of mundane armor from the character.
    RemoveMundaneArmor(String),
    /// Equip a specific piece of armor.
    EquipArmor(ArmorNameMutation),
    /// Unequip any armor currently worn.
    UnequipArmor,
    /// Add a manse, its associated demense, and its associated hearthstone
    /// to the character.
    AddManse(AddManse), // Manse, demense, hearthstone
    /// Add a hearthstone to a character without a manse.
    AddHearthstone(AddHearthstone),
    /// Add a demense to a character without a manse.
    AddDemense(String, GeomancyLevel),
    /// Remove a demense (without a manse) from a character.
    RemoveDemense(String),
    /// Slot a hearthstone into an artifact.
    SlotHearthstone(ArtifactNameMutation, String),
    /// Unslot a hearthstone from its current position.
    UnslotHearthstone(String),
    /// Remove a hearthstone from the character, unslotting it in the process
    /// if needed.
    RemoveHearthstone(String),
    /// Attune to an artifact, committing motes to its ongoing use.
    AttuneArtifact(ArtifactNameMutation, MotePoolName),
    /// Add a stackable merit with an id for this instance and detail
    AddStackableMerit(StackableMeritId, StackableMerit),
    /// Remove a stackable merit
    RemoveStackableMerit(StackableMeritId),
    /// Add a nonstackable merit
    AddNonStackableMerit(NonStackableMeritId, NonStackableMerit),
    /// Remove a nonstackable merit
    RemoveNonStackableMerit(NonStackableMeritId),
    /// Add a language
    AddLanguage(LanguageMutation),
    /// Set the character's native language.
    SetNativeLanguage(LanguageMutation),
    /// Remove a language from the character
    RemoveLanguage(LanguageMutation),
    /// Adds the Exalted Healing merit to the character. This is not required
    /// for Exalts.
    AddExaltedHealing,
    /// Removes the Exalted Healing merit from the character. This is not
    /// allowed for Exalts.
    RemoveExaltedHealing,
    /// Adds a circle of Sorcery to a character. The circle, archetype, shaping
    /// ritual, and control spell must be provided. Circles must be provided in
    /// order: Terrestrial, Celestial, Solar.
    AddSorcery(Box<AddSorcery>),
    /// Removes the currently highest-known level of sorcery from the
    /// character.
    RemoveSorcery,
    /// Adds a merit tied to a Sorcery Archetype owned by the character.
    AddSorceryArchetypeMerit(
        SorceryArchetypeName,
        String,
        SorceryArchetypeMerit,
    ),
    /// Removes a sorcery archetype merit.
    RemoveSorceryArchetypeMerit(String),
    /// Adds a Charm to the character.
    AddCharm(AddCharm),
    /// Removes a Charm from the character. Note that this may cause cascading
    /// drops due to Charm tree dependencies.
    RemoveCharm(CharmNameMutation),
    /// Adds a Flaw to the character.
    AddFlaw(FlawMutation),
    /// Removes a Flaw from the character.
    RemoveFlaw(String),
    /// Adds an Intimacy to the character.
    AddIntimacy(IntimacyMutation),
    /// Removes an Intimacy from a character
    RemoveIntimacy(IntimacyMutation),
    /// Increases the Exalt's Limit track.
    GainLimit(NonZeroU8),
    /// Reduces the Exalt's Limit track.
    ReduceLimit(NonZeroU8),
    /// Sets the Exalt's Limit trigger.
    SetLimitTrigger(String),
    /// Adds normal, non-Exalt experience
    GainExperience(NonZeroU16),
    /// Spends normal, non-Exalt experience
    SpendExperience(NonZeroU16),
    /// Adds Exalt experience (Solar Experience, for example)
    GainExaltExperience(NonZeroU16),
    /// Spends Exalt experience
    SpendExaltExperince(NonZeroU16),
}
