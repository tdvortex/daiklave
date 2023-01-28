mod mutation;
pub use mutation::AbilityNameQualifiedMutation;

use super::AbilityNameVanilla;

pub enum AbilityNameQualified<'source> {
    Vanilla(AbilityNameVanilla),
    Craft(&'source str),
    MartialArts(&'source str),
}

impl From<AbilityNameVanilla> for AbilityNameQualified<'_> {
    fn from(vanilla: AbilityNameVanilla) -> Self {
        Self::Vanilla(vanilla)
    }
}

impl<'source> From<&'source AbilityNameQualifiedMutation> for AbilityNameQualified<'source> {
    fn from(name: &'source AbilityNameQualifiedMutation) -> Self {
        match name {
            AbilityNameQualifiedMutation::Vanilla(vanilla) => vanilla.into(),
            AbilityNameQualifiedMutation::Craft(_) => todo!(),
            AbilityNameQualifiedMutation::MartialArts(_) => todo!(),
        }
    }
}