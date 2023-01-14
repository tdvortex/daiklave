mod caste_ability;
mod memo;
mod supernal_ability;

pub use caste_ability::DawnCasteAbility;
pub(crate) use memo::DawnMemo;
pub use supernal_ability::DawnSupernalAbility;

use crate::abilities::AbilityName;

/// Caste traits for the Dawn Caste Solar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Dawn {
    pub caste_not_supernal: [DawnCasteAbility; 4],
    pub supernal: DawnSupernalAbility,
}

impl Dawn {
    pub(crate) fn as_memo(&self) -> DawnMemo {
        DawnMemo::new(self.caste_not_supernal, self.supernal)
    }

    /// Returns true if the ability is a chosen Caste ability. If Brawl is a
    /// Caste ability, then Martial Arts is also a Caste ability.
    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        let search_ability = if ability == AbilityName::MartialArts {
            AbilityName::Brawl
        } else {
            ability
        };

        self.caste_not_supernal
            .iter()
            .any(|dawn_caste_ability| AbilityName::from(*dawn_caste_ability) == search_ability)
            || AbilityName::from(self.supernal) == search_ability
    }

    /// Returns the Dawn's Supernal ability.
    pub fn supernal_ability(&self) -> AbilityName {
        AbilityName::from(self.supernal)
    }
}
