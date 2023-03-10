use std::collections::HashSet;

use crate::{
    book_reference::BookReference, exaltation::exalt::essence::MoteCommitment,
    hearthstones::hearthstone::Hearthstone, merits::merit::Merit,
};

use self::{
    artifact::{builder::ArtifactWeaponBuilder, AddBaseArtifactWeapon},
    base::{builder::BaseWeaponBuilder, BaseWeapon},
    range::WeaponRange,
};

mod ability;

/// Traits that are specific to Artifact weapons.
pub mod artifact;
mod base;

/// Builder structs for weapons.
pub mod builder;
mod damage_type;
mod equip;
pub(crate) mod equipped;
mod handedness;
mod name;
mod unequip;
pub use unequip::UnequipWeapon;

/// Traits that are specific to mundane (non-Artifact) weapons.
pub mod mundane;
mod range;
mod tag;
mod weapon_type;
mod weight_class;

pub use equip::EquipWeapon;
pub use equipped::{EquipHand, Equipped};
pub use name::WeaponName;
pub(crate) use name::WeaponNameMutation;
pub use range::{AttackRange, RangeBand};
pub use tag::{OptionalWeaponTag, WeaponTag};
pub(crate) use weapon_type::WeaponType;
pub use weight_class::WeaponWeightClass;

/// The interface for a specific individual weapon
pub struct Weapon<'source>(pub(crate) WeaponType<'source>);

impl<'source> Weapon<'source> {
    pub(crate) fn merits(&self) -> Vec<Merit<'source>> {
        self.0.merits()
    }

    /// Starts constructing a base weapon, which is either a mundane
    /// weapon (like "sword") or base artifact weapon (like "daiklave").
    pub fn new_base(name: impl Into<String>) -> BaseWeaponBuilder {
        BaseWeaponBuilder {
            name: name.into(),
            book_reference: None,
            attack_range: WeaponRange::ContactOnly,
            tags: HashSet::new(),
        }
    }

    /// Starts constructing a unique, named artifact weapon (like "Volcano
    /// Cutter").
    pub fn new_artifact(base_weapon: AddBaseArtifactWeapon) -> ArtifactWeaponBuilder {
        ArtifactWeaponBuilder {
            base_weapon_name: base_weapon.name,
            base_weapon: base_weapon.weapon,
            book_reference: None,
            lore: None,
            powers: None,
        }
    }

    /// Returns true if the weapon is an artifact weapon
    pub fn is_artifact(&self) -> bool {
        self.0.is_artifact()
    }

    /// Returns true if the weapon is currently attuned
    pub fn is_attuned(&self) -> bool {
        self.0.is_attuned()
    }

    /// If the weapon is equipped (or natural) then returns an enum
    /// detailing that position; if unequipped, returns None
    pub fn is_equipped(&self) -> Option<Equipped> {
        self.0.is_equipped()
    }

    /// If the weapon is currently attuned, returns the commitment. Also
    /// returns the artifact weapon name for later unattunement.
    pub fn mote_commitment(&self) -> Option<MoteCommitment<'source>> {
        self.0.mote_commitment()
    }

    /// The name of the weapon, which is either the name of a generic mundane
    /// weapon like "sword" or the specific name of a unique artifact weapon,
    /// like "Volcano Cutter"
    pub fn name(&self) -> WeaponName<'source> {
        self.0.name()
    }

    /// The book reference for the item
    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference()
    }

    /// If the weapon is an artifact, the number of merit dots it is worth.
    /// Returns None for mundane weapons.
    pub fn merit_dots(&self) -> Option<u8> {
        self.0.merit_dots()
    }

    /// If the weapon is an artifact, it may have a lore description. Always
    /// None for mundane weapons.
    pub fn lore(&self) -> Option<&'source str> {
        self.0.lore()
    }

    /// If the weapon is an artifact, it may have unique powers. Always None
    /// for mundane weapons.
    pub fn powers(&self) -> Option<&'source str> {
        self.0.powers()
    }

    /// The number of hearthstone slots (occupied and unoccupied) in the
    /// weapon. Always 0 for mundane weapons.
    pub fn hearthstone_slots(&self) -> u8 {
        self.0.hearthstone_slots()
    }

    /// An iterator over all of the hearthstones currently slotted into the
    /// artifact weapon. Returns an empty iterator for mundane weapons.
    pub fn slotted_hearthstones(&self) -> impl Iterator<Item = Hearthstone<'source>> + '_ {
        self.0.slotted_hearthstones()
    }

    /// The number of open hearthstone slots in the weapon.
    pub fn open_slots(&self) -> u8 {
        self.0.open_slots()
    }

    /// If the weapon is an artifact weapon, returns the base weapon and its Id.
    /// For example, the base weapon for "Volcano Cutter" would be "daiklave".
    pub fn base_artifact_weapon(&self) -> Option<(&'source str, &'source BaseWeapon)> {
        self.0.base_artifact_weapon()
    }

    /// The tags associated with the weapon. Follows the ordering conventions
    /// in the core rulebook: Lethal/Bashing, then
    /// Archery/Brawl/Melee/MartialArts/Thrown, then other tags in alphabetical
    /// order. Note: Archery weapons are two-handed by default.
    pub fn tags(&self) -> impl Iterator<Item = WeaponTag> + '_ {
        self.0.tags()
    }

    /// The weight class of the weapon.
    pub fn weight_class(&self) -> WeaponWeightClass {
        self.0.weight_class()
    }

    /// The accuracy of the weapon, at the specified attack range. Returns
    /// None if the weapon is unusable at that range. Note that Medium
    /// Melee/Thrown weapons have different accuracies at Close range
    /// depending on if they are wielded as melee or thrown.
    pub fn accuracy(&self, attack_range: AttackRange) -> Option<i8> {
        self.0.accuracy(attack_range)
    }

    /// The damage rating of the weapon at the specified attack range.
    /// Returns None if unusable at that range. Note that the Powerful
    /// tag modifies weapon damage at close range meaning that in some
    /// cases the weapon damage is not static across ranges.
    pub fn damage(&self, attack_range: AttackRange) -> Option<u8> {
        self.0.damage(attack_range)
    }

    /// The weapon's bonus or penalty to Parry defense. Returns None
    /// if the weapon cannot be used to parry.
    pub fn parry_mod(&self) -> Option<i8> {
        self.0.parry_mod()
    }

    /// The weapon's Overwhelming value.
    pub fn overwhelming(&self) -> u8 {
        self.0.overwhelming()
    }

    /// Returns true if the weapon is a Natural weapon.
    pub fn is_natural(&self) -> bool {
        self.0.is_natural()
    }

    /// Returns true if the weapon is a Worn weapon.
    pub fn is_worn(&self) -> bool {
        self.0.is_worn()
    }

    /// Returns true if the weapon is a One-Handed weapon.
    pub fn is_one_handed(&self) -> bool {
        self.0.is_one_handed()
    }

    /// Returns true if the weapon is a Two-Handed weapon.
    pub fn is_two_handed(&self) -> bool {
        self.0.is_two_handed()
    }

    /// The number of copies of this weapon (in this equipped position).
    pub fn quantity(&self) -> u8 {
        self.0.quantity()
    }
}
