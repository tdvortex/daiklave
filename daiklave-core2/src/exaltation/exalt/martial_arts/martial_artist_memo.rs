use serde::{Deserialize, Serialize};

use crate::{
    abilities::AbilityRatingMemo,
    martial_arts::{charm::MartialArtsCharmDetails, style::MartialArtsStyleDetails},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltMartialArtistMemo {
    pub style: MartialArtsStyleDetails,
    pub ability: AbilityRatingMemo,
    pub charms: Vec<(String, MartialArtsCharmDetails)>,
}