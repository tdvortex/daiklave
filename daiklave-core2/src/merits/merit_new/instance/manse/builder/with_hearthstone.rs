use crate::{hearthstones::hearthstone::AddHearthstone, merits::{merit_new::instance::{manse::{AddManse, ManseName}, DemenseName}}};

pub struct ManseBuilderWithHearthstone {
    name: ManseName,
    hearthstone: AddHearthstone
}

impl ManseBuilderWithHearthstone {
    pub fn demense(self, demense: impl Into<DemenseName>) -> AddManse {
        AddManse {
            manse_name: self.name,
            demense_name: demense.into(),
            hearthstone: self.hearthstone,
        }
    }
}