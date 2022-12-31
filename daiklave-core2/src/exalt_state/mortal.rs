use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::martial_arts::{MartialArtsStyleId, MortalMartialArtist, MortalMartialArtistView};
use crate::sorcery::{TerrestrialCircleSorcerer, TerrestrialCircleSorcererView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct Mortal {
    pub martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtist>,
    pub sorcery: Option<TerrestrialCircleSorcerer>, 
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalView<'source> {
    pub martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtistView<'source>>,
    pub sorcery: Option<TerrestrialCircleSorcererView<'source>>, 
}
