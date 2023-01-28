mod mutation;
pub use mutation::WeaponNameMutation;

/// The name of a weapon.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum WeaponName<'source> {
    /// All characters have the Unarmed weapon for free, and it cannot
    /// be removed.
    Unarmed,
    /// A mundane weapon without artifact traits.
    Mundane(&'source str),
    /// A unique magical weapon.
    Artifact(&'source str),
}


impl<'source> From<&'source WeaponNameMutation> for WeaponName<'source> {
    fn from(weapon_name: &'source WeaponNameMutation) -> Self {
        match weapon_name {
            WeaponNameMutation::Unarmed => Self::Unarmed,
            WeaponNameMutation::Mundane(mundane_name) => Self::Mundane(mundane_name.as_str()),
            WeaponNameMutation::Artifact(artifact_name) => Self::Artifact(artifact_name.as_str()),
        }
    }
}