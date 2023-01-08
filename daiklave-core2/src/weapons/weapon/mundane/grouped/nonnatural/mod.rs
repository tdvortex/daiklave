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
