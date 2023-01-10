use crate::artifact::ArtifactId;

use super::{slotted::SlottedHearthstone, unslotted::UnslottedHearthstone};

pub(crate) enum HearthstonePosition<'source> {
    Slotted(ArtifactId, SlottedHearthstone<'source>),
    Unslotted(UnslottedHearthstone<'source>),
}