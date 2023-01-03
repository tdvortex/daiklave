use std::ops::Deref;

use serde::{Serialize, Deserialize};

use super::{worn::{WornArtifactWeapon, WornArtifactWeaponMemo}, one_handed::OneHandedArtifactWeapon, two_handed::{TwoHandedArtifactWeapon, TwoHandedArtifactWeaponMemo}, named::NamedArtifactWeapon, OneHandedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) struct NonnaturalArtifactWeapon<'source>(NonnaturalArtifactWeaponNoAttunement<'source>, Option<u8>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) enum NonnaturalArtifactWeaponNoAttunement<'source> {
    Worn(WornArtifactWeapon<'source>),
    OneHanded(OneHandedArtifactWeapon<'source>),
    TwoHanded(TwoHandedArtifactWeapon<'source>),
}

impl<'source> Deref for NonnaturalArtifactWeaponNoAttunement<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            NonnaturalArtifactWeaponNoAttunement::Worn(deref) => deref,
            NonnaturalArtifactWeaponNoAttunement::OneHanded(deref) => deref,
            NonnaturalArtifactWeaponNoAttunement::TwoHanded(deref) => deref,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) enum NonnaturalArtifactWeaponNoAttunementMemo {
    Worn(WornArtifactWeaponMemo),
    OneHanded(OneHandedArtifactWeaponMemo),
    TwoHanded(TwoHandedArtifactWeaponMemo),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) struct NonnaturalArtifactWeaponMemo(NonnaturalArtifactWeaponNoAttunementMemo, Option<u8>);