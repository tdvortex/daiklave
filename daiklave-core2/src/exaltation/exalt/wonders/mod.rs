mod memo;
pub(crate) use memo::ExaltWondersMemo;

use std::collections::HashMap;

use crate::{
    armor::armor_item::artifact::ArtifactError,
    artifact::wonders::{OwnedWonder, WonderId, WonderNoAttunement},
    exaltation::mortal::MortalWonders,
    hearthstones::{HearthstoneError, HearthstoneId, SlottedHearthstone, UnslottedHearthstone},
    CharacterMutationError,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExaltWonders<'source>(
    pub(crate) HashMap<WonderId, (WonderNoAttunement<'source>, Option<u8>)>,
);

impl<'source> ExaltWonders<'source> {
    pub fn as_memo(&self) -> ExaltWondersMemo {
        ExaltWondersMemo(
            self.0
                .iter()
                .map(|(k, (no_attunement, attunement))| {
                    (*k, (no_attunement.as_memo(), *attunement))
                })
                .collect(),
        )
    }

    pub fn iter(&self) -> impl Iterator<Item = WonderId> + '_ {
        self.0.keys().copied()
    }

    pub fn get(&self, wonder_id: WonderId) -> Option<OwnedWonder<'source>> {
        self.0.get(&wonder_id).map(|(no_attunement, attunement)| {
            OwnedWonder(wonder_id, no_attunement.clone(), *attunement)
        })
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
            .0
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
            .0
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

impl<'source> From<MortalWonders<'source>> for ExaltWonders<'source> {
    fn from(mortal: MortalWonders<'source>) -> Self {
        ExaltWonders(
            mortal
                .0
                .into_iter()
                .map(|(id, no_attunement)| (id, (no_attunement, None)))
                .collect(),
        )
    }
}
