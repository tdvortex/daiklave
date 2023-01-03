use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::exaltation::exalt::essence::MoteCommitment;

use super::{natural::{NaturalArtifactWeapon, NaturalArtifactWeaponMemo}, worn::{WornArtifactWeapon, WornArtifactWeaponMemo}, named::NamedArtifactWeapon};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) struct HandlessArtifactWeapon<'source>(pub HandlessArtifactWeaponNoAttunement<'source>, pub Option<u8>);

impl<'source> HandlessArtifactWeapon<'source> {
    pub fn as_memo(&self) -> HandlessArtifactWeaponMemo {
        HandlessArtifactWeaponMemo(self.0.as_memo(), self.1)
    }

    pub fn is_attuned(&self) -> bool {
        self.1.is_some()
    }

    pub fn motes_committed(&self) -> Option<MoteCommitment<'source>> {
        self.1.map(|personal_committed| {
            MoteCommitment{
                name: self.0.name(),
                peripheral: 5 - 5.min(personal_committed),
                personal: 5.min(personal_committed),
            }
        })
    }
}

impl<'source> Deref for HandlessArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) enum HandlessArtifactWeaponNoAttunement<'source> {
    Natural(NaturalArtifactWeapon<'source>),
    Worn(WornArtifactWeapon<'source>),
}

impl<'source> HandlessArtifactWeaponNoAttunement<'source> {
    pub fn as_memo(&self) -> HandlessArtifactWeaponNoAttunementMemo {
        match self {
            HandlessArtifactWeaponNoAttunement::Natural(view) => HandlessArtifactWeaponNoAttunementMemo::Natural(view.as_memo()),
            HandlessArtifactWeaponNoAttunement::Worn(view) => HandlessArtifactWeaponNoAttunementMemo::Worn(view.as_memo()),
        }
    }
}

impl<'source> Deref for HandlessArtifactWeaponNoAttunement<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            HandlessArtifactWeaponNoAttunement::Natural(deref) => deref,
            HandlessArtifactWeaponNoAttunement::Worn(deref) => deref,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) struct HandlessArtifactWeaponMemo(HandlessArtifactWeaponNoAttunementMemo, Option<u8>);

impl<'source> HandlessArtifactWeaponMemo {
    pub fn as_ref(&'source self) -> HandlessArtifactWeapon<'source> {
        HandlessArtifactWeapon(self.0.as_ref(), self.1)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) enum HandlessArtifactWeaponNoAttunementMemo {
    Natural(NaturalArtifactWeaponMemo),
    Worn(WornArtifactWeaponMemo),
}

impl<'source> HandlessArtifactWeaponNoAttunementMemo {
    pub fn as_ref(&'source self) -> HandlessArtifactWeaponNoAttunement<'source> {
        match self {
            HandlessArtifactWeaponNoAttunementMemo::Natural(memo) => HandlessArtifactWeaponNoAttunement::Natural(memo.as_ref()),
            HandlessArtifactWeaponNoAttunementMemo::Worn(memo) => HandlessArtifactWeaponNoAttunement::Worn(memo.as_ref()),
        }
    }
}

