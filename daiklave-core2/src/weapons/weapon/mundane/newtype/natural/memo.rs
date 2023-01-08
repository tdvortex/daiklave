use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;

use super::NaturalMundaneWeaponView;

/// A mundane weapon that is part of the user's body, like Unarmed
/// or shapeshifted claws.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalMundaneWeapon(pub(crate) BaseWeapon);

impl<'source> NaturalMundaneWeapon {
    pub(crate) fn as_ref(&'source self) -> NaturalMundaneWeaponView<'source> {
        NaturalMundaneWeaponView(&self.0)
    }
}
