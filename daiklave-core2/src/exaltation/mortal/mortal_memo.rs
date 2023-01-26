use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::sorcery::circles::terrestrial::sorcerer_memo::TerrestrialCircleSorcererMemo;

use super::{
    armor::MortalArmorMemo, martial_arts::MortalMartialArtistMemo, weapons::MortalWeaponsMemo,
    wonders::MortalWondersMemo, Mortal,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMemo {
    pub armor: MortalArmorMemo,
    pub martial_arts_styles: HashMap<String, MortalMartialArtistMemo>,
    pub sorcery: Option<TerrestrialCircleSorcererMemo>,
    pub weapons: MortalWeaponsMemo,
    pub wonders: MortalWondersMemo,
    pub exalted_healing: bool,
}

impl<'source> MortalMemo {
    pub fn as_ref(&'source self) -> Mortal<'source> {
        Mortal {
            martial_arts_styles: {
                self.martial_arts_styles
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_ref()))
                    .collect()
            },
            sorcery: self.sorcery.as_ref().map(|sorcery| sorcery.as_ref()),
            weapons: self.weapons.as_ref(),
            armor: self.armor.as_ref(),
            wonders: self.wonders.as_ref(),
            exalted_healing: self.exalted_healing,
        }
    }
}
