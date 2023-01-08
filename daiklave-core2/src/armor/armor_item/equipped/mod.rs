mod memo;
mod no_attunement;
pub(crate) use memo::EquippedArmorMemo;
pub(crate) use no_attunement::{EquippedArmorNoAttunement, EquippedArmorNoAttunementMemo};

use super::{
    artifact::{ArtifactArmor, ArtifactArmorId},
    mundane::MundaneArmor,
    BaseArmorId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedArmor<'source> {
    Mundane(BaseArmorId, MundaneArmor<'source>),
    Artifact(ArtifactArmorId, ArtifactArmor<'source>),
}

impl<'source> EquippedArmor<'source> {
    pub fn as_memo(&self) -> EquippedArmorMemo {
        match self {
            EquippedArmor::Mundane(id, view) => EquippedArmorMemo::Mundane(*id, view.as_memo()),
            EquippedArmor::Artifact(id, view) => EquippedArmorMemo::Artifact(*id, view.as_memo()),
        }
    }
}
