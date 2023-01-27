use super::artifact::ArtifactArmorId;

/// The name of a piece of armor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmorName {
    /// Mundane, non-artifact armor.
    Mundane(String),
    /// Artifact armor. This is the name for the specific piece of armor (like
    /// "Brilliant Sentinel"), not the generic item name (like "Articulated
    /// Plate (Artifact)").
    Artifact(ArtifactArmorId),
}
