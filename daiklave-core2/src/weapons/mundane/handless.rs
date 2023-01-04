use serde::{Deserialize, Serialize};

use super::{
    natural::{NaturalMundaneWeapon, NaturalMundaneWeaponMemo},
    worn::{WornMundaneWeapon, WornMundaneWeaponMemo},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HandlessMundaneWeapon<'source> {
    Natural(NaturalMundaneWeapon<'source>),
    Worn(WornMundaneWeapon<'source>),
}

impl<'source> HandlessMundaneWeapon<'source> {
    pub fn as_memo(&self) -> HandlessMundaneWeaponMemo {
        match self {
            HandlessMundaneWeapon::Natural(view) => {
                HandlessMundaneWeaponMemo::Natural(view.as_memo())
            }
            HandlessMundaneWeapon::Worn(view) => HandlessMundaneWeaponMemo::Worn(view.as_memo()),
        }
    }
}

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
