use serde::{Deserialize, Serialize};

use crate::weapons::weapon::mundane::{NaturalMundaneWeapon, WornMundaneWeapon};

use super::HandlessMundaneWeapon;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum HandlessMundaneWeaponMemo {
    Natural(NaturalMundaneWeapon),
    Worn(WornMundaneWeapon),
}

impl From<&HandlessMundaneWeapon<'_>> for HandlessMundaneWeaponMemo {
    fn from(handless: &HandlessMundaneWeapon<'_>) -> Self {
        match handless {
            HandlessMundaneWeapon::Natural(weapon) => Self::Natural(weapon.into()),
            HandlessMundaneWeapon::Worn(weapon) => Self::Worn(weapon.into()),
        }
    }
}