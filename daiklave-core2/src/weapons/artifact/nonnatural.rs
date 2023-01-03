use std::ops::Deref;

use serde::{Serialize, Deserialize};

use super::{worn::{WornArtifactWeapon, WornArtifactWeaponMemo}, one_handed::OneHandedArtifactWeapon, two_handed::{TwoHandedArtifactWeapon, TwoHandedArtifactWeaponMemo}, named::NamedArtifactWeapon, OneHandedArtifactWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) struct NonnaturalArtifactWeapon<'source>(pub NonnaturalArtifactWeaponNoAttunement<'source>, Option<u8>);

impl<'source> NonnaturalArtifactWeapon<'source> {
    pub fn as_memo(&self) -> NonnaturalArtifactWeaponMemo {
        NonnaturalArtifactWeaponMemo(self.0.as_memo(), self.1)
    }
}

impl<'source> From<NonnaturalArtifactWeaponNoAttunement<'source>> for NonnaturalArtifactWeapon<'source> {
    fn from(unattuned: NonnaturalArtifactWeaponNoAttunement<'source>) -> Self {
        Self(unattuned, None)
    }
}


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

impl<'source> NonnaturalArtifactWeaponNoAttunement<'source> {
    pub fn as_memo(&self) -> NonnaturalArtifactWeaponNoAttunementMemo {
        match self {
            NonnaturalArtifactWeaponNoAttunement::Worn(view) => NonnaturalArtifactWeaponNoAttunementMemo::Worn(view.as_memo()),
            NonnaturalArtifactWeaponNoAttunement::OneHanded(view) => NonnaturalArtifactWeaponNoAttunementMemo::OneHanded(view.as_memo()),
            NonnaturalArtifactWeaponNoAttunement::TwoHanded(view) => NonnaturalArtifactWeaponNoAttunementMemo::TwoHanded(view.as_memo()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) enum NonnaturalArtifactWeaponNoAttunementMemo {
    Worn(WornArtifactWeaponMemo),
    OneHanded(OneHandedArtifactWeaponMemo),
    TwoHanded(TwoHandedArtifactWeaponMemo),
}

impl<'source> NonnaturalArtifactWeaponNoAttunementMemo {
    pub fn as_ref(&'source self) -> NonnaturalArtifactWeaponNoAttunement<'source> {
        match self {
            NonnaturalArtifactWeaponNoAttunementMemo::Worn(memo) => NonnaturalArtifactWeaponNoAttunement::Worn(memo.as_ref()),
            NonnaturalArtifactWeaponNoAttunementMemo::OneHanded(memo) => NonnaturalArtifactWeaponNoAttunement::OneHanded(memo.as_ref()),
            NonnaturalArtifactWeaponNoAttunementMemo::TwoHanded(memo) => NonnaturalArtifactWeaponNoAttunement::TwoHanded(memo.as_ref()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) struct NonnaturalArtifactWeaponMemo(NonnaturalArtifactWeaponNoAttunementMemo, Option<u8>);

impl<'source> NonnaturalArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> NonnaturalArtifactWeapon<'source> {
        NonnaturalArtifactWeapon(self.0.as_ref(), self.1)
    }
}