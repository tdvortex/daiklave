use crate::{CharacterMutation, merits::merit_new::RemoveMerit};

use super::{artifact::ArtifactArmorName, mundane::{MundaneArmorName, RemoveMundaneArmor}, ArmorName};

pub enum RemoveArmor {
    Artifact(ArtifactArmorName),
    Mundane(MundaneArmorName),
}

impl From<ArmorName<'_>> for RemoveArmor {
    fn from(name: ArmorName<'_>) -> Self {
        name.remove()
    }
}

impl From<RemoveArmor> for CharacterMutation {
    fn from(remove_armor: RemoveArmor) -> Self {
        match remove_armor {
            RemoveArmor::Artifact(artifact_armor_name) => RemoveMerit::Artifact(artifact_armor_name.into()).into(),
            RemoveArmor::Mundane(mundane_armor_name) => RemoveMundaneArmor(mundane_armor_name).into(),
        }
    }
}