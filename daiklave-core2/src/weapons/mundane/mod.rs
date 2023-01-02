mod base;
mod handless;
mod nonnatural;
mod natural;
mod one_handed;
mod two_handed;
mod worn;

use std::ops::Deref;

pub(in crate::weapons) use handless::HandlessMundaneWeapon;
pub(in crate::weapons) use nonnatural::NonnaturalMundaneWeapon;
pub use one_handed::OneHandedMundaneWeapon;
pub use two_handed::TwoHandedMundaneWeapon;
pub use natural::NaturalMundaneWeapon;
pub use worn::WornMundaneWeapon;

use super::base::BaseWeapon;

pub(in crate::weapons) enum MundaneWeapon<'source> {
    Natural(NaturalMundaneWeapon<'source>),
    Worn(WornMundaneWeapon<'source>),
    OneHanded(OneHandedMundaneWeapon<'source>),
    TwoHanded(TwoHandedMundaneWeapon<'source>),
}

impl<'source> Deref for MundaneWeapon<'source> {
    type Target = BaseWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            MundaneWeapon::Natural(deref) => deref,
            MundaneWeapon::Worn(deref) => deref,
            MundaneWeapon::OneHanded(deref) => deref,
            MundaneWeapon::TwoHanded(deref) => deref,
        }
    }
}