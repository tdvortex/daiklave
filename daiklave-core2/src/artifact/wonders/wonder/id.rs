use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::unique_id::UniqueId;

/// The Id for a magical item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WonderId(pub UniqueId);

impl Deref for WonderId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}