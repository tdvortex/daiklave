mod memo;
pub use memo::NonnaturalMundaneWeaponMemo;

use crate::weapons::weapon::mundane::{
    OneHandedMundaneWeapon, TwoHandedMundaneWeapon, WornMundaneWeapon,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NonnaturalMundaneWeapon<'source> {
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
