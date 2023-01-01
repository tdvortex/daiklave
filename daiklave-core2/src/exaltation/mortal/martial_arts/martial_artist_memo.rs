use serde::{Deserialize, Serialize};

use crate::{
    abilities::AbilityMemo,
    martial_arts::MartialArtsStyle,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct MortalMartialArtistMemo {
    pub style: MartialArtsStyle,
    pub ability: AbilityMemo,
}