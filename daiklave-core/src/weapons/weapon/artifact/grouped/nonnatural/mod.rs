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

impl<'source> From<&'source NonnaturalArtifactWeaponMemo> for NonnaturalArtifactWeapon<'source> {
    fn from(value: &'source NonnaturalArtifactWeaponMemo) -> Self {
        Self((&value.0).into(), value.1)
    }
}