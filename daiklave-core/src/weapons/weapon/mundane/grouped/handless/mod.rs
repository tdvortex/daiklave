mod memo;
pub(crate) use memo::HandlessMundaneWeaponMemo;

use crate::weapons::weapon::mundane::{NaturalMundaneWeaponView, WornMundaneWeaponView};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HandlessMundaneWeapon<'source> {
    Natural(NaturalMundaneWeaponView<'source>),
    Worn(WornMundaneWeaponView<'source>),
}

impl<'source> From<&'source HandlessMundaneWeaponMemo> for HandlessMundaneWeapon<'source> {
    fn from(value: &'source HandlessMundaneWeaponMemo) -> Self {
        match value {
            HandlessMundaneWeaponMemo::Natural(weapon) => Self::Natural(weapon.into()),
            HandlessMundaneWeaponMemo::Worn(weapon) => Self::Worn(weapon.into()),
        }
    }
}