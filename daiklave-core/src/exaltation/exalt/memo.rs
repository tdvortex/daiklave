use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::charms::charm::evocation::Evocation;

use super::{
    armor::ExaltArmorMemo, essence::EssenceStateMemo, exalt_type::ExaltTypeMemo,
    martial_arts::ExaltMartialArtistDetailsMemo, weapons::ExaltWeaponsMemo,
    wonders::ExaltWondersMemo, Exalt,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltMemo {
    pub(crate) armor: ExaltArmorMemo,
    pub(crate) essence: EssenceStateMemo,
    pub(crate) evocations: Vec<(String, Evocation)>,
    pub(crate) martial_arts_styles: HashMap<String, ExaltMartialArtistDetailsMemo>,
    pub(crate) exalt_type: ExaltTypeMemo,
    pub(crate) weapons: ExaltWeaponsMemo,
    pub(crate) wonders: ExaltWondersMemo,
}

impl From<&Exalt<'_>> for ExaltMemo {
    fn from(exalt: &Exalt<'_>) -> Self {
        Self {
            armor: (&exalt.armor).into(),
            essence: (&exalt.essence).into(),
            evocations: exalt
                .evocations
                .iter()
                .map(|(name, evocation)| ((*name).into(), (*evocation).to_owned()))
                .collect(),
            martial_arts_styles: exalt
                .martial_arts_styles
                .iter()
                .map(|(name, details)| ((*name).into(), details.into()))
                .collect(),
            exalt_type: (&exalt.exalt_type).into(),
            weapons: (&exalt.weapons).into(),
            wonders: (&exalt.wonders).into(),
        }
    }
}
