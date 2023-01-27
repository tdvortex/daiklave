use crate::{artifact::ArtifactId, book_reference::BookReference};

use super::{
    category::HearthstoneCategory, geomancy_level::GeomancyLevel, id::HearthstoneId,
    keyword::HearthstoneKeyword, slotted::SlottedHearthstone, unslotted::UnslottedHearthstone,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HearthstonePosition<'source> {
    Slotted(ArtifactId<'source>, SlottedHearthstone<'source>),
    Unslotted(HearthstoneId, UnslottedHearthstone<'source>),
}

impl<'source> HearthstonePosition<'source> {
    pub fn id(&self) -> HearthstoneId {
        match self {
            HearthstonePosition::Slotted(_, slotted) => slotted.id(),
            HearthstonePosition::Unslotted(id, _) => *id,
        }
    }

    pub fn name(&self) -> &'source str {
        match self {
            HearthstonePosition::Slotted(_, slotted) => slotted.name(),
            HearthstonePosition::Unslotted(_, unslotted) => unslotted.name(),
        }
    }

    pub fn slotted_into(&self) -> Option<ArtifactId<'source>> {
        match self {
            HearthstonePosition::Slotted(artifact_id, _) => Some(*artifact_id),
            HearthstonePosition::Unslotted(_, _) => None,
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            HearthstonePosition::Slotted(_, slotted) => slotted.book_reference(),
            HearthstonePosition::Unslotted(_, unslotted) => unslotted.book_reference(),
        }
    }

    pub fn category(&self) -> HearthstoneCategory {
        match self {
            HearthstonePosition::Slotted(_, slotted) => slotted.category(),
            HearthstonePosition::Unslotted(_, unslotted) => unslotted.category(),
        }
    }

    pub fn powers(&self) -> &'source str {
        match self {
            HearthstonePosition::Slotted(_, slotted) => slotted.powers(),
            HearthstonePosition::Unslotted(_, unslotted) => unslotted.powers(),
        }
    }

    pub fn geomancy_level(&self) -> GeomancyLevel {
        match self {
            HearthstonePosition::Slotted(_, slotted) => slotted.geomancy_level(),
            HearthstonePosition::Unslotted(_, unslotted) => unslotted.geomancy_level(),
        }
    }

    pub fn keywords(&self) -> impl Iterator<Item = HearthstoneKeyword> {
        let mut keywords = match self {
            HearthstonePosition::Slotted(_, slotted) => {
                slotted.keywords().collect::<Vec<HearthstoneKeyword>>()
            }
            HearthstonePosition::Unslotted(_, unslotted) => {
                unslotted.keywords().collect::<Vec<HearthstoneKeyword>>()
            }
        };
        keywords.sort();
        keywords.into_iter()
    }

    pub fn manse_and_demense(&self) -> Option<(&'source str, &'source str)> {
        match self {
            HearthstonePosition::Slotted(_, slotted) => slotted.manse_and_demense(),
            HearthstonePosition::Unslotted(_, unslotted) => unslotted.manse_and_demense(),
        }
    }
}
