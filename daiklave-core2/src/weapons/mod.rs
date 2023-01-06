mod ability;
pub(crate) mod artifact;
mod base;
/// Builders for weapons.
pub mod builder;
mod damage_type;
mod equipped;
pub(crate) mod exalt;
mod hearthstone;
pub(crate) mod mortal;
mod mundane;
mod range;
mod tag;
mod unarmed;
mod weapon_id;
mod weight_class;

use std::collections::HashSet;

pub use weapon_id::{ArtifactId, ArtifactWeaponId, BaseWeaponId, WeaponId};
pub use tag::{WeaponTag, OptionalWeaponTag};
pub use range::{AttackRange, RangeBand};

use crate::{
    book_reference::BookReference,
    exaltation::{exalt::essence::MoteCommitment, Exaltation},
};

use self::{
    base::BaseWeapon, hearthstone::OwnedHearthstone, mundane::MundaneWeapon, builder::{artifact::ArtifactWeaponBuilder, base::BaseWeaponBuilder}, range::WeaponRange,
};
pub(crate) use unarmed::unarmed;
pub use weight_class::WeaponWeightClass;
pub use artifact::ArtifactWeapon;
pub use mundane::MundaneWeaponMemo;

/// The interface for a character's weapons.
pub struct Weapons<'view, 'source>(pub(crate) &'view Exaltation<'source>);

impl<'view, 'source> Weapons<'view, 'source> {
    /// Retrieves the details for a specific weapon, if it exists.
    pub fn get(&self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        if matches!(weapon_id, WeaponId::Unarmed) {
            Some(unarmed())
        } else {
            self.0.get_weapon(weapon_id)
        }
    }

    /// Iterates over all of the weapons the character possesses by ID.
    pub fn iter(&self) -> impl Iterator<Item = WeaponId> + '_ {
        self.0.iter_weapons()
    }
}

/// The position of an equipped weapon.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Equipped {
    /// Natural weapons are always equipped.
    Natural,
    /// Worn weapons may be equipped without using a hand.
    Worn,
    /// One-handed weapons may be wielded in the main hand.
    MainHand,
    /// One-handed weapons may be wielded in the off hand.
    OffHand,
    /// Two-handed weapons require two hands to wield.
    TwoHanded,
}

/// For one-handed weapons, the position of that weapon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipHand {
    /// Wielded in the main hand
    MainHand,
    /// Wielded in the off hand
    OffHand,
}

impl Default for EquipHand {
    fn default() -> Self {
        Self::MainHand
    }
}

/// The interface for a specific individual weapon
pub struct Weapon<'source>(WeaponType<'source>);

impl<'view, 'source> Weapon<'source> {
    /// Starts constructing a base weapon, which is either a mundane
    /// weapon (like "sword") or base artifact weapon (like "daiklave").
    pub fn base(name: &str) -> BaseWeaponBuilder {
        BaseWeaponBuilder {
            name: name.to_owned(),
            book_reference: None,
            attack_range: WeaponRange::ContactOnly,
            tags: HashSet::new(),
        }
    }

    /// Starts constructing a unique, named artifact weapon (like "Volcano
    /// Cutter").
    pub fn artifact(name: &'source str) -> ArtifactWeaponBuilder<'source> {
        ArtifactWeaponBuilder {
            name,
            book_reference: None,
            lore: None,
            powers: None,
        }
    }

    /// The weapon's Id
    pub fn id(&self) -> WeaponId {
        self.0.id()
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
    /// returns the artifact weapon Id for later unattunement.
    pub fn mote_commitment(&self) -> Option<(ArtifactWeaponId, MoteCommitment<'source>)> {
        self.0.mote_commitment()
    }

    /// The name of the weapon, which is either the name of a generic mundane
    /// weapon like "sword" or the specific name of a unique artifact weapon,
    /// like "Volcano Cutter"
    pub fn name(&self) -> &'source str {
        self.0.name()
    }

    /// The book reference for the item
    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference()
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
    pub fn slotted_heathstones(
        &'view self,
    ) -> impl Iterator<Item = &'view OwnedHearthstone<'source>> + '_ {
        self.0.slotted_hearthstones()
    }

    /// If the weapon is an artifact weapon, returns the base weapon and its Id.
    /// For example, the base weapon for "Volcano Cutter" would be "daiklave".
    pub fn base_artifact_weapon(&self) -> Option<(BaseWeaponId, &BaseWeapon<'source>)> {
        self.0.base_artifact_weapon()
    }

    /// The tags associated with the weapon. Follows the ordering conventions
    /// in the core rulebook: Lethal/Bashing, then 
    /// Archery/Brawl/Melee/MartialArts/Thrown, then other tags in alphabetical
    /// order. Note: Archery weapons are two-handed by default.
    pub fn tags(&self) -> impl Iterator<Item = WeaponTag> {
        vec![].into_iter()
    }

    /// The weight class of the weapon.
    pub fn weight_class(&self) -> WeaponWeightClass {
        todo!()
    }

    /// The accuracy of the weapon, at the specified attack range. Returns
    /// None if the weapon is unusable at that range. Note that Medium
    /// Melee/Thrown weapons have different accuracies at Close range 
    /// depending on if they are wielded as melee or thrown.
    pub fn accuracy(&self, _attack_range: AttackRange) -> Option<u8> {
        todo!()
    }

    /// The damage rating of the weapon at the specified attack range.
    /// Returns None if unusable at that range. Note that the Powerful
    /// tag modifies weapon damage at close range meaning that in some
    /// cases the weapon damage is not static across ranges.
    pub fn damage(&self, _attack_range: AttackRange) -> Option<u8> {
        todo!()
    }

    /// The weapon's bonus or penalty to Parry defense. Returns None
    /// if the weapon cannot be used to parry.
    pub fn parry_mod(&self) -> Option<i8> {
        todo!()
    }

    /// The weapon's Overwhelming value.
    pub fn overwhelming(&self) -> u8 {
        todo!()
    }
}

enum WeaponType<'source> {
    Mundane(BaseWeaponId, MundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, ArtifactWeapon<'source>, Option<u8>),
}

impl<'view, 'source> WeaponType<'source> {
    pub fn id(&self) -> WeaponId {
        match self {
            WeaponType::Mundane(base_id, _) => WeaponId::Mundane(*base_id),
            WeaponType::Artifact(artifact_id, _, _) => WeaponId::Artifact(*artifact_id),
        }
    }

    pub fn is_artifact(&self) -> bool {
        matches!(self, WeaponType::Artifact(_, _, _))
    }

    pub fn is_attuned(&self) -> bool {
        match self {
            WeaponType::Mundane(_, _) => false,
            WeaponType::Artifact(_, _, maybe) => maybe.is_some(),
        }
    }

    pub fn is_equipped(&self) -> Option<Equipped> {
        match self {
            WeaponType::Mundane(_, mundane) => mundane.is_equipped(),
            WeaponType::Artifact(_, artifact, _) => artifact.is_equipped(),
        }
    }

    pub fn mote_commitment(&self) -> Option<(ArtifactWeaponId, MoteCommitment<'source>)> {
        match self {
            WeaponType::Mundane(_, _) => None,
            WeaponType::Artifact(_, _, _) => todo!(),
        }
    }

    pub fn name(&self) -> &'source str {
        match self {
            WeaponType::Mundane(_, mundane) => mundane.name(),
            WeaponType::Artifact(_, artifact, _) => (*artifact).name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            WeaponType::Mundane(_, mundane) => mundane.book_reference,
            WeaponType::Artifact(_, artifact, _) => (*artifact).book_reference(),
        }
    }

    pub fn lore(&self) -> Option<&'source str> {
        match self {
            WeaponType::Mundane(_, _) => None,
            WeaponType::Artifact(_, artifact, _) => (*artifact).lore(),
        }
    }

    pub fn powers(&self) -> Option<&'source str> {
        match self {
            WeaponType::Mundane(_, _) => None,
            WeaponType::Artifact(_, artifact, _) => (*artifact).powers(),
        }
    }

    pub fn hearthstone_slots(&self) -> u8 {
        match self {
            WeaponType::Mundane(_, _) => 0,
            WeaponType::Artifact(_, artifact, _) => {
                (*artifact).hearthstone_slots().min(u8::MAX as usize) as u8
            }
        }
    }

    pub fn slotted_hearthstones(
        &'view self,
    ) -> impl Iterator<Item = &'view OwnedHearthstone<'source>> {
        match self {
            WeaponType::Mundane(_, _) => Vec::new().into_iter(),
            WeaponType::Artifact(_, artifact, _) => (*artifact)
                .slotted_heathstones()
                .collect::<Vec<&'view OwnedHearthstone>>()
                .into_iter(),
        }
    }

    pub fn base_artifact_weapon(&self) -> Option<(BaseWeaponId, &BaseWeapon<'source>)> {
        match self {
            WeaponType::Mundane(_, _) => None,
            WeaponType::Artifact(_, artifact, _) => Some((
                (*artifact).base_artifact_weapon_id(),
                (*artifact).base_artifact_weapon(),
            )),
        }
    }
}
