mod base;
mod handless;
mod nonnatural;
mod natural;
mod one_handed;
mod two_handed;
mod worn;

pub(in crate::weapons) use handless::HandlessMundaneWeapon;
pub(in crate::weapons) use nonnatural::NonnaturalMundaneWeapon;
pub(in crate::weapons) use one_handed::OneHandedMundaneWeapon;
pub(in crate::weapons) use two_handed::TwoHandedMundaneWeapon;

use self::{natural::NaturalMundaneWeapon, worn::WornMundaneWeapon};

enum MundaneWeapon<'source> {
    Natural(NaturalMundaneWeapon<'source>),
    Worn(WornMundaneWeapon<'source>),
    OneHanded(OneHandedMundaneWeapon<'source>),
    TwoHanded(TwoHandedMundaneWeapon<'source>),
}