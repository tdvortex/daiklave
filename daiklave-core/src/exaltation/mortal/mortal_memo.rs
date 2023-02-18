use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::sorcery::circles::terrestrial::sorcerer_memo::TerrestrialCircleSorcererMemo;

use super::{
    armor::MortalArmorMemo, martial_arts::MortalMartialArtistDetailsMemo, weapons::MortalWeaponsMemo,
    wonders::MortalWondersMemo, Mortal,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMemo {
    pub armor: MortalArmorMemo,
    pub martial_arts_styles: HashMap<String, MortalMartialArtistDetailsMemo>,
    pub sorcery: Option<TerrestrialCircleSorcererMemo>,
    pub weapons: MortalWeaponsMemo,
    pub wonders: MortalWondersMemo,
    pub exalted_healing: bool,
}

impl From<&Mortal<'_>> for MortalMemo {
    fn from(mortal: &Mortal<'_>) -> Self {
        Self {
            armor: (&mortal.armor).into(),
            martial_arts_styles: mortal.martial_arts_styles.iter().map(|(&name, details)| (name.into(), details.into())).collect(),
            sorcery: mortal.sorcery.as_ref().map(|terrestrial| terrestrial.into()),
            weapons: (&mortal.weapons).into(),
            wonders: (&mortal.wonders).into(),
            exalted_healing: mortal.exalted_healing,
        }
    }
}