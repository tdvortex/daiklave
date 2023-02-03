mod memo;
pub use memo::NonnaturalMundaneWeaponMemo;

use crate::weapons::weapon::mundane::{
    OneHandedMundaneWeaponView, TwoHandedMundaneWeaponView, WornMundaneWeaponView,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NonnaturalMundaneWeapon<'source> {
    Worn(WornMundaneWeaponView<'source>),
    OneHanded(OneHandedMundaneWeaponView<'source>),
    TwoHanded(TwoHandedMundaneWeaponView<'source>),
}
