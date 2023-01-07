use crate::weapons::weapon::{artifact::ArtifactWeaponMemo, ArtifactWeaponId};

use super::Artifact;

/// An owned copy of an Artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactMemo {
    /// An artifact weapon.
    Weapon(ArtifactWeaponId, ArtifactWeaponMemo),
}

impl<'source> ArtifactMemo {
    /// Uses the Artifact as a source and copies all Copy values and derefs
    /// String to &'source str
    pub fn as_ref(&'source self) -> Artifact<'source> {
        match self {
            ArtifactMemo::Weapon(id, memo) => Artifact::Weapon(*id, memo.as_ref()),
        }
    }
}
