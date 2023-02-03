mod memo;
mod no_attunement;
pub(crate) use memo::EquippedArmorMemo;
pub(crate) use no_attunement::{EquippedArmorNoAttunement, EquippedArmorNoAttunementMemo};

use super::{artifact::ArtifactArmorView, mundane::MundaneArmorView};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EquippedArmor<'source> {
    Mundane(&'source str, MundaneArmorView<'source>),
    Artifact(&'source str, ArtifactArmorView<'source>),
}

impl<'source> From<&'source EquippedArmorMemo> for EquippedArmor<'source> {
    fn from(memo: &'source EquippedArmorMemo) -> Self {
        match memo {
            EquippedArmorMemo::Mundane(name, memo) => {
                EquippedArmor::Mundane(name.as_str(), memo.into())
            }
            EquippedArmorMemo::Artifact(name, memo) => {
                EquippedArmor::Artifact(name.as_str(), memo.into())
            }
        }
    }
}
