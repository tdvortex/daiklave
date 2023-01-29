use crate::weapons::weapon::EquipHand;

use super::{NaturalArtifactWeaponMemo, WornArtifactWeaponMemo, OneHandedArtifactWeaponMemo, TwoHandedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ArtifactWeaponHandedness {
    Natural(NaturalArtifactWeaponMemo),
    Worn(WornArtifactWeaponMemo, bool),
    OneHanded(OneHandedArtifactWeaponMemo, Option<EquipHand>),
    TwoHanded(TwoHandedArtifactWeaponMemo, bool),
}