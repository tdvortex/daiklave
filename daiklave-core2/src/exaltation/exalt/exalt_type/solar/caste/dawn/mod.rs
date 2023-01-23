mod anima_effect;
mod caste_ability;
mod memo;
mod supernal_ability;

pub use caste_ability::DawnCasteAbility;
pub(crate) use caste_ability::DawnCasteAbilityNoBrawl;
pub(crate) use memo::DawnMemo;
pub use supernal_ability::DawnSupernalAbility;
pub(crate) use supernal_ability::DawnSupernalLayout;

use crate::{abilities::AbilityName, exaltation::exalt::AnimaEffect};

use self::anima_effect::{DAWN_ONE, DAWN_TWO, DAWN_THREE};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Dawn {
    pub layout: DawnSupernalLayout,
}

impl Dawn {
    pub(crate) fn as_memo(&self) -> DawnMemo {
        DawnMemo {
            layout: self.layout,
        }
    }

    pub fn has_caste_ability(&self, ability: AbilityName) -> bool {
        match (self.layout, ability) {
            (DawnSupernalLayout::Brawl(_), AbilityName::MartialArts)
            | (DawnSupernalLayout::Brawl(_), AbilityName::Brawl)
            | (DawnSupernalLayout::MartialArts(_), AbilityName::MartialArts)
            | (DawnSupernalLayout::MartialArts(_), AbilityName::Brawl) => true,
            (DawnSupernalLayout::Other(caste, _), AbilityName::Brawl)
            | (DawnSupernalLayout::Other(caste, _), AbilityName::MartialArts) => {
                caste.contains(&DawnCasteAbility::Brawl)
            }
            (DawnSupernalLayout::Brawl(no_brawl), other_ability)
            | (DawnSupernalLayout::MartialArts(no_brawl), other_ability) => {
                let search_ability = match other_ability {
                    AbilityName::Archery => DawnCasteAbilityNoBrawl::Archery,
                    AbilityName::Awareness => DawnCasteAbilityNoBrawl::Awareness,
                    AbilityName::Dodge => DawnCasteAbilityNoBrawl::Dodge,
                    AbilityName::Melee => DawnCasteAbilityNoBrawl::Melee,
                    AbilityName::Resistance => DawnCasteAbilityNoBrawl::Resistance,
                    AbilityName::Thrown => DawnCasteAbilityNoBrawl::Thrown,
                    AbilityName::War => DawnCasteAbilityNoBrawl::War,
                    _ => {
                        return false;
                    }
                };

                no_brawl.contains(&search_ability)
            }
            (DawnSupernalLayout::Other(caste, supernal), other_ability) => {
                let is_supernal = match (supernal, other_ability) {
                    (DawnCasteAbilityNoBrawl::Archery, AbilityName::Archery) => true,
                    (DawnCasteAbilityNoBrawl::Awareness, AbilityName::Awareness) => true,
                    (DawnCasteAbilityNoBrawl::Dodge, AbilityName::Dodge) => true,
                    (DawnCasteAbilityNoBrawl::Melee, AbilityName::Melee) => true,
                    (DawnCasteAbilityNoBrawl::Resistance, AbilityName::Resistance) => true,
                    (DawnCasteAbilityNoBrawl::Thrown, AbilityName::Thrown) => true,
                    (DawnCasteAbilityNoBrawl::War, AbilityName::War) => true,
                    (_, _) => false,
                };

                is_supernal
                    || caste
                        .iter()
                        .any(|caste_ability| AbilityName::from(*caste_ability) == other_ability)
            }
        }
    }

    pub fn supernal_ability(&self) -> AbilityName {
        match self.layout {
            DawnSupernalLayout::MartialArts(_) => AbilityName::MartialArts,
            DawnSupernalLayout::Brawl(_) => AbilityName::Brawl,
            DawnSupernalLayout::Other(_, supernal) => match supernal {
                DawnCasteAbilityNoBrawl::Archery => AbilityName::Archery,
                DawnCasteAbilityNoBrawl::Awareness => AbilityName::Awareness,
                DawnCasteAbilityNoBrawl::Dodge => AbilityName::Dodge,
                DawnCasteAbilityNoBrawl::Melee => AbilityName::Melee,
                DawnCasteAbilityNoBrawl::Resistance => AbilityName::Resistance,
                DawnCasteAbilityNoBrawl::Thrown => AbilityName::Thrown,
                DawnCasteAbilityNoBrawl::War => AbilityName::War,
            },
        }
    }

    pub fn anima_effects(&self) -> [AnimaEffect<'static>; 3] {
        [
            DAWN_ONE,
            DAWN_TWO,
            DAWN_THREE
        ]
    }
}
