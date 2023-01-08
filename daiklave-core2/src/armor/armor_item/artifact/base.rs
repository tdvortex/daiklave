use crate::armor::armor_item::base::BaseArmor;

/// The base piece of artifact armor. Note that while many armor items have the
/// same name as their non-artifact equivalents (like "Chain Shirt"), some
/// armor items are only available as artifacts (like Silken Armor).
pub struct BaseArtifactArmor(pub(crate) BaseArmor);