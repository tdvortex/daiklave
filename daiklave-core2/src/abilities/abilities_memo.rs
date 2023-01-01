use serde::{Deserialize, Serialize};

use super::{ability_memo::AbilityMemo, AbilitiesView};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AbilitiesMemo {
    archery: AbilityMemo,
    athletics: AbilityMemo,
    awareness: AbilityMemo,
    brawl: AbilityMemo,
    bureaucracy: AbilityMemo,
    dodge: AbilityMemo,
    integrity: AbilityMemo,
    investigation: AbilityMemo,
    larceny: AbilityMemo,
    linguistics: AbilityMemo,
    lore: AbilityMemo,
    medicine: AbilityMemo,
    melee: AbilityMemo,
    occult: AbilityMemo,
    performance: AbilityMemo,
    presence: AbilityMemo,
    resistance: AbilityMemo,
    ride: AbilityMemo,
    sail: AbilityMemo,
    socialize: AbilityMemo,
    stealth: AbilityMemo,
    survival: AbilityMemo,
    thrown: AbilityMemo,
    war: AbilityMemo,
}

impl<'source> AbilitiesMemo {
    pub fn as_ref(&'source self) -> AbilitiesView<'source> {
        AbilitiesView {
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