use serde::{Serialize, Deserialize};

use crate::armor::armor_item::base::BaseArmor;

/// A piece of mundane armor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MundaneArmorMemo(pub(crate) BaseArmor);