use super::{worn::WornMundaneWeapon, natural::NaturalMundaneWeapon};

pub(in crate::weapons) enum HandlessMundaneWeapon<'source> {
    Natural(NaturalMundaneWeapon<'source>),
    Worn(WornMundaneWeapon<'source>),
}