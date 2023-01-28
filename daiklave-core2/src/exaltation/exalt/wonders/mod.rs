mod memo;
pub(crate) use memo::ExaltWondersMemo;

use std::collections::HashMap;

use crate::{
    armor::armor_item::artifact::ArtifactError,
    artifact::wonders::{OwnedWonder, WonderNoAttunement},
    exaltation::mortal::MortalWonders,
    hearthstones::{HearthstoneError, SlottedHearthstone, UnslottedHearthstone},
    CharacterMutationError,
};

use super::essence::EssenceError;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ExaltWonders<'source>(
    pub(crate) HashMap<&'source str, (WonderNoAttunement<'source>, Option<u8>)>,
);

impl<'source> ExaltWonders<'source> {
    pub fn as_memo(&self) -> ExaltWondersMemo {
        ExaltWondersMemo(
            self.0
                .iter()
                .map(|(k, (no_attunement, attunement))| {
                    ((*k).to_owned(), (no_attunement.as_memo(), *attunement))
                })
                .collect(),
        )
    }

    pub fn iter(&self) -> impl Iterator<Item = &'source str> + '_ {
        self.0.keys().copied()
    }

    pub fn get(&self, name: &str) -> Option<OwnedWonder<'source>> {
        self.0
            .get_key_value(name)
            .map(|(name, (no_attunement, attunement))| {
                OwnedWonder(*name, no_attunement.clone(), *attunement)
            })
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
            .0
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
            .0
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

    pub fn attune_wonder(
        &mut self,
        wonder_name: &str,
        personal_committed: u8,
    ) -> Result<&mut Self, CharacterMutationError> {
        let attunement = &mut self
            .0
            .get_mut(wonder_name)
            .ok_or(CharacterMutationError::ArtifactError(
                ArtifactError::NotFound,
            ))?
            .1;
        if attunement.is_some() {
            Err(CharacterMutationError::EssenceError(
                EssenceError::AlreadyAttuned,
            ))
        } else {
            *attunement = Some(personal_committed);
            Ok(self)
        }
    }

    pub fn unattune_wonder(
        &mut self,
        wonder_name: &str,
    ) -> Result<(u8, u8), CharacterMutationError> {
        let wonder = self
            .0
            .get_mut(wonder_name)
            .ok_or(CharacterMutationError::ArtifactError(
                ArtifactError::NotFound,
            ))?;

        if let Some(amount) = wonder.0.attunement_cost {
            if let Some(personal) = wonder.1.take() {
                Ok((amount - amount.min(personal), amount.min(personal)))
            } else {
                Err(CharacterMutationError::EssenceError(EssenceError::NotFound))
            }
        } else {
            Err(CharacterMutationError::EssenceError(
                EssenceError::NoAttunementCost,
            ))
        }
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
