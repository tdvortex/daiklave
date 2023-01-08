use serde::{Deserialize, Serialize};

use crate::armor::armor_item::base::BaseArmor;

use super::MundaneArmor;

/// A piece of mundane armor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MundaneArmorMemo(pub(crate) BaseArmor);

impl<'source> MundaneArmorMemo {
    pub(crate) fn as_ref(&'source self) -> MundaneArmor<'source> {
        MundaneArmor(&self.0)
    }
}
