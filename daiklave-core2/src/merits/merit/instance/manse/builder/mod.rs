mod with_hearthstone;
pub use with_hearthstone::ManseBuilderWithHearthstone;

use crate::hearthstones::{hearthstone::AddHearthstone, HearthstoneError, HearthstoneStability};

use super::ManseName;

/// A builder to construct a new Manse (with an associated demense and hearthstone).
pub struct ManseBuilder {
    name: ManseName
}

impl ManseBuilder {
    /// Starts a builder with the given manse name.
    pub fn name(name: impl Into<ManseName>) -> Self {
        Self {
            name: name.into(),
        }
    }

    /// Adds a hearthstone to the manse. Return an Err if the hearthstone is 
    /// Wild-Born.
    pub fn hearthstone(self, hearthstone: AddHearthstone) -> Result<ManseBuilderWithHearthstone, HearthstoneError> {
        if hearthstone.template.stability == HearthstoneStability::WildBorn {
            Err(HearthstoneError::WildBornWithManse)
        } else {
            Ok(ManseBuilderWithHearthstone {
                name: self.name,
                hearthstone,
            })
        }
    }
}