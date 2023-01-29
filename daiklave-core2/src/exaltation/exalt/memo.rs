use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::charms::charm::evocation::Evocation;

use super::{
    armor::ExaltArmorMemo, essence::EssenceStateMemo, exalt_type::ExaltTypeMemo,
    martial_arts::ExaltMartialArtistMemo, weapons::ExaltWeaponsMemo, wonders::ExaltWondersMemo,
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
