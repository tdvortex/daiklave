use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactName;

use super::{
    builder::{HearthstoneBuilder, HearthstoneBuilderWithCategory},
    HearthstoneCategory, SlotHearthstone, UnslotHearthstone,
};

/// The name of a Hearthstone.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HearthstoneName(String);

impl HearthstoneName {
    /// Starts a builder to construct a hearthstone with this name and category.
    pub fn category(self, category: HearthstoneCategory) -> HearthstoneBuilderWithCategory {
        HearthstoneBuilder::name(self).category(category)
    }

    /// Creates a mutation to slot this hearthstone into an artifact.
    pub fn slot_into(self, artifact_name: ArtifactName<'_>) -> SlotHearthstone {
        SlotHearthstone {
            artifact_name: artifact_name.into(),
            hearthstone_name: self,
        }
    }

    /// Creates a mutation to unslot this hearthstone.
    pub fn unslot(self) -> UnslotHearthstone {
        UnslotHearthstone(self)
    }
}

impl<T> From<T> for HearthstoneName
where
    T: Into<String>,
{
    fn from(name: T) -> Self {
        Self(name.into())
    }
}

impl Deref for HearthstoneName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
