mod memo;
pub(crate) use memo::MortalWondersMemo;

use std::collections::HashMap;

use crate::{
    artifact::wonders::{OwnedWonder, WonderId, WonderNoAttunement},
    exaltation::exalt::ExaltWonders, hearthstones::{HearthstoneId, UnslottedHearthstone, HearthstoneError, SlottedHearthstone}, CharacterMutationError, armor::armor_item::artifact::ArtifactError,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalWonders<'source>(pub(crate) HashMap<WonderId, WonderNoAttunement<'source>>);

impl<'source> MortalWonders<'source> {
    pub fn as_memo(&self) -> MortalWondersMemo {
        MortalWondersMemo(self.0.iter().map(|(k, v)| (*k, v.as_memo())).collect())
    }

    pub fn iter(&self) -> impl Iterator<Item = WonderId> + '_ {
        self.0.keys().copied()
    }

    pub fn get(&self, wonder_id: WonderId) -> Option<OwnedWonder<'source>> {
        self.0
            .get(&wonder_id)
            .map(|no_attunement| OwnedWonder(wonder_id, no_attunement.clone(), None))
    }

    pub fn slot_hearthstone(
        &mut self,
        wonder_id: WonderId,
        hearthstone_id: HearthstoneId,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        *self
            .0
            .get_mut(&wonder_id)
            .ok_or(CharacterMutationError::ArtifactError(
                ArtifactError::NotFound,
            ))?
            .hearthstone_slots
            .iter_mut()
            .find(|maybe_hearthstone| maybe_hearthstone.is_none())
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::AllSlotsFilled,
            ))? = Some(SlottedHearthstone {
            hearthstone_id,
            details: unslotted.details,
            origin: unslotted.origin,
        });
        Ok(self)
    }

    pub fn unslot_hearthstone(
        &mut self,
        wonder_id: WonderId,
        hearthstone_id: HearthstoneId,
    ) -> Result<UnslottedHearthstone<'source>, CharacterMutationError> {
        let SlottedHearthstone {
            hearthstone_id: _,
            details,
            origin,
        } = self
            .0
            .get_mut(&wonder_id)
            .ok_or(CharacterMutationError::ArtifactError(
                ArtifactError::NotFound,
            ))?
            .hearthstone_slots
            .iter_mut()
            .find_map(|maybe_hearthstone| {
                if maybe_hearthstone.map_or(false, |hearthstone| hearthstone.id() == hearthstone_id)
                {
                    maybe_hearthstone.take()
                } else {
                    None
                }
            })
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))?;

        Ok(UnslottedHearthstone { details, origin })
    }
}

impl<'source> From<ExaltWonders<'source>> for MortalWonders<'source> {
    fn from(exalt: ExaltWonders<'source>) -> Self {
        MortalWonders(
            exalt
                .0
                .into_iter()
                .map(|(id, (no_attunement, _))| (id, no_attunement))
                .collect(),
        )
    }
}
