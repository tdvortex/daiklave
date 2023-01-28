mod memo;
pub(crate) use memo::MortalWondersMemo;

use std::collections::HashMap;

use crate::{
    armor::armor_item::artifact::ArtifactError,
    artifact::wonders::{OwnedWonder, WonderNoAttunement},
    exaltation::exalt::ExaltWonders,
    hearthstones::{HearthstoneError, SlottedHearthstone, UnslottedHearthstone},
    CharacterMutationError,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct MortalWonders<'source>(
    pub(crate) HashMap<&'source str, WonderNoAttunement<'source>>,
);

impl<'source> MortalWonders<'source> {
    pub fn as_memo(&self) -> MortalWondersMemo {
        MortalWondersMemo(
            self.0
                .iter()
                .map(|(k, v)| ((*k).to_owned(), v.as_memo()))
                .collect(),
        )
    }

    pub fn iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        self.0.keys().copied()
    }

    pub fn get(&self, name: &str) -> Option<OwnedWonder<'source>> {
        self.0
            .get_key_value(name)
            .map(|(name, no_attunement)| OwnedWonder(*name, no_attunement.clone(), None))
    }

    pub fn slot_hearthstone(
        &mut self,
        wonder_name: &str,
        hearthstone_name: &'source str,
        unslotted: UnslottedHearthstone<'source>,
    ) -> Result<&mut Self, CharacterMutationError> {
        *self
            .0
            .get_mut(wonder_name)
            .ok_or(CharacterMutationError::ArtifactError(
                ArtifactError::NotFound,
            ))?
            .hearthstone_slots
            .iter_mut()
            .find(|maybe_hearthstone| maybe_hearthstone.is_none())
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::AllSlotsFilled,
            ))? = Some(SlottedHearthstone {
            name: hearthstone_name,
            details: unslotted.details,
            origin: unslotted.origin,
        });
        Ok(self)
    }

    pub fn unslot_hearthstone(
        &mut self,
        wonder_name: &str,
        hearthstone_name: &str,
    ) -> Result<(&'source str, UnslottedHearthstone<'source>), CharacterMutationError> {
        let SlottedHearthstone {
            name,
            details,
            origin,
        } = self
            .0
            .get_mut(wonder_name)
            .ok_or(CharacterMutationError::ArtifactError(
                ArtifactError::NotFound,
            ))?
            .hearthstone_slots
            .iter_mut()
            .find_map(|maybe_hearthstone| {
                if maybe_hearthstone
                    .map_or(false, |hearthstone| hearthstone.name == hearthstone_name)
                {
                    maybe_hearthstone.take()
                } else {
                    None
                }
            })
            .ok_or(CharacterMutationError::HearthstoneError(
                HearthstoneError::NotFound,
            ))?;

        Ok((name, UnslottedHearthstone { details, origin }))
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
