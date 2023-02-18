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

impl<'source> From<&'source NonnaturalMundaneWeaponMemo> for NonnaturalMundaneWeapon<'source> {
    fn from(value: &'source NonnaturalMundaneWeaponMemo) -> Self {
        match value {
            NonnaturalMundaneWeaponMemo::Worn(weapon) => Self::Worn(weapon.into()),
            NonnaturalMundaneWeaponMemo::OneHanded(weapon) => Self::OneHanded(weapon.into()),
            NonnaturalMundaneWeaponMemo::TwoHanded(weapon) => Self::TwoHanded(weapon.into()),
        }
    }
}