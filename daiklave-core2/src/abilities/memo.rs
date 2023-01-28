use serde::{Deserialize, Serialize};

use super::{AbilitiesVanilla, ability::AbilityRatingMemo};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AbilitiesMemo {
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

impl<'source> AbilitiesMemo {
    pub fn as_ref(&'source self) -> AbilitiesVanilla<'source> {
        AbilitiesVanilla {
            archery: self.archery.as_ref(),
            athletics: self.athletics.as_ref(),
            awareness: self.awareness.as_ref(),
            brawl: self.brawl.as_ref(),
            bureaucracy: self.bureaucracy.as_ref(),
            dodge: self.dodge.as_ref(),
            integrity: self.integrity.as_ref(),
            investigation: self.investigation.as_ref(),
            larceny: self.larceny.as_ref(),
            linguistics: self.linguistics.as_ref(),
            lore: self.lore.as_ref(),
            medicine: self.medicine.as_ref(),
            melee: self.melee.as_ref(),
            occult: self.occult.as_ref(),
            performance: self.performance.as_ref(),
            presence: self.presence.as_ref(),
            resistance: self.resistance.as_ref(),
            ride: self.ride.as_ref(),
            sail: self.sail.as_ref(),
            socialize: self.socialize.as_ref(),
            stealth: self.stealth.as_ref(),
            survival: self.survival.as_ref(),
            thrown: self.thrown.as_ref(),
            war: self.war.as_ref(),
        }
    }
}
