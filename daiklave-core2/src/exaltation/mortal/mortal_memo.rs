use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    martial_arts::MartialArtsStyleId,
    sorcery::circles::terrestrial::sorcerer_memo::TerrestrialCircleSorcererMemo,
};

use super::{martial_arts::MortalMartialArtistMemo, weapons::MortalWeaponsMemo, Mortal};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMemo {
    martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtistMemo>,
    sorcery: Option<TerrestrialCircleSorcererMemo>,
    weapons: MortalWeaponsMemo,
}

impl<'source> MortalMemo {
    pub fn new(
        martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtistMemo>,
        sorcery: Option<TerrestrialCircleSorcererMemo>,
        weapons: MortalWeaponsMemo,
    ) -> Self {
        Self {
            martial_arts_styles,
            sorcery,
            weapons,
        }
    }

    pub fn as_ref(&'source self) -> Mortal<'source> {
        Mortal {
            martial_arts_styles: {
                self.martial_arts_styles
                    .iter()
                    .map(|(k, v)| (*k, v.as_ref()))
                    .collect()
            },
            sorcery: self.sorcery.as_ref().map(|sorcery| sorcery.as_ref()),
            weapons: self.weapons.as_ref(),
        }
    }
}
