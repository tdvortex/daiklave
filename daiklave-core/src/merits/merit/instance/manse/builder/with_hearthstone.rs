use crate::{
    hearthstones::hearthstone::AddHearthstone,
    merits::merit::instance::{
        manse::{AddManse, ManseName},
        DemenseName,
    },
};

/// A manse builder after the hearthstone has been specified.
pub struct ManseBuilderWithHearthstone {
    pub(crate) name: ManseName,
    pub(crate) hearthstone: AddHearthstone,
}

impl ManseBuilderWithHearthstone {
    /// Specifies the demense associated with this manse and hearthstone.
    pub fn demense(self, demense: impl Into<DemenseName>) -> AddManse {
        AddManse {
            manse_name: self.name,
            demense_name: demense.into(),
            hearthstone: self.hearthstone,
        }
    }
}
