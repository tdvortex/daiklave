mod memo;
pub(crate) use memo::EquippedArmorNoAttunementMemo;

use crate::armor::armor_item::{artifact::ArtifactArmorNoAttunement, mundane::MundaneArmorView};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedArmorNoAttunement<'source> {
    Mundane(&'source str, MundaneArmorView<'source>),
    Artifact(&'source str, ArtifactArmorNoAttunement<'source>),
}

impl<'source> From<&'source EquippedArmorNoAttunementMemo> for EquippedArmorNoAttunement<'source> {
    fn from(memo: &'source EquippedArmorNoAttunementMemo) -> Self {
        match memo {
            EquippedArmorNoAttunementMemo::Mundane(name, mundane_armor) => {
                EquippedArmorNoAttunement::Mundane(name.as_str(), mundane_armor.into())
            }
            EquippedArmorNoAttunementMemo::Artifact(name, artifact_armor) => {
                EquippedArmorNoAttunement::Artifact(name.as_str(), artifact_armor.into())
            }
        }
    }
}