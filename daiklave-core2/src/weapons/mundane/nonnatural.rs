use serde::{Serialize, Deserialize};

use super::{worn::WornMundaneWeapon, one_handed::OneHandedMundaneWeapon, two_handed::TwoHandedMundaneWeapon, WornMundaneWeaponMemo, OneHandedMundaneWeaponMemo, TwoHandedMundaneWeaponMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::weapons) enum NonnaturalMundaneWeapon<'source> {
    Worn(WornMundaneWeapon<'source>),
    OneHanded(OneHandedMundaneWeapon<'source>),
    TwoHanded(TwoHandedMundaneWeapon<'source>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::weapons) enum NonnaturalMundaneWeaponMemo {
    Worn(WornMundaneWeaponMemo),
    OneHanded(OneHandedMundaneWeaponMemo),
    TwoHanded(TwoHandedMundaneWeaponMemo),
}