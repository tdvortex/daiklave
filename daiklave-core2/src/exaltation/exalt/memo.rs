use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::charms::charm::evocation::{Evocation};

use super::{
    armor::ExaltArmorMemo, essence::EssenceStateMemo, exalt_type::ExaltTypeMemo,
    martial_arts::ExaltMartialArtistMemo, weapons::ExaltWeaponsMemo, wonders::ExaltWondersMemo,
    Exalt,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltMemo {
    pub(crate) armor: ExaltArmorMemo,
    pub(crate) essence: EssenceStateMemo,
    pub(crate) evocations: Vec<(String, Evocation)>,
    pub(crate) martial_arts_styles: HashMap<String, ExaltMartialArtistMemo>,
    pub(crate) exalt_type: ExaltTypeMemo,
    pub(crate) weapons: ExaltWeaponsMemo,
    pub(crate) wonders: ExaltWondersMemo,
}

impl<'source> ExaltMemo {
    pub fn as_ref(&'source self) -> Exalt<'source> {
        Exalt {
            armor: self.armor.as_ref(),
            essence: self.essence.as_ref(),
            evocations: self
                .evocations
                .iter()
                .map(|(id, charm)| (id.as_str(), charm))
                .collect(),
            martial_arts_styles: self
                .martial_arts_styles
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_ref()))
                .collect(),
            exalt_type: self.exalt_type.as_ref(),
            weapons: self.weapons.as_ref(),
            wonders: self.wonders.as_ref(),
        }
    }
}
