use serde::{Deserialize, Serialize};

use crate::{
    abilities::AbilityRatingMemo,
    martial_arts::{charm::MartialArtsCharmDetails, style::MartialArtsStyleDetails},
};

use super::ExaltMartialArtistDetails;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltMartialArtistDetailsMemo {
    pub style: MartialArtsStyleDetails,
    pub ability: AbilityRatingMemo,
    pub charms: Vec<(String, MartialArtsCharmDetails)>,
}

impl From<&ExaltMartialArtistDetails<'_>> for ExaltMartialArtistDetailsMemo {
    fn from(value: &ExaltMartialArtistDetails<'_>) -> Self {
        Self {
            style: value.style.to_owned(),
            ability: (&value.ability).into(),
            charms: value
                .charms
                .iter()
                .map(|(name, details)| ((*name).into(), (*details).to_owned()))
                .collect(),
        }
    }
}
