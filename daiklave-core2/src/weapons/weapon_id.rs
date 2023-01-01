use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::unique_id::UniqueId;

/// A unique identifier for either a mortal weapon (e.g. sword) or a base
/// artifact weapon (e.g. daiklave)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct WeaponId(pub UniqueId);

impl Deref for WeaponId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
