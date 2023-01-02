mod base;
mod handless;
mod named;
mod natural;
mod nonnatural;
mod one_handed;
mod two_handed;
mod worn;

pub(in crate::weapons) use handless::{HandlessArtifactWeapon, HandlessArtifactWeaponNoAttunement};
pub(in crate::weapons) use one_handed::OneHandedArtifactWeapon;
pub(in crate::weapons) use two_handed::TwoHandedArtifactWeapon;
pub(in crate::weapons) use nonnatural::{NonnaturalArtifactWeapon, NonnaturalArtifactWeaponNoAttunement};