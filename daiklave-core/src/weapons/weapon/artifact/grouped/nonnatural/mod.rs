mod memo;
mod no_attunement;

pub(crate) use memo::NonnaturalArtifactWeaponMemo;
pub(crate) use no_attunement::{
    NonnaturalArtifactWeaponNoAttunement, NonnaturalArtifactWeaponNoAttunementMemo,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NonnaturalArtifactWeapon<'source>(
    pub NonnaturalArtifactWeaponNoAttunement<'source>,
    pub Option<u8>,
);

impl<'source> From<NonnaturalArtifactWeaponNoAttunement<'source>>
    for NonnaturalArtifactWeapon<'source>
{
    fn from(unattuned: NonnaturalArtifactWeaponNoAttunement<'source>) -> Self {
        Self(unattuned, None)
    }
}
