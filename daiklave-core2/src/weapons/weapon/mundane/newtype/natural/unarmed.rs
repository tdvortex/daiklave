use crate::weapons::weapon::{Weapon, WeaponType};

pub fn unarmed() -> Weapon<'static> {
    Weapon(WeaponType::Unarmed)
}
