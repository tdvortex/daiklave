mod memo;
pub(crate) use memo::HandlessMundaneWeaponMemo;

use crate::weapons::weapon::mundane::{NaturalMundaneWeaponView, WornMundaneWeaponView};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HandlessMundaneWeapon<'source> {
    Natural(NaturalMundaneWeaponView<'source>),
    Worn(WornMundaneWeaponView<'source>),
}