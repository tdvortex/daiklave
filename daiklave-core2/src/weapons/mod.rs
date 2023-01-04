mod ability;
mod artifact;
mod base;
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

pub use weapon_id::{ArtifactId, ArtifactWeaponId, BaseWeaponId, WeaponId};

use crate::{
    book_reference::BookReference,
    exaltation::{exalt::essence::MoteCommitment, Exaltation},
};

use self::{
    artifact::ArtifactWeapon, base::BaseWeapon, hearthstone::OwnedHearthstone,
    mundane::MundaneWeapon,
};
pub(crate) use unarmed::unarmed;
pub use weight_class::WeaponWeightClass;

/// The interface for a character's weapons.
pub struct Weapons<'view, 'source>(pub(crate) &'view Exaltation<'source>);

impl<'view, 'source> Weapons<'view, 'source> {
    /// Retrieves the details for a specific weapon, if it exists.
    pub fn get(&self, weapon_id: WeaponId) -> Option<Weapon<'view, 'source>> {
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
pub enum EquipHand {
    /// Wielded in the main hand
    MainHand,
    /// Wielded in the off hand
    OffHand,
}

/// The interface for a specific individual weapon
pub struct Weapon<'view, 'source>(WeaponType<'view, 'source>);

impl<'view, 'source> Weapon<'view, 'source> {
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
    pub fn base_artifact_weapon(&self) -> Option<(BaseWeaponId, BaseWeapon<'source>)> {
        self.0.base_artifact_weapon()
    }
}

enum WeaponType<'view, 'source> {
    Mundane(BaseWeaponId, MundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, ArtifactWeapon<'view, 'source>, Option<u8>),
}

impl<'view, 'source> WeaponType<'view, 'source> {
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
            WeaponType::Mundane(_, mundane) => (*mundane).name(),
            WeaponType::Artifact(_, artifact, _) => (*artifact).name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            WeaponType::Mundane(_, mundane) => (*mundane).book_reference(),
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

    pub fn base_artifact_weapon(&self) -> Option<(BaseWeaponId, BaseWeapon<'source>)> {
        match self {
            WeaponType::Mundane(_, _) => None,
            WeaponType::Artifact(_, artifact, _) => Some((
                (*artifact).base_artifact_weapon_id(),
                (*artifact).base_artifact_weapon(),
            )),
        }
    }
}
