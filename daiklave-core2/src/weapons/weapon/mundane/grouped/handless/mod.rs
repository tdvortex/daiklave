mod memo;
pub(crate) use memo::HandlessMundaneWeaponMemo;

use crate::weapons::weapon::mundane::{NaturalMundaneWeapon, WornMundaneWeapon};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HandlessMundaneWeapon<'source> {
    Natural(NaturalMundaneWeapon<'source>),
    Worn(WornMundaneWeapon<'source>),
}

impl<'source> HandlessMundaneWeapon<'source> {
    pub fn as_memo(&self) -> HandlessMundaneWeaponMemo {
        match self {
            HandlessMundaneWeapon::Natural(view) => {
                HandlessMundaneWeaponMemo::Natural(view.as_memo())
            }
            HandlessMundaneWeapon::Worn(view) => HandlessMundaneWeaponMemo::Worn(view.as_memo()),
        }
    }
}
