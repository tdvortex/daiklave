use serde::{Deserialize, Serialize};

use crate::weapons::weapon::{artifact::ArtifactWeaponName, mundane::MundaneWeaponName};

use super::WeaponName;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub(crate) enum WeaponNameMutation {
    Unarmed,
    Mundane(MundaneWeaponName),
    Artifact(ArtifactWeaponName),
}

impl From<WeaponName<'_>> for WeaponNameMutation {
    fn from(name: WeaponName) -> Self {
        match name {
            WeaponName::Unarmed => todo!(),
            WeaponName::Mundane(name) => Self::Mundane(name.into()),
            WeaponName::Artifact(name) => Self::Artifact(name.into()),
        }
    }
}

#[allow(clippy::from_over_into)]
impl<'source> Into<WeaponName<'source>> for &'source WeaponNameMutation {
    fn into(self) -> WeaponName<'source> {
        match self {
            WeaponNameMutation::Unarmed => WeaponName::Unarmed,
            WeaponNameMutation::Mundane(name) => WeaponName::Mundane(name.as_str()),
            WeaponNameMutation::Artifact(name) => WeaponName::Artifact(name.as_str()),
        }
    }
}

impl From<MundaneWeaponName> for WeaponNameMutation {
    fn from(mundane_name: MundaneWeaponName) -> Self {
        Self::Mundane(mundane_name)
    }
}

impl From<ArtifactWeaponName> for WeaponNameMutation {
    fn from(artifact_name: ArtifactWeaponName) -> Self {
        Self::Artifact(artifact_name)
    }
}
