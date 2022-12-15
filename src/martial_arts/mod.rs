use serde::{Serialize, Deserialize};

use crate::charms::MartialArtsCharm;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MartialArtistTraits {
    style: String,
    dots: u8,
    martial_arts_charms: Vec<MartialArtsCharm>,
}