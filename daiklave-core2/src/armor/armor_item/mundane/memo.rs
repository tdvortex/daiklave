use serde::{Deserialize, Serialize};

use crate::armor::armor_item::base::BaseArmor;

/// A piece of mundane armor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MundaneArmor(pub(crate) BaseArmor);