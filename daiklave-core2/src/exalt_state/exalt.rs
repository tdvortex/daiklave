pub mod essence;
pub mod exalt_type;
use std::collections::HashMap;

use essence::{Essence, EssenceView};
use exalt_type::{ExaltType, ExaltTypeView};
use serde::{Serialize, Deserialize};

use crate::{martial_arts::{MartialArtsStyleId, ExaltMartialArtist, ExaltMartialArtistView}};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Exalt {
    essence: Essence,
    martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtist>,
    exalt_type: ExaltType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExaltView<'source> {
    essence: EssenceView<'source>,
    martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtistView<'source>>,
    exalt_type: ExaltTypeView,
}