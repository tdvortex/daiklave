pub mod builder;
mod id;

use std::collections::HashSet;

pub use id::BaseWeaponId;

use serde::{Deserialize, Serialize};

use crate::book_reference::BookReference;

use super::{
    ability::WeaponAbility, damage_type::WeaponDamageType, range::WeaponRange,
    tag::OptionalWeaponTag, weight_class::WeaponWeightClass, WeaponTag,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseWeaponMemo {
    pub(crate) name: String,
    pub(crate) book_reference: Option<BookReference>,
    pub(crate) weight_class: WeaponWeightClass,
    pub(crate) range_bands: WeaponRange,
    pub(crate) primary_ability: WeaponAbility,
    pub(crate) damage_type: WeaponDamageType,
    pub(crate) tags: HashSet<OptionalWeaponTag>,
}

impl BaseWeaponMemo {
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
}