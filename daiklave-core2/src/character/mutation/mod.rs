mod error;
pub use error::CharacterMutationError;

use crate::{
    abilities::AbilityNameVanilla,
    armor::armor_item::{mundane::MundaneArmor, ArmorId, BaseArmorId},
    artifact::{Artifact, ArtifactId},
    attributes::AttributeName,
    exaltation::exalt::{
        essence::{MoteCommitmentId, MotePoolName, OtherMoteCommitmentId},
        exalt_type::solar::NewSolar,
    },
    health::{DamageLevel, WoundPenalty},
    hearthstones::{
        hearthstone::{GeomancyLevel, HearthstoneTemplate},
        HearthstoneId,
    },
    languages::language::LanguageMutation,
    martial_arts::{MartialArtsStyle, MartialArtsStyleId},
    merits::merit::{NonStackableMerit, NonStackableMeritId, StackableMerit, StackableMeritId},
    sorcery::{
        circles::{
            celestial::AddCelestialSorcery, solar::AddSolarSorcery,
            terrestrial::AddTerrestrialSorcery,
        },
        SorceryArchetypeId, SorceryArchetypeMerit, SorceryArchetypeMeritId,
    },
    unique_id::UniqueId,
    weapons::weapon::{mundane::MundaneWeapon, BaseWeaponId, EquipHand, Equipped, WeaponId},
};

/// The API for the character, expressed as an owned struct. Each mutation has
/// an associated pub method on Character and CharacterEventSource which
/// returns Result<&mut Self, CharacterMutationError>. All API events also has
///  a "check_" variant which returns Result<(), CharacterMutationError>.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterMutation {
    /// Set the Character's name
    SetName(String),
    /// Set the Character's concept
    SetConcept(String),
    /// Remove the Character's concept
    RemoveConcept,
    /// Set character to be mortal
    SetMortal,
    /// Set character to be Solar
    SetSolar(NewSolar),
    /// Spend motes, starting with one pool
    SpendMotes(MotePoolName, u8),
    /// Commit motes into a persistent effect, starting with one pool
    CommitMotes(OtherMoteCommitmentId, String, MotePoolName, u8),
    /// Recover motes, always starting from peripheral
    RecoverMotes(u8),
    /// Uncommit motes from a peristent effect
    UncommitMotes(MoteCommitmentId),
    /// Set the Essence rating of the character. Note: also ends all mote
    /// commitments and recovers all motes.
    SetEssenceRating(u8),
    /// Sets the current willpower value of the character.
    SetCurrentWillpower(u8),
    /// Sets the permanent willpower rating of the character. Also resets
    /// current willpower to permanent rating.
    SetWillpowerRating(u8),
    /// Changes the character's health track to have the specified wound
    /// penalties.
    SetWoundPenalties(Vec<WoundPenalty>),
    /// Adds the specified amount and type of damage to the character's
    /// health track, accounting for overflows.
    TakeDamage(DamageLevel, u8),
    /// Heals the specified amount of damage, always bashing then lethal then
    /// aggravated.
    HealDamage(u8),
    /// Sets an attribute to a specific rating.
    SetAttribute(AttributeName, u8),
    /// Sets an ability (other than Craft or Martial Arts) to a dot rating.
    SetAbilityDots(AbilityNameVanilla, u8),
    /// Adds a specialty to a non-zero, non-Craft, non-Martial Arts ability.
    AddSpecialty(AbilityNameVanilla, String),
    /// Removes a specialty from a non-Craft, non-Martial Arts ability.
    RemoveSpecialty(AbilityNameVanilla, String),
    /// Adds a Martial Arts style to a character. This purchases the
    /// MartialArtist merit for the style, but does not grant any Martial Arts
    /// dots or Martial Arts charms.
    AddMartialArtsStyle(MartialArtsStyleId, MartialArtsStyle),
    /// Removes a Martial Arts style from a character, including the merit,
    /// associated ability dots, specialties, and Charms.
    RemoveMartialArtsStyle(MartialArtsStyleId),
    /// Sets the Ability dots for a specific Martial Arts style.
    SetMartialArtsDots(MartialArtsStyleId, u8),
    /// Sets the Craft dots for a particular focus area.
    SetCraftDots(String, u8),
    /// Adds a mundane weapon to the character.
    AddMundaneWeapon(BaseWeaponId, MundaneWeapon),
    /// Removes a mundane weapon from the character.
    RemoveMundaneWeapon(BaseWeaponId),
    /// Equips the specific weapon. For a OneHanded weapon, will equip into
    /// the specified hand, otherwise the parameter is ignored.
    EquipWeapon(WeaponId, Option<EquipHand>),
    /// Unequips the specific weapon at the specified equipped position.
    UnequipWeapon(WeaponId, Equipped),
    /// Add an artifact to the character, which may be a weapon, armor item,
    /// warstrider, or wonder.
    AddArtifact(Artifact),
    /// Removes an artifact from the character.
    RemoveArtifact(ArtifactId),
    /// Adds a piece of mundane armor.
    AddMundaneArmor(BaseArmorId, MundaneArmor),
    /// Removes a piece of mundane armor from the character.
    RemoveMundaneArmor(BaseArmorId),
    /// Equip a specific piece of armor.
    EquipArmor(ArmorId),
    /// Unequip any armor currently worn.
    UnequipArmor,
    /// Add a manse, its associated demense, and its associated hearthstone
    /// to the character.
    AddManse(String, String, HearthstoneId, HearthstoneTemplate),
    /// Add a hearthstone to a character without a manse.
    AddHearthstone(HearthstoneId, HearthstoneTemplate),
    /// Add a demense to a character without a manse.
    AddDemense(UniqueId, String, GeomancyLevel),
    /// Remove a demense (without a manse) from a character.
    RemoveDemense(UniqueId),
    /// Slot a hearthstone into an artifact.
    SlotHearthstone(ArtifactId, HearthstoneId),
    /// Unslot a hearthstone from its current position.
    UnslotHearthstone(HearthstoneId),
    /// Remove a hearthstone from the character, unslotting it in the process
    /// if needed.
    RemoveHearthstone(HearthstoneId),
    /// Attune to an artifact, committing motes to its ongoing use.
    AttuneArtifact(ArtifactId, MotePoolName),
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
    /// Adds Terrestrial circle sorcery to the character. The archetype,
    /// shaping ritual, and control spell must be provided, along with an
    /// Id for each.
    AddTerrestrialSorcery(Box<AddTerrestrialSorcery>),
    /// Removes Terrestrial circle sorcery from the character, making them no
    /// longer a sorcerer.
    RemoveTerrestrialSorcery,
    /// Increases the character from Terrestrial sorcery to Celestial. If the
    /// character is adding a shaping ritual to an already-known archetype, the
    /// SorceryArchetype text may be left as None, otherwise it is required.
    AddCelestialSorcery(Box<AddCelestialSorcery>),
    /// Removes Celestial circle sorcery from the character, making them a
    /// Terrestrial circle sorcerer.
    RemoveCelestialSorcery,
    /// Increases the character from Celestial sorcery to Solar circle. If the
    /// character is adding a shaping ritual to an already-known archetype, the
    /// SorceryArchetype text may be left as None, otherwise it is required.
    AddSolarSorcery(Box<AddSolarSorcery>),
    /// Removes Solar circle sorcery from the character, making them a
    /// Celestial circle sorcerer.
    RemoveSolarSorcery,
    /// Adds a merit tied to a Sorcery Archetype owned by the character.
    AddSorceryArchetypeMerit(
        SorceryArchetypeId,
        SorceryArchetypeMeritId,
        SorceryArchetypeMerit,
    ),
    /// Removes a sorcery archetype merit.
    RemoveSorceryArchetypeMerit(SorceryArchetypeMeritId),
}
