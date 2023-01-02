use std::ops::Deref;

use crate::exaltation::exalt::essence::MoteCommitment;

use super::{natural::NaturalArtifactWeapon, worn::WornArtifactWeapon, named::NamedArtifactWeapon};

pub(in crate::weapons) struct HandlessArtifactWeapon<'source>(HandlessArtifactWeaponNoAttunement<'source>, Option<u8>);

impl<'source> HandlessArtifactWeapon<'source> {
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


pub(in crate::weapons) enum HandlessArtifactWeaponNoAttunement<'source> {
    Natural(NaturalArtifactWeapon<'source>),
    Worn(WornArtifactWeapon<'source>),
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