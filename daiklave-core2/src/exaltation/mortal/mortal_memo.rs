use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    martial_arts::{MartialArtsStyleId,
    },
    sorcery::circles::terrestrial::sorcerer_memo::TerrestrialCircleSorcererMemo,
};

use super::{martial_arts::MortalMartialArtistMemo, MortalView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct MortalMemo {
    martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtistMemo>,
    sorcery: Option<TerrestrialCircleSorcererMemo>,
}

impl<'source> MortalMemo {
    pub fn new(
        martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtistMemo>,
        sorcery: Option<TerrestrialCircleSorcererMemo>,
    ) -> Self {
        Self {
            martial_arts_styles,
            sorcery,
        }
    }
    
    pub fn as_ref(&'source self) -> MortalView<'source> {
        MortalView { 
            martial_arts_styles: {
                self.martial_arts_styles.iter().map(|(k, v)| (*k, v.as_ref())).collect()
            }, 
            sorcery: self.sorcery.as_ref().map(|sorcery| sorcery.as_ref()) 
        }
    }
}