use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::martial_arts::MartialArtsStyleId;

use super::{
    armor::ExaltArmorMemo, essence::EssenceStateMemo, exalt_type::ExaltTypeMemo,
    martial_arts::ExaltMartialArtistMemo, weapons::ExaltWeaponsMemo, wonders::ExaltWondersMemo,
    Exalt,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltMemo {
    armor: ExaltArmorMemo,
    essence: EssenceStateMemo,
    martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtistMemo>,
    exalt_type: ExaltTypeMemo,
    weapons: ExaltWeaponsMemo,
    wonders: ExaltWondersMemo,
}

impl<'source> ExaltMemo {
    pub(in crate::exaltation::exalt) fn new(
        armor: ExaltArmorMemo,
        essence: EssenceStateMemo,
        martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtistMemo>,
        exalt_type: ExaltTypeMemo,
        weapons: ExaltWeaponsMemo,
        wonders: ExaltWondersMemo,
    ) -> Self {
        Self {
            armor,
            essence,
            martial_arts_styles,
            exalt_type,
            weapons,
            wonders,
        }
    }

    pub fn as_ref(&'source self) -> Exalt<'source> {
        Exalt::new(
            self.armor.as_ref(),
            self.essence.as_ref(),
            self.martial_arts_styles
                .iter()
                .map(|(k, v)| (*k, v.as_ref()))
                .collect(),
            self.exalt_type.as_ref(),
            self.weapons.as_ref(),
            self.wonders.as_ref(),
        )
    }
}
