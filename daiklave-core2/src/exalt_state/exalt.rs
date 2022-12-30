/// Structs and methods related to the Essence rating and mote pools for a
/// character.
pub mod essence;

/// Structs and methods related to various Exalt subtypes (Solar, Lunar, etc).
pub mod exalt_type;
use std::collections::HashMap;

use essence::{Essence, EssenceView};
use exalt_type::{ExaltType, ExaltTypeView};
use serde::{Deserialize, Serialize};

use crate::martial_arts::{ExaltMartialArtist, ExaltMartialArtistView, MartialArtsStyleId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Exalt {
    essence: Essence,
    pub(crate) martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtist>,
    exalt_type: ExaltType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExaltView<'source> {
    essence: EssenceView<'source>,
    pub(crate) martial_arts_styles: HashMap<MartialArtsStyleId, ExaltMartialArtistView<'source>>,
    exalt_type: ExaltTypeView,
}
