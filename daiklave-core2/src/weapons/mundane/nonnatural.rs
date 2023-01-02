use super::{worn::WornMundaneWeapon, one_handed::OneHandedMundaneWeapon, two_handed::TwoHandedMundaneWeapon};

pub(in crate::weapons) enum NonnaturalMundaneWeapon<'source> {
    Worn(WornMundaneWeapon<'source>),
    OneHanded(OneHandedMundaneWeapon<'source>),
    TwoHanded(TwoHandedMundaneWeapon<'source>),
}