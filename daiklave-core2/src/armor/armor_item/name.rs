/// The name of a piece of armor to be added, removed, equipped, or unequipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmorNameMutation {
    /// Mundane, non-artifact armor.
    Mundane(String),
    /// Artifact armor. This is the name for the specific piece of armor (like
    /// "Brilliant Sentinel"), not the generic item name (like "Articulated
    /// Plate (Artifact)").
    Artifact(String),
}

impl<'source> ArmorNameMutation {
    pub(crate) fn as_ref(&'source self) -> ArmorName<'source> {
        match self {
            ArmorNameMutation::Mundane(name) => ArmorName::Mundane(name.as_str()),
            ArmorNameMutation::Artifact(name) => ArmorName::Artifact(name.as_str()),
        }
    }
}

/// The name of a piece of armor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArmorName<'source> {
    /// Mundane, non-artifact armor.
    Mundane(&'source str),
    /// Artifact armor. This is the name for the specific piece of armor (like
    /// "Brilliant Sentinel"), not the generic item name (like "Articulated
    /// Plate (Artifact)").
    Artifact(&'source str),
}
