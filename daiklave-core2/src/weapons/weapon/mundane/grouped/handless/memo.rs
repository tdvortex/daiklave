use serde::{Deserialize, Serialize};

use crate::weapons::weapon::mundane::{NaturalMundaneWeaponMemo, WornMundaneWeaponMemo};

use super::HandlessMundaneWeapon;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum HandlessMundaneWeaponMemo {
    Natural(NaturalMundaneWeaponMemo),
    Worn(WornMundaneWeaponMemo),
}

impl<'source> HandlessMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> HandlessMundaneWeapon<'source> {
        match self {
            HandlessMundaneWeaponMemo::Natural(memo) => {
                HandlessMundaneWeapon::Natural(memo.as_ref())
            }
            HandlessMundaneWeaponMemo::Worn(memo) => HandlessMundaneWeapon::Worn(memo.as_ref()),
        }
    }
}
