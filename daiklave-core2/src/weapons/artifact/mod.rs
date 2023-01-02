mod base;
mod handless;
mod named;
mod natural;
mod nonnatural;
mod one_handed;
mod two_handed;
mod worn;

use std::ops::Deref;

pub(in crate::weapons) use handless::{HandlessArtifactWeapon, HandlessArtifactWeaponNoAttunement};
pub use one_handed::OneHandedArtifactWeapon;
pub use two_handed::TwoHandedArtifactWeapon;
pub(in crate::weapons) use nonnatural::{NonnaturalArtifactWeapon, NonnaturalArtifactWeaponNoAttunement};
pub use natural::NaturalArtifactWeapon;
pub use worn::WornArtifactWeapon;
pub use base::BaseArtifactWeapon;

use self::named::NamedArtifactWeapon;

pub enum ArtifactWeapon<'source> {
    Natural(NaturalArtifactWeapon<'source>),
    Worn(WornArtifactWeapon<'source>),
    OneHanded(OneHandedArtifactWeapon<'source>),
    TwoHanded(TwoHandedArtifactWeapon<'source>),
}

impl<'source> Deref for ArtifactWeapon<'source> {
    type Target = NamedArtifactWeapon<'source>;

    fn deref(&self) -> &Self::Target {
        match self {
            ArtifactWeapon::Natural(deref) => deref,
            ArtifactWeapon::Worn(deref) => deref,
            ArtifactWeapon::OneHanded(deref) => deref,
            ArtifactWeapon::TwoHanded(deref) => deref,
        }
    }
}