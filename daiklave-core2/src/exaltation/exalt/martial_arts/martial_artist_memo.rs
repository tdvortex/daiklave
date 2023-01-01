use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    abilities::AbilityMemo,
    martial_arts::{MartialArtsCharm, MartialArtsCharmId, MartialArtsStyle},
};

use super::ExaltMartialArtistView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ExaltMartialArtistMemo {
    style: MartialArtsStyle,
    ability: AbilityMemo,
    charms: HashMap<MartialArtsCharmId, MartialArtsCharm>,
}

impl<'source> ExaltMartialArtistMemo {
    pub(in crate::exaltation::exalt::martial_arts) fn new(
        style: MartialArtsStyle,
        ability: AbilityMemo,
        charms: HashMap<MartialArtsCharmId, MartialArtsCharm>,
    ) -> Self {
        Self {
            style,
            ability,
            charms,
        }
    }

    pub fn as_ref(&'source self) -> ExaltMartialArtistView<'source> {
        ExaltMartialArtistView::new(
            &self.style,
            self.ability.as_ref(),
            self.charms.iter().map(|(k, v)| (*k, v)).collect(),
        )
    }
}
