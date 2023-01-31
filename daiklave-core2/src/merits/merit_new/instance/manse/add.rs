use crate::{merits::merit_new::instance::DemenseName, hearthstones::hearthstone::AddHearthstone};

use super::{name::ManseName, builder::ManseBuilder};

pub struct AddManse {
    manse_name: ManseName,
    demense_name: DemenseName,
    hearthstone: AddHearthstone,
}

impl AddManse {
    pub fn name(name: impl Into<ManseName>) -> ManseBuilder {
        ManseBuilder {
            name: name.into(),
        }
    }
}