use serde::{Deserialize, Serialize};

use crate::armor::armor_item::base::BaseArmor;

use super::MundaneArmorView;

/// A piece of mundane armor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MundaneArmor(pub(crate) BaseArmor);

impl<'source> MundaneArmor {
    pub(crate) fn as_ref(&'source self) -> MundaneArmorView<'source> {
        MundaneArmorView(&self.0)
    }
}
