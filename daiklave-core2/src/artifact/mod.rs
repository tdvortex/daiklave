mod add;

/// Builders for Wonders and Warstriders.
pub mod builder;
mod magic_material;
mod name;
mod sonance;

/// Artifacts which are not weapons, armor, or warstriders.
pub mod wonders;
mod attune;

pub use add::AddArtifact;
pub use attune::AttuneArtifact;
pub use magic_material::MagicMaterial;
pub use name::{ArtifactName, ArtifactNameMutation};
pub use sonance::Sonance;

