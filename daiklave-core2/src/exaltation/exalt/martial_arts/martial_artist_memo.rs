use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    abilities::AbilityMemo,
    martial_arts::{MartialArtsCharm, MartialArtsCharmId, MartialArtsStyle},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltMartialArtistMemo {
    style: MartialArtsStyle,
    ability: AbilityMemo,
    charms: HashMap<MartialArtsCharmId, MartialArtsCharm>,
}