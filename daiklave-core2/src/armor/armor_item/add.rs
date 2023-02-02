use crate::CharacterMutation;

use super::{
    artifact::{builder::ArtifactArmorItemBuilder, AddArtifactArmor, ArtifactArmorName},
    builder::base::{BaseArtifactArmorBuilder, MundaneArmorBuilder},
    mundane::{AddMundaneArmor, MundaneArmorName},
};

/// A mutation to add a piece of armor to a character.
pub enum AddArmor {
    /// Adds a named piece of artifact armor to the character.
    Artifact(AddArtifactArmor),
    /// Adds a mundane, nonmagical piece of armor to the character.
    Mundane(AddMundaneArmor),
}

impl AddArmor {
    /// Starts constructing a base artifact armor item.
    pub fn base_artifact(name: impl Into<String>) -> BaseArtifactArmorBuilder {
        BaseArtifactArmorBuilder::name(name)
    }

    /// Starts constructing a piece of named artifact armor.
    pub fn artifact(name: impl Into<ArtifactArmorName>) -> ArtifactArmorItemBuilder {
        ArtifactArmorItemBuilder::name(name)
    }

    /// Starts constructing a piece of mundane armor.
    pub fn mundane(name: impl Into<MundaneArmorName>) -> MundaneArmorBuilder {
        MundaneArmorBuilder::name(name)
    }
}

impl From<AddArtifactArmor> for AddArmor {
    fn from(add_artifact: AddArtifactArmor) -> Self {
        Self::Artifact(add_artifact)
    }
}

impl From<AddMundaneArmor> for AddArmor {
    fn from(add_mundane: AddMundaneArmor) -> Self {
        Self::Mundane(add_mundane)
    }
}

impl From<AddArmor> for CharacterMutation {
    fn from(add_armor: AddArmor) -> Self {
        match add_armor {
            AddArmor::Artifact(add_artifact) => add_artifact.into(),
            AddArmor::Mundane(add_mundane) => add_mundane.into(),
        }
    }
}
