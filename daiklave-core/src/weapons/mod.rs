mod error;

/// Weapon properties.
pub mod weapon;

pub use error::WeaponError;

use self::weapon::mundane::unarmed;
use self::weapon::{Equipped, Weapon, WeaponName};
use crate::exaltation::Exaltation;

/// The interface for a character's weapons.
pub struct Weapons<'view, 'source>(pub(crate) &'view Exaltation<'source>);

impl<'view, 'source> Weapons<'view, 'source> {
    /// Retrieves the details for a specific weapon, if it exists.
    pub fn get(
        &self,
        weapon_name: WeaponName<'_>,
        equipped: Option<Equipped>,
    ) -> Option<Weapon<'source>> {
        if matches!(weapon_name, WeaponName::Unarmed) {
            Some(unarmed())
        } else {
            self.0.get_weapon(weapon_name, equipped)
        }
    }

    /// Iterates over all of the weapons the character possesses by their name.
    pub fn iter(&self) -> impl Iterator<Item = (WeaponName<'source>, Option<Equipped>)> + '_ {
        self.0.iter_weapons()
    }
}
