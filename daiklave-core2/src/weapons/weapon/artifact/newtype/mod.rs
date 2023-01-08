mod natural;
mod one_handed;
mod two_handed;
mod worn;

pub use natural::NaturalArtifactWeapon;
pub use one_handed::OneHandedArtifactWeapon;
pub use two_handed::TwoHandedArtifactWeapon;
pub use worn::WornArtifactWeapon;

pub(crate) use natural::NaturalArtifactWeaponView;
pub(crate) use one_handed::OneHandedArtifactWeaponView;
pub(crate) use two_handed::TwoHandedArtifactWeaponView;
pub(crate) use worn::WornArtifactWeaponView;
