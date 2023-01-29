mod add_base;
mod base;
pub(crate) mod builder;
mod grouped;
mod handedness;
mod inner;
mod memo;
mod newtype;
mod name;
pub use name::ArtifactWeaponName;

use std::ops::Deref;

pub use add_base::AddBaseArtifactWeapon;
pub use base::BaseArtifactWeapon;
pub(crate) use grouped::{
    HandlessArtifactWeapon, HandlessArtifactWeaponMemo, HandlessArtifactWeaponNoAttunement,
    HandlessArtifactWeaponNoAttunementMemo, NonnaturalArtifactWeapon, NonnaturalArtifactWeaponMemo,
    NonnaturalArtifactWeaponNoAttunement, NonnaturalArtifactWeaponNoAttunementMemo,
};
pub use memo::AddArtifactWeapon;
pub(crate) use newtype::{
    NaturalArtifactWeaponView, NaturalArtifactWeaponMemo, OneHandedArtifactWeaponView,
    OneHandedArtifactWeaponMemo, TwoHandedArtifactWeaponView, TwoHandedArtifactWeaponMemo,
    WornArtifactWeaponMemo, WornArtifactWeaponView,
};

use self::{inner::ArtifactWeaponInner, handedness::ArtifactWeaponHandedness};

use super::{
    equipped::{EquipHand, Equipped},
    WeaponTag,
};

pub(crate) enum ArtifactWeapon<'source> {
    Natural(NaturalArtifactWeaponView<'source>),
    Worn(WornArtifactWeaponView<'source>, bool),
    OneHanded(OneHandedArtifactWeaponView<'source>, Option<EquipHand>),
    TwoHanded(TwoHandedArtifactWeaponView<'source>, bool),
}

impl<'source> From<&'source ArtifactWeaponHandedness> for ArtifactWeapon<'source> {
    fn from(memo: &'source ArtifactWeaponHandedness) -> Self {
        match memo {
            ArtifactWeaponHandedness::Natural(natural) => Self::Natural(natural.into()),
            ArtifactWeaponHandedness::Worn(worn, is_worn) => Self::Worn(worn.into(), *is_worn),
            ArtifactWeaponHandedness::OneHanded(one_handed, maybe_equipped) => Self::OneHanded(one_handed.into(), *maybe_equipped),
            ArtifactWeaponHandedness::TwoHanded(two_handed, is_equipped) => Self::TwoHanded(two_handed.into(), *is_equipped),
        }
    }
}



impl<'source> Deref for ArtifactWeapon<'source> {
    type Target = ArtifactWeaponInner<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            ArtifactWeapon::Natural(deref) => deref,
            ArtifactWeapon::Worn(deref, _) => deref,
            ArtifactWeapon::OneHanded(deref, _) => deref,
            ArtifactWeapon::TwoHanded(deref, _) => deref,
        }
    }
}

impl<'source> ArtifactWeapon<'source> {
    pub fn lore(&self) -> Option<&'source str> {
        match self {
            ArtifactWeapon::Natural(weapon) => weapon.lore(),
            ArtifactWeapon::Worn(weapon, _) => weapon.lore(),
            ArtifactWeapon::OneHanded(weapon, _) => weapon.lore(),
            ArtifactWeapon::TwoHanded(weapon, _) => weapon.lore(),
        }
    }

    pub fn powers(&self) -> Option<&'source str> {
        match self {
            ArtifactWeapon::Natural(weapon) => weapon.powers(),
            ArtifactWeapon::Worn(weapon, _) => weapon.powers(),
            ArtifactWeapon::OneHanded(weapon, _) => weapon.powers(),
            ArtifactWeapon::TwoHanded(weapon, _) => weapon.powers(),
        }
    }

    pub fn is_equipped(&self) -> Option<Equipped> {
        match self {
            ArtifactWeapon::Natural(_) => Some(Equipped::Natural),
            ArtifactWeapon::Worn(_, is_equipped) => {
                if *is_equipped {
                    Some(Equipped::Worn)
                } else {
                    None
                }
            }
            ArtifactWeapon::OneHanded(_, maybe_hand) => match maybe_hand {
                None => None,
                Some(EquipHand::MainHand) => Some(Equipped::MainHand),
                Some(EquipHand::OffHand) => Some(Equipped::OffHand),
            },
            ArtifactWeapon::TwoHanded(_, is_equipped) => {
                if *is_equipped {
                    Some(Equipped::TwoHanded)
                } else {
                    None
                }
            }
        }
    }

    pub(crate) fn tags(&self) -> std::vec::IntoIter<WeaponTag> {
        match self {
            ArtifactWeapon::Natural(base) => {
                base.base_artifact_weapon().tags(WeaponTag::Natural)
            }
            ArtifactWeapon::Worn(base, _) => base.base_artifact_weapon().tags(WeaponTag::Worn),
            ArtifactWeapon::OneHanded(base, _) => {
                base.base_artifact_weapon().tags(WeaponTag::OneHanded)
            }
            ArtifactWeapon::TwoHanded(base, _) => {
                base.base_artifact_weapon().tags(WeaponTag::TwoHanded)
            }
        }
    }
}
