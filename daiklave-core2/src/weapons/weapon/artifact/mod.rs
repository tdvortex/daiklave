mod add;
mod base;
pub(crate) mod builder;
mod grouped;
mod inner;
mod memo;
mod newtype;
mod name;
pub use name::ArtifactWeaponName;

use std::ops::Deref;

pub use add::AddBaseArtifactWeapon;
pub use base::BaseArtifactWeapon;
pub(crate) use grouped::{
    HandlessArtifactWeapon, HandlessArtifactWeaponMemo, HandlessArtifactWeaponNoAttunement,
    HandlessArtifactWeaponNoAttunementMemo, NonnaturalArtifactWeapon, NonnaturalArtifactWeaponMemo,
    NonnaturalArtifactWeaponNoAttunement, NonnaturalArtifactWeaponNoAttunementMemo,
};
pub use memo::ArtifactWeapon;
pub(crate) use newtype::{
    NaturalArtifactWeapon, NaturalArtifactWeaponView, OneHandedArtifactWeapon,
    OneHandedArtifactWeaponView, TwoHandedArtifactWeapon, TwoHandedArtifactWeaponView,
    WornArtifactWeapon, WornArtifactWeaponView,
};

use self::inner::ArtifactWeaponInner;

use super::{
    equipped::{EquipHand, Equipped},
    WeaponTag,
};

pub(crate) enum ArtifactWeaponView<'source> {
    Natural(NaturalArtifactWeaponView<'source>),
    Worn(WornArtifactWeaponView<'source>, bool),
    OneHanded(OneHandedArtifactWeaponView<'source>, Option<EquipHand>),
    TwoHanded(TwoHandedArtifactWeaponView<'source>, bool),
}

impl<'source> Deref for ArtifactWeaponView<'source> {
    type Target = ArtifactWeaponInner<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            ArtifactWeaponView::Natural(deref) => deref,
            ArtifactWeaponView::Worn(deref, _) => deref,
            ArtifactWeaponView::OneHanded(deref, _) => deref,
            ArtifactWeaponView::TwoHanded(deref, _) => deref,
        }
    }
}

impl<'source> ArtifactWeaponView<'source> {
    pub fn lore(&self) -> Option<&'source str> {
        match self {
            ArtifactWeaponView::Natural(weapon) => weapon.lore(),
            ArtifactWeaponView::Worn(weapon, _) => weapon.lore(),
            ArtifactWeaponView::OneHanded(weapon, _) => weapon.lore(),
            ArtifactWeaponView::TwoHanded(weapon, _) => weapon.lore(),
        }
    }

    pub fn powers(&self) -> Option<&'source str> {
        match self {
            ArtifactWeaponView::Natural(weapon) => weapon.powers(),
            ArtifactWeaponView::Worn(weapon, _) => weapon.powers(),
            ArtifactWeaponView::OneHanded(weapon, _) => weapon.powers(),
            ArtifactWeaponView::TwoHanded(weapon, _) => weapon.powers(),
        }
    }

    pub fn is_equipped(&self) -> Option<Equipped> {
        match self {
            ArtifactWeaponView::Natural(_) => Some(Equipped::Natural),
            ArtifactWeaponView::Worn(_, is_equipped) => {
                if *is_equipped {
                    Some(Equipped::Worn)
                } else {
                    None
                }
            }
            ArtifactWeaponView::OneHanded(_, maybe_hand) => match maybe_hand {
                None => None,
                Some(EquipHand::MainHand) => Some(Equipped::MainHand),
                Some(EquipHand::OffHand) => Some(Equipped::OffHand),
            },
            ArtifactWeaponView::TwoHanded(_, is_equipped) => {
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
            ArtifactWeaponView::Natural(base) => {
                base.base_artifact_weapon().tags(WeaponTag::Natural)
            }
            ArtifactWeaponView::Worn(base, _) => base.base_artifact_weapon().tags(WeaponTag::Worn),
            ArtifactWeaponView::OneHanded(base, _) => {
                base.base_artifact_weapon().tags(WeaponTag::OneHanded)
            }
            ArtifactWeaponView::TwoHanded(base, _) => {
                base.base_artifact_weapon().tags(WeaponTag::TwoHanded)
            }
        }
    }
}
