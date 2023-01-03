use serde::{Serialize, Deserialize};

use super::{worn::{WornMundaneWeapon, WornMundaneWeaponMemo}, natural::{NaturalMundaneWeapon, NaturalMundaneWeaponMemo}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) enum HandlessMundaneWeapon<'source> {
    Natural(NaturalMundaneWeapon<'source>),
    Worn(WornMundaneWeapon<'source>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) enum HandlessMundaneWeaponMemo {
    Natural(NaturalMundaneWeaponMemo),
    Worn(WornMundaneWeaponMemo),
}

impl<'source> HandlessMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> HandlessMundaneWeapon<'source> {
        match self {
            HandlessMundaneWeaponMemo::Natural(memo) => HandlessMundaneWeapon::Natural(memo.as_ref()),
            HandlessMundaneWeaponMemo::Worn(memo) => HandlessMundaneWeapon::Worn(memo.as_ref()),
        }
    }
}