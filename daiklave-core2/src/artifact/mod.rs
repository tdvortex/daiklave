use serde::{Serialize, Deserialize};

use crate::weapons::{artifact::ArtifactWeaponMemo, ArtifactWeapon};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactMemo {
    Weapon(ArtifactWeaponMemo)
}

impl<'source> ArtifactMemo {
    pub fn as_ref(&'source self) -> Artifact<'source, 'source> {
        todo!()
    }
}

pub enum Artifact<'view, 'source> {
    Weapon(ArtifactWeapon<'view, 'source>)
}

impl<'view, 'source> Artifact<'view, 'source> {
    pub fn as_memo(&self) -> ArtifactMemo {
        todo!()
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