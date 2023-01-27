mod memo;
mod no_attunement;
pub(crate) use memo::EquippedArmorMemo;
pub(crate) use no_attunement::{EquippedArmorNoAttunement, EquippedArmorNoAttunementMemo};

use super::{
    artifact::{ArtifactArmorId, ArtifactArmorView},
    mundane::MundaneArmorView,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedArmor<'source> {
    Mundane(&'source str, MundaneArmorView<'source>),
    Artifact(ArtifactArmorId, ArtifactArmorView<'source>),
}

impl<'source> EquippedArmor<'source> {
    pub fn as_memo(&self) -> EquippedArmorMemo {
        match self {
            EquippedArmor::Mundane(name, view) => {
                EquippedArmorMemo::Mundane((*name).to_owned(), view.as_memo())
            }
            EquippedArmor::Artifact(id, view) => EquippedArmorMemo::Artifact(*id, view.as_memo()),
        }
    }
}
