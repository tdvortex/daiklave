use super::BaseArtifactArmor;

/// The name and properties of a piece of generic artifact armor to be used as
/// a base for a specific piece of artifact armor.
pub struct AddBaseArtifactArmor {
    pub(crate) name: String,
    pub(crate) armor: BaseArtifactArmor,
}