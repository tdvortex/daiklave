use crate::{
    artifact::ArtifactName,
    book_reference::BookReference,
    merits::merit::{Merit, MeritSource},
};

use super::{
    category::HearthstoneCategory, geomancy_level::GeomancyLevel, keyword::HearthstoneKeyword,
    slotted::SlottedHearthstone, unslotted::UnslottedHearthstone,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HearthstonePosition<'source> {
    Slotted(ArtifactName<'source>, SlottedHearthstone<'source>),
    Unslotted(&'source str, UnslottedHearthstone<'source>),
}

impl<'source> HearthstonePosition<'source> {
    pub(crate) fn merits(&self) -> Vec<Merit<'source>> {
        match self {
            HearthstonePosition::Slotted(_, slotted) => slotted.merits(),
            HearthstonePosition::Unslotted(name, slotted) => {
                if let Some((manse, demense)) = slotted.manse_and_demense() {
                    vec![
                        Merit(MeritSource::Demense {
                            name: demense,
                            has_manse: true,
                            geomancy_level: self.geomancy_level(),
                        }),
                        Merit(MeritSource::Hearthstone {
                            name,
                            has_manse: true,
                            geomancy_level: self.geomancy_level(),
                        }),
                        Merit(MeritSource::Manse {
                            name: manse,
                            geomancy_level: self.geomancy_level(),
                        }),
                    ]
                } else {
                    vec![Merit(MeritSource::Hearthstone {
                        name,
                        has_manse: false,
                        geomancy_level: self.geomancy_level(),
                    })]
                }
            }
        }
    }

    pub fn name(&self) -> &'source str {
        match self {
            HearthstonePosition::Slotted(_, slotted) => slotted.name,
            HearthstonePosition::Unslotted(name, _) => name,
        }
    }

    pub fn slotted_into(&self) -> Option<ArtifactName<'source>> {
        match self {
            HearthstonePosition::Slotted(artifact_name, _) => Some(*artifact_name),
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
