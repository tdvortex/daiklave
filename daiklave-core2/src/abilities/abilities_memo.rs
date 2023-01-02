use serde::{Deserialize, Serialize};

use super::{ability_memo::AbilityMemo, AbilitiesVanilla};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AbilitiesMemo {
    pub(in crate::abilities) archery: AbilityMemo,
    pub(in crate::abilities) athletics: AbilityMemo,
    pub(in crate::abilities) awareness: AbilityMemo,
    pub(in crate::abilities) brawl: AbilityMemo,
    pub(in crate::abilities) bureaucracy: AbilityMemo,
    pub(in crate::abilities) dodge: AbilityMemo,
    pub(in crate::abilities) integrity: AbilityMemo,
    pub(in crate::abilities) investigation: AbilityMemo,
    pub(in crate::abilities) larceny: AbilityMemo,
    pub(in crate::abilities) linguistics: AbilityMemo,
    pub(in crate::abilities) lore: AbilityMemo,
    pub(in crate::abilities) medicine: AbilityMemo,
    pub(in crate::abilities) melee: AbilityMemo,
    pub(in crate::abilities) occult: AbilityMemo,
    pub(in crate::abilities) performance: AbilityMemo,
    pub(in crate::abilities) presence: AbilityMemo,
    pub(in crate::abilities) resistance: AbilityMemo,
    pub(in crate::abilities) ride: AbilityMemo,
    pub(in crate::abilities) sail: AbilityMemo,
    pub(in crate::abilities) socialize: AbilityMemo,
    pub(in crate::abilities) stealth: AbilityMemo,
    pub(in crate::abilities) survival: AbilityMemo,
    pub(in crate::abilities) thrown: AbilityMemo,
    pub(in crate::abilities) war: AbilityMemo,
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
