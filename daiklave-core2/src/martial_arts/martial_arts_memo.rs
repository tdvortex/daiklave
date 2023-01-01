use crate::CharacterMemo;

use super::{style_id::MartialArtsStyleId, MartialArtistMemo};

/// All of the character's Martial Arts styles.
pub struct MartialArtsMemo<'char>(&'char CharacterMemo);

impl<'char> MartialArtsMemo<'char> {
    /// The details of a particular Martial Arts style.
    pub fn style(&self, id: MartialArtsStyleId) -> Option<MartialArtistMemo<'char>> {
        self.0.exalt_state.martial_artist(id)
    }

    /// Iterates over the Ids for all Martial Arts the character knows.
    pub fn iter(&self) -> impl Iterator<Item = MartialArtsStyleId> {
        self.0.exalt_state.martial_arts_id_iter()
    }
}
