mod add;
mod add_base;
mod base;
/// A builder path for constructing a new Artifact armor item.
pub(crate) mod builder;
mod error;
mod memo;
mod name;
mod no_attunement;

pub use add::AddArtifactArmor;
pub use add_base::AddBaseArtifactArmor;
pub use base::BaseArtifactArmor;
pub use error::ArtifactError;
pub(crate) use memo::ArtifactArmor;
pub use name::ArtifactArmorName;
pub(crate) use no_attunement::{ArtifactArmorNoAttunement, ArtifactArmorNoAttunementMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArtifactArmorView<'source>(
    pub ArtifactArmorNoAttunement<'source>,
    pub Option<u8>,
);

impl<'source> From<&'source ArtifactArmor> for ArtifactArmorView<'source> {
    fn from(artifact_armor: &'source ArtifactArmor) -> Self {
        Self((&artifact_armor.0).into(), artifact_armor.1)
    }
}
