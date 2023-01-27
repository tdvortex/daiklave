mod add;
mod add_base;
mod base;
/// A builder path for constructing a new Artifact armor item.
pub(crate) mod builder;
mod error;
mod memo;
mod no_attunement;

pub use add::AddArtifactArmor;
pub use add_base::AddBaseArtifactArmor;
pub use base::BaseArtifactArmor;
pub use error::ArtifactError;
pub use memo::ArtifactArmor;
pub(crate) use no_attunement::{ArtifactArmorNoAttunement, ArtifactArmorNoAttunementMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArtifactArmorView<'source>(
    pub ArtifactArmorNoAttunement<'source>,
    pub Option<u8>,
);

impl<'source> ArtifactArmorView<'source> {
    pub fn as_memo(&self) -> ArtifactArmor {
        ArtifactArmor(self.0.as_memo(), self.1)
    }
}
