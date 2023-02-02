use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::sorcery::circles::terrestrial::sorcerer_memo::TerrestrialCircleSorcererMemo;

use super::{
    armor::MortalArmorMemo, martial_arts::MortalMartialArtistMemo, weapons::MortalWeaponsMemo,
    wonders::MortalWondersMemo,
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
