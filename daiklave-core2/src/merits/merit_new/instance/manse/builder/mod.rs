mod with_hearthstone;
pub use with_hearthstone::ManseBuilderWithHearthstone;

use crate::hearthstones::{hearthstone::AddHearthstone, HearthstoneError, HearthstoneStability};

use super::ManseName;

pub struct ManseBuilder {
    name: ManseName
}

impl ManseBuilder {
    pub fn name(name: impl Into<ManseName>) -> Self {
        Self {
            name: name.into(),
        }
    }

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