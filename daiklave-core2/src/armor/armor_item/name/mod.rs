mod mutation;
pub use mutation::ArmorNameMutation;

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

impl<'source> From<&'source ArmorNameMutation> for ArmorName<'source> {
    fn from(name: &'source ArmorNameMutation) -> Self {
        match name {
            ArmorNameMutation::Mundane(name) => Self::Mundane(name),
            ArmorNameMutation::Artifact(name) => Self::Artifact(name),
        }
    }
}