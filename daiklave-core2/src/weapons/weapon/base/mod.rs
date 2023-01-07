pub mod builder;
mod id;

use std::collections::HashSet;

pub use id::BaseWeaponId;

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

use super::{
    ability::WeaponAbility, damage_type::WeaponDamageType, range::WeaponRange,
    tag::OptionalWeaponTag, weight_class::WeaponWeightClass, WeaponTag, AttackRange, RangeBand,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseWeapon {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) weight_class: WeaponWeightClass,
    pub(crate) range_bands: WeaponRange,
    pub(crate) primary_ability: WeaponAbility,
    pub(crate) damage_type: WeaponDamageType,
    pub(crate) tags: HashSet<OptionalWeaponTag>,
}

impl BaseWeapon {
    pub fn tags(&self, handedness_tag: WeaponTag) -> std::vec::IntoIter<WeaponTag> {
        let mut output = Vec::new();

        // Natural and Worn always appear in the list
        // OneHanded only appears for Archery weapons
        // TwoHanded does not appear for Archery weapons
        match (self.primary_ability, handedness_tag) {
            (_, WeaponTag::Natural) => {output.push(WeaponTag::Natural);}
            (_, WeaponTag::Worn) => {output.push(WeaponTag::Worn);}
            (WeaponAbility::Archery, WeaponTag::OneHanded) => {output.push(WeaponTag::OneHanded)}
            (WeaponAbility::Archery, _) => {/* Do nothing */}
            (_, WeaponTag::TwoHanded) => {output.push(WeaponTag::TwoHanded);}
            _ => {/* Do nothing */}
        }

        output.push(self.damage_type.into());

        // Thrown and Archery are handled via RangeBands
        // This is so that weapons can be Martial Arts + Thrown
        match self.primary_ability {
            WeaponAbility::Brawl => {output.push(WeaponTag::Brawl);}
            WeaponAbility::Melee => {output.push(WeaponTag::Melee);}
            WeaponAbility::MartialArts => {output.push(WeaponTag::MartialArts);}
            _ => {/* Do nothing */}
        }

        match self.range_bands {
            WeaponRange::ContactOnly => {/* Do nothing */}
            WeaponRange::Throwable(range) => {output.push(WeaponTag::Thrown(range));}
            WeaponRange::Archery(range) => {output.push(WeaponTag::Archery(range));}
        }

        for tag in self.tags.iter().copied() {
            output.push(tag.into());
        }

        output.sort();

        output.into_iter()
    }

    pub fn accuracy(&self, attack_range: AttackRange, is_artifact: bool) -> Option<i8> {
        match (self.primary_ability, self.range_bands, attack_range) {
            (WeaponAbility::Thrown, _, AttackRange::Melee) | (WeaponAbility::Archery, _, AttackRange::Melee) => None,
            (_, WeaponRange::ContactOnly, AttackRange::Ranged(_)) => None,
            (_, _, AttackRange::Melee) => {
                Some(match self.weight_class {
                    WeaponWeightClass::Light => 4,
                    WeaponWeightClass::Medium => 2,
                    WeaponWeightClass::Heavy => 0,
                } + i8::from(is_artifact))
            }
            (_, WeaponRange::Archery(max_range), AttackRange::Ranged(try_range)) => {
                if try_range > max_range {
                    None
                } else {
                    let flame_bonus = 2 * i8::from(try_range == RangeBand::Close && self.tags.contains(&OptionalWeaponTag::Flame));
                    let accuracy_curve = match try_range {
                        RangeBand::Close => -2,
                        RangeBand::Short => 4,
                        RangeBand::Medium => 2,
                        RangeBand::Long => 0,
                        RangeBand::Extreme => -2,
                    };
                    let artifact_bonus = i8::from(is_artifact);
                    Some(accuracy_curve + flame_bonus + artifact_bonus)
                }
            }
            (_, WeaponRange::Throwable(max_range), AttackRange::Ranged(try_range)) => {
                if try_range > max_range {
                    None
                } else {
                    let accuracy_curve = match try_range {
                        RangeBand::Close => 4,
                        RangeBand::Short => 3,
                        RangeBand::Medium => 2,
                        RangeBand::Long => -1,
                        RangeBand::Extreme => -3,
                    };
                    let artifact_bonus = i8::from(is_artifact);
                    Some(accuracy_curve + artifact_bonus)
                }
            }
        }
    }

    pub fn damage(&self, attack_range: AttackRange, is_artifact: bool) -> Option<u8> {
        match (self.primary_ability, self.range_bands, attack_range) {
            (WeaponAbility::Thrown, _, AttackRange::Melee) | (WeaponAbility::Archery, _, AttackRange::Melee) => None,
            (_, WeaponRange::ContactOnly, AttackRange::Ranged(_)) => None,
            (_, _, AttackRange::Melee) => {
                let shield_penalty = 2 * u8::from(self.tags.contains(&OptionalWeaponTag::Shield));
                Some(match self.weight_class {
                    WeaponWeightClass::Light => 7,
                    WeaponWeightClass::Medium => 9,
                    WeaponWeightClass::Heavy => 11,
                } + 3 * u8::from(is_artifact)
                - shield_penalty)
            }
            (_, WeaponRange::Archery(max_range), AttackRange::Ranged(try_range)) => {
                if try_range > max_range {
                    None
                } else {
                    let flame_or_crossbow = 4 * u8::from(self.tags.contains(&OptionalWeaponTag::Crossbow) || self.tags.contains(&OptionalWeaponTag::Flame));
                    let base_damage = if try_range == RangeBand::Close && self.tags.contains(&OptionalWeaponTag::Powerful) {
                        11
                    } else {
                        match self.weight_class {
                            WeaponWeightClass::Light => 7,
                            WeaponWeightClass::Medium => 9,
                            WeaponWeightClass::Heavy => 11,
                        }
                    };
                    let artifact_bonus = 3 * u8::from(is_artifact);

                    Some(base_damage + flame_or_crossbow + artifact_bonus)
                }
            }
            (_, WeaponRange::Throwable(max_range), AttackRange::Ranged(try_range)) => {
                if try_range > max_range {
                    None
                } else {
                    Some(match self.weight_class {
                        WeaponWeightClass::Light => 7,
                        WeaponWeightClass::Medium => 9,
                        WeaponWeightClass::Heavy => 11,
                    } + 3 * u8::from(is_artifact))
                }
            }
        }
    }

    pub fn parry_mod(&self, is_artifact: bool) -> Option<i8> {
        if matches!(self.primary_ability, WeaponAbility::Thrown | WeaponAbility::Archery) {
            None
        } else {
            Some(match self.weight_class {
                WeaponWeightClass::Light => 0,
                WeaponWeightClass::Medium => 1,
                WeaponWeightClass::Heavy => if is_artifact {0} else {-1}
            })
        }
    }
}