mod ability;
mod artifact;
mod base;
mod damage_type;
mod equipped;
mod exalt;
mod hearthstone;
mod mortal;
mod mundane;
mod range;
mod tag;
mod unarmed;
mod weapon_id;
mod weight_class;

pub use weapon_id::{BaseWeaponId, ArtifactWeaponId, ArtifactId};

use crate::{exaltation::{Exaltation, exalt::essence::MoteCommitment}, book_reference::BookReference};

use self::{weapon_id::WeaponId, artifact::{ArtifactWeapon}, mundane::MundaneWeapon, hearthstone::{OwnedHearthstone}, base::BaseWeapon};
pub use weight_class::WeaponWeightClass;

pub struct Weapons<'view, 'source>(&'view Exaltation<'source>);

impl<'view, 'source> Weapons<'view, 'source> {
    pub fn get(&self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        self.0.get_weapon(weapon_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = WeaponId> + '_ {
        self.0.iter_weapons()
    }
}


pub struct Weapon<'source>(WeaponType<'source>);

impl<'view, 'source> Weapon<'source> {
    pub fn id(&self) -> WeaponId {
        self.0.id()
    }

    pub fn is_artifact(&self) -> bool {
        self.0.is_artifact()
    }

    pub fn is_attuned(&self) -> bool {
        self.0.is_attuned()
    }

    pub fn mote_commitment(&self) -> Option<(ArtifactWeaponId, MoteCommitment<'source>)> {
        self.0.mote_commitment()
    }

    pub fn name(&self) -> &'source str {
        self.0.name()
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        self.0.book_reference()
    }

    pub fn lore(&self) -> Option<&'source str> {
        self.0.lore()
    }

    pub fn powers(&self) -> Option<&'source str> {
        self.0.powers()
    }

    pub fn hearthstone_slots(&self) -> u8 {
        self.0.hearthstone_slots()
    }

    pub fn slotted_heathstones(&'view self) -> impl Iterator<Item = &'view OwnedHearthstone<'source>> + '_ {
        self.0.slotted_hearthstones()
    }

    pub fn base_artifact_weapon(&self) -> Option<(BaseWeaponId, BaseWeapon<'source>)> {
        self.0.base_artifact_weapon()
    }
}

enum WeaponType<'source> {
    Mundane(BaseWeaponId, MundaneWeapon<'source>),
    Artifact(ArtifactWeaponId, ArtifactWeapon<'source>, Option<u8>),
}

impl<'view, 'source> WeaponType<'source> {
    pub fn id(&self) -> WeaponId {
        match self {
            WeaponType::Mundane(base_id, _) => WeaponId::NonArtifact(*base_id),
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
            WeaponType::Artifact(_, artifact, _) => (*artifact).hearthstone_slots().min(u8::MAX as usize) as u8,
        }
    }

    pub fn slotted_hearthstones(&'view self) -> impl Iterator<Item = &'view OwnedHearthstone<'source>> {
        match self {
            WeaponType::Mundane(_, _) => Vec::new().into_iter(),
            WeaponType::Artifact(_, artifact, _) => (*artifact).slotted_heathstones().collect::<Vec<&'view OwnedHearthstone>>().into_iter(),
        }
    }

    pub fn base_artifact_weapon(&self) -> Option<(BaseWeaponId, BaseWeapon<'source>)> {
        match self {
            WeaponType::Mundane(_, _) => None,
            WeaponType::Artifact(_, artifact, _) => Some(((*artifact).base_artifact_weapon_id(), (*artifact).base_artifact_weapon()))
        }
    }
}

