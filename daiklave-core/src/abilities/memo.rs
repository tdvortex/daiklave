use serde::{Deserialize, Serialize};

use super::{ability::AbilityRatingMemo, AbilitiesVanilla};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AbilitiesVanillaMemo {
    pub(in crate::abilities) archery: AbilityRatingMemo,
    pub(in crate::abilities) athletics: AbilityRatingMemo,
    pub(in crate::abilities) awareness: AbilityRatingMemo,
    pub(in crate::abilities) brawl: AbilityRatingMemo,
    pub(in crate::abilities) bureaucracy: AbilityRatingMemo,
    pub(in crate::abilities) dodge: AbilityRatingMemo,
    pub(in crate::abilities) integrity: AbilityRatingMemo,
    pub(in crate::abilities) investigation: AbilityRatingMemo,
    pub(in crate::abilities) larceny: AbilityRatingMemo,
    pub(in crate::abilities) linguistics: AbilityRatingMemo,
    pub(in crate::abilities) lore: AbilityRatingMemo,
    pub(in crate::abilities) medicine: AbilityRatingMemo,
    pub(in crate::abilities) melee: AbilityRatingMemo,
    pub(in crate::abilities) occult: AbilityRatingMemo,
    pub(in crate::abilities) performance: AbilityRatingMemo,
    pub(in crate::abilities) presence: AbilityRatingMemo,
    pub(in crate::abilities) resistance: AbilityRatingMemo,
    pub(in crate::abilities) ride: AbilityRatingMemo,
    pub(in crate::abilities) sail: AbilityRatingMemo,
    pub(in crate::abilities) socialize: AbilityRatingMemo,
    pub(in crate::abilities) stealth: AbilityRatingMemo,
    pub(in crate::abilities) survival: AbilityRatingMemo,
    pub(in crate::abilities) thrown: AbilityRatingMemo,
    pub(in crate::abilities) war: AbilityRatingMemo,
}

impl From<&AbilitiesVanilla<'_>> for AbilitiesVanillaMemo {
    fn from(view: &AbilitiesVanilla<'_>) -> Self {
        Self {
            archery: (&view.archery).into(),
            athletics: (&view.athletics).into(),
            awareness: (&view.awareness).into(),
            brawl: (&view.brawl).into(),
            bureaucracy: (&view.bureaucracy).into(),
            dodge: (&view.dodge).into(),
            integrity: (&view.integrity).into(),
            investigation: (&view.investigation).into(),
            larceny: (&view.larceny).into(),
            linguistics: (&view.linguistics).into(),
            lore: (&view.lore).into(),
            medicine: (&view.medicine).into(),
            melee: (&view.melee).into(),
            occult: (&view.occult).into(),
            performance: (&view.performance).into(),
            presence: (&view.presence).into(),
            resistance: (&view.resistance).into(),
            ride: (&view.ride).into(),
            sail: (&view.sail).into(),
            socialize: (&view.socialize).into(),
            stealth: (&view.stealth).into(),
            survival: (&view.survival).into(),
            thrown: (&view.thrown).into(),
            war: (&view.war).into(),
        }
    }
}