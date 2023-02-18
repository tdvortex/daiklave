use serde::{Deserialize, Serialize};

use crate::weapons::weapon::mundane::{
    OneHandedMundaneWeaponMemo, TwoHandedMundaneWeapon, WornMundaneWeapon,
};

use super::NonnaturalMundaneWeapon;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonnaturalMundaneWeaponMemo {
    Worn(WornMundaneWeapon),
    OneHanded(OneHandedMundaneWeaponMemo),
    TwoHanded(TwoHandedMundaneWeapon),
}

impl From<&NonnaturalMundaneWeapon<'_>> for NonnaturalMundaneWeaponMemo {
    fn from(value: &NonnaturalMundaneWeapon<'_>) -> Self {
        match value {
            NonnaturalMundaneWeapon::Worn(weapon) => Self::Worn(weapon.into()),
            NonnaturalMundaneWeapon::OneHanded(weapon) => Self::OneHanded(weapon.into()),
            NonnaturalMundaneWeapon::TwoHanded(weapon) => Self::TwoHanded(weapon.into()),
        }
    }
}
