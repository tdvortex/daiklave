use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::martial_arts::{MortalMartialArtistView, MartialArtsStyleId, MortalMartialArtist};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct Mortal {
    martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtist>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalView<'source> {
    martial_arts_styles: HashMap<MartialArtsStyleId, MortalMartialArtistView<'source>>,
}