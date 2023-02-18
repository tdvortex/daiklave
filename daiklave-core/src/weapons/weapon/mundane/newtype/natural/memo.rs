use serde::{Deserialize, Serialize};

use crate::weapons::weapon::base::BaseWeapon;

use super::NaturalMundaneWeaponView;

/// A mundane weapon that is part of the user's body, like Unarmed
/// or shapeshifted claws.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalMundaneWeapon(pub(crate) BaseWeapon);

impl From<&NaturalMundaneWeaponView<'_>> for NaturalMundaneWeapon {
    fn from(view: &NaturalMundaneWeaponView<'_>) -> Self {
        Self(view.0.to_owned())
    }
}