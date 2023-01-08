use serde::{Deserialize, Serialize};

use crate::weapons::weapon::mundane::{NaturalMundaneWeapon, WornMundaneWeapon};

use super::HandlessMundaneWeapon;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum HandlessMundaneWeaponMemo {
    Natural(NaturalMundaneWeapon),
    Worn(WornMundaneWeapon),
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
