mod add;
mod remove;

/// Builders for Wonders and Warstriders.
pub mod builder;
mod magic_material;
mod name;
mod sonance;

mod attune;
/// Artifacts which are not weapons, armor, or warstriders.
pub mod wonders;

pub use add::AddArtifact;
pub use attune::AttuneArtifact;
pub use magic_material::MagicMaterial;
pub use name::ArtifactName;
pub(crate) use name::ArtifactNameMutation;
pub use remove::RemoveArtifact;
pub use sonance::Sonance;
