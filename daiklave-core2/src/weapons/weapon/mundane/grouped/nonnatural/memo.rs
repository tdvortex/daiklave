use serde::{Deserialize, Serialize};

use crate::weapons::weapon::mundane::{
    OneHandedMundaneWeaponMemo, TwoHandedMundaneWeaponMemo, WornMundaneWeaponMemo,
};

use super::NonnaturalMundaneWeapon;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonnaturalMundaneWeaponMemo {
    Worn(WornMundaneWeaponMemo),
    OneHanded(OneHandedMundaneWeaponMemo),
    TwoHanded(TwoHandedMundaneWeaponMemo),
}

impl<'source> NonnaturalMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> NonnaturalMundaneWeapon<'source> {
        match self {
            NonnaturalMundaneWeaponMemo::Worn(memo) => NonnaturalMundaneWeapon::Worn(memo.as_ref()),
            NonnaturalMundaneWeaponMemo::OneHanded(memo) => {
                NonnaturalMundaneWeapon::OneHanded(memo.as_ref())
            }
            NonnaturalMundaneWeaponMemo::TwoHanded(memo) => {
                NonnaturalMundaneWeapon::TwoHanded(memo.as_ref())
            }
        }
    }
}
