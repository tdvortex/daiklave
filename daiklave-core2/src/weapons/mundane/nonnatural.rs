use serde::{Deserialize, Serialize};

use super::{
    one_handed::OneHandedMundaneWeapon, two_handed::TwoHandedMundaneWeapon,
    worn::WornMundaneWeapon, OneHandedMundaneWeaponMemo, TwoHandedMundaneWeaponMemo,
    WornMundaneWeaponMemo,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NonnaturalMundaneWeapon<'source> {
    Worn(WornMundaneWeapon<'source>),
    OneHanded(OneHandedMundaneWeapon<'source>),
    TwoHanded(TwoHandedMundaneWeapon<'source>),
}

impl<'source> NonnaturalMundaneWeapon<'source> {
    pub fn as_memo(&self) -> NonnaturalMundaneWeaponMemo {
        match self {
            NonnaturalMundaneWeapon::Worn(view) => {
                NonnaturalMundaneWeaponMemo::Worn(view.as_memo())
            }
            NonnaturalMundaneWeapon::OneHanded(view) => {
                NonnaturalMundaneWeaponMemo::OneHanded(view.as_memo())
            }
            NonnaturalMundaneWeapon::TwoHanded(view) => {
                NonnaturalMundaneWeaponMemo::TwoHanded(view.as_memo())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum NonnaturalMundaneWeaponMemo {
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
