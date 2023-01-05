use serde::{Serialize, Deserialize};

use crate::weapons::{artifact::ArtifactWeaponMemo, ArtifactWeapon};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactMemo {
    Weapon(ArtifactWeaponMemo)
}

impl<'source> ArtifactMemo {
    pub fn as_ref(&'source self) -> Artifact<'source> {
        match self {
            ArtifactMemo::Weapon(memo) => Artifact::Weapon(memo.as_ref()),
        }
    }
}

pub enum Artifact<'source> {
    Weapon(ArtifactWeapon<'source>)
}

impl<'source> Artifact<'source> {
    pub fn as_memo(&self) -> ArtifactMemo {
        match self {
            Artifact::Weapon(view) => ArtifactMemo::Weapon(view.as_memo())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum MagicMaterial {
    Orichalcum,
    Starmetal,
    Soulsteel,
    Moonsilver,
    RedJade,
    BlueJade,
    GreenJade,
    BlackJade,
    WhiteJade,
}