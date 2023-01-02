use crate::weapons::equipped::{EquippedOneHandedWeapon, EquippedTwoHandedWeapon};

pub(in crate::weapons::exalt) enum ExaltHands<'source> {
    Empty,
    MainHand(EquippedOneHandedWeapon<'source>),
    OffHand(EquippedOneHandedWeapon<'source>),
    Both([EquippedOneHandedWeapon<'source>; 2]),
    TwoHanded(EquippedTwoHandedWeapon<'source>),
}