use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::weapons::{BaseWeaponId, ArtifactWeaponId, mundane::{NonnaturalMundaneWeapon, NonnaturalMundaneWeaponMemo, MundaneWeapon}, artifact::{NonnaturalArtifactWeapon, NonnaturalArtifactWeaponMemo}, mortal::MortalUnequippedWeapons, WeaponId, Weapon, WeaponType};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::weapons) struct ExaltUnequippedWeapons<'source> {
    pub mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeapon<'source>>,
    pub artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeapon<'source>>,
}

impl<'source> From<MortalUnequippedWeapons<'source>> for ExaltUnequippedWeapons<'source> {
    fn from(mortal: MortalUnequippedWeapons<'source>) -> Self {
        Self {
            mundane: mortal.mundane,
            artifact: mortal.artifact.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl<'source> ExaltUnequippedWeapons<'source> {
    pub fn as_memo(&self) -> ExaltUnequippedWeaponsMemo {
        ExaltUnequippedWeaponsMemo {
            mundane: self.mundane.iter().map(|(k, v)| (*k, v.as_memo())).collect(),
            artifact: self.artifact.iter().map(|(k, v)| (*k, v.as_memo())).collect(),
        }
    }

    pub fn get_weapon(&self, weapon_id: WeaponId) -> Option<Weapon<'source>> {
        match weapon_id {
            WeaponId::Unarmed =>Some(crate::weapons::unarmed()),
            WeaponId::Mundane(target_id) => {
                match self.mundane.get(&target_id)? {
                    NonnaturalMundaneWeapon::Worn(worn_weapon) => {
                        Some(Weapon(WeaponType::Mundane(target_id, MundaneWeapon::Worn(*worn_weapon, false))))
                    }
                    NonnaturalMundaneWeapon::OneHanded(one) => {
                        Some(Weapon(WeaponType::Mundane(target_id, MundaneWeapon::OneHanded(*one, None))))
                    }
                    NonnaturalMundaneWeapon::TwoHanded(two) => {
                        Some(Weapon(WeaponType::Mundane(target_id, MundaneWeapon::TwoHanded(*two, false))))
                    }
                }
            }
            WeaponId::Artifact(target_id) => {
                let nonnatural_artifact_weapon = self.artifact.get(&target_id)?;
                let (without_attunement, attunement) = (nonnatural_artifact_weapon.0, nonnatural_artifact_weapon.1);

                None
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons::exalt) struct ExaltUnequippedWeaponsMemo {
    mundane: HashMap<BaseWeaponId, NonnaturalMundaneWeaponMemo>,
    artifact: HashMap<ArtifactWeaponId, NonnaturalArtifactWeaponMemo>,
}

impl<'source> ExaltUnequippedWeaponsMemo {
    pub fn as_ref(&'source self) -> ExaltUnequippedWeapons<'source> {
        ExaltUnequippedWeapons {
            mundane: self.mundane.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
            artifact: self.artifact.iter().map(|(k, v)| (*k, v.as_ref())).collect(),
        }
    }
}