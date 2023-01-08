mod base;
/// A builder path for constructing a new Artifact armor item.
pub mod builder;
mod error;
mod id;
mod memo;
mod no_attunement;

pub use base::BaseArtifactArmor;
pub use error::ArtifactError;
pub use id::ArtifactArmorId;
pub use memo::ArtifactArmorMemo;
pub(crate) use no_attunement::{ArtifactArmorNoAttunement, ArtifactArmorNoAttunementMemo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArtifactArmor<'source>(pub ArtifactArmorNoAttunement<'source>, pub Option<u8>);

impl<'source> ArtifactArmor<'source> {
    pub fn as_memo(&self) -> ArtifactArmorMemo {
        ArtifactArmorMemo(self.0.as_memo(), self.1)
    }
}
