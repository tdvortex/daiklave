use std::ops::Deref;

use serde::{Serialize, Deserialize};

use crate::unique_id::UniqueId;

/// A id for a base armor item, like "Chain Shirt" or "Silken Armor" but not
/// a named artifact armor item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BaseArmorId(pub UniqueId);

impl Deref for BaseArmorId {
    type Target = UniqueId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}