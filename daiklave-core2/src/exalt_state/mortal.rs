use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::martial_arts::{MartialArtsStyleId, MortalMartialArtist, MortalMartialArtistView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct Mortal {
    pub(crate) martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtist>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalView<'source> {
    pub(crate) martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtistView<'source>>,
}
