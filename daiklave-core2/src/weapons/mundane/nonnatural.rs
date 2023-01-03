use serde::{Serialize, Deserialize};

use super::{worn::WornMundaneWeapon, one_handed::OneHandedMundaneWeapon, two_handed::TwoHandedMundaneWeapon, WornMundaneWeaponMemo, OneHandedMundaneWeaponMemo, TwoHandedMundaneWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) enum NonnaturalMundaneWeapon<'source> {
    Worn(WornMundaneWeapon<'source>),
    OneHanded(OneHandedMundaneWeapon<'source>),
    TwoHanded(TwoHandedMundaneWeapon<'source>),
}

impl<'source> NonnaturalMundaneWeapon<'source> {
    pub fn as_memo(&self) -> NonnaturalMundaneWeaponMemo {
        match self {
            NonnaturalMundaneWeapon::Worn(view) => NonnaturalMundaneWeaponMemo::Worn(view.as_memo()),
            NonnaturalMundaneWeapon::OneHanded(view) => NonnaturalMundaneWeaponMemo::OneHanded(view.as_memo()),
            NonnaturalMundaneWeapon::TwoHanded(view) => NonnaturalMundaneWeaponMemo::TwoHanded(view.as_memo())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) enum NonnaturalMundaneWeaponMemo {
    Worn(WornMundaneWeaponMemo),
    OneHanded(OneHandedMundaneWeaponMemo),
    TwoHanded(TwoHandedMundaneWeaponMemo),
}

impl<'source> NonnaturalMundaneWeaponMemo {
    pub fn as_ref(&'source self) -> NonnaturalMundaneWeapon<'source> {
        match self {
            NonnaturalMundaneWeaponMemo::Worn(memo) => NonnaturalMundaneWeapon::Worn(memo.as_ref()),
            NonnaturalMundaneWeaponMemo::OneHanded(memo) => NonnaturalMundaneWeapon::OneHanded(memo.as_ref()),
            NonnaturalMundaneWeaponMemo::TwoHanded(memo) => NonnaturalMundaneWeapon::TwoHanded(memo.as_ref()),
        }
    }
}