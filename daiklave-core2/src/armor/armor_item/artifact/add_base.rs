use crate::armor::armor_item::builder::base::{BaseArtifactArmorBuilderWithWeightClass, BaseArtifactArmorBuilder};

use super::{BaseArtifactArmor, ArtifactArmorName, builder::{ArtifactArmorItemBuilderWithBaseArmor, ArtifactArmorItemBuilder}};

/// The name and properties of a piece of generic artifact armor to be used as
/// a base for a specific piece of artifact armor.
pub struct AddBaseArtifactArmor {
    pub(crate) name: String,
    pub(crate) armor: BaseArtifactArmor,
}

impl AddBaseArtifactArmor {
    /// Starts to build a new piece of base artifact armor with the given name.
    pub fn name(name: impl Into<String>) -> BaseArtifactArmorBuilder {
        BaseArtifactArmorBuilder::name(name)
    }

    /// Starts to build a unique armor item out of this generic piece.
    pub fn unique_name(self, name: impl Into<ArtifactArmorName>) -> ArtifactArmorItemBuilderWithBaseArmor {
        ArtifactArmorItemBuilder::name(name).base_artifact(self)
    }
}

impl From<BaseArtifactArmorBuilderWithWeightClass> for AddBaseArtifactArmor {
    fn from(builder: BaseArtifactArmorBuilderWithWeightClass) -> Self {
        builder.build()
    }
}