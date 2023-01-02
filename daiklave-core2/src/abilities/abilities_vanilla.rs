use super::{ability::Ability, AbilitiesMemo, AbilityNameVanilla};

/// A struct representing all non-Craft, non-Martial Arts abilities, including
/// any specialties.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AbilitiesVanilla<'source> {
    pub(in crate::abilities) archery: Ability<'source>,
    pub(in crate::abilities) athletics: Ability<'source>,
    pub(in crate::abilities) awareness: Ability<'source>,
    pub(in crate::abilities) brawl: Ability<'source>,
    pub(in crate::abilities) bureaucracy: Ability<'source>,
    pub(in crate::abilities) dodge: Ability<'source>,
    pub(in crate::abilities) integrity: Ability<'source>,
    pub(in crate::abilities) investigation: Ability<'source>,
    pub(in crate::abilities) larceny: Ability<'source>,
    pub(in crate::abilities) linguistics: Ability<'source>,
    pub(in crate::abilities) lore: Ability<'source>,
    pub(in crate::abilities) medicine: Ability<'source>,
    pub(in crate::abilities) melee: Ability<'source>,
    pub(in crate::abilities) occult: Ability<'source>,
    pub(in crate::abilities) performance: Ability<'source>,
    pub(in crate::abilities) presence: Ability<'source>,
    pub(in crate::abilities) resistance: Ability<'source>,
    pub(in crate::abilities) ride: Ability<'source>,
    pub(in crate::abilities) sail: Ability<'source>,
    pub(in crate::abilities) socialize: Ability<'source>,
    pub(in crate::abilities) stealth: Ability<'source>,
    pub(in crate::abilities) survival: Ability<'source>,
    pub(in crate::abilities) thrown: Ability<'source>,
    pub(in crate::abilities) war: Ability<'source>,
}

impl<'source> AbilitiesVanilla<'source> {
    pub(crate) fn as_memo(&self) -> AbilitiesMemo {
        AbilitiesMemo {
            archery: self.archery.as_memo(),
            athletics: self.athletics.as_memo(),
            awareness: self.awareness.as_memo(),
            brawl: self.brawl.as_memo(),
            bureaucracy: self.bureaucracy.as_memo(),
            dodge: self.dodge.as_memo(),
            integrity: self.integrity.as_memo(),
            investigation: self.investigation.as_memo(),
            larceny: self.larceny.as_memo(),
            linguistics: self.linguistics.as_memo(),
            lore: self.lore.as_memo(),
            medicine: self.medicine.as_memo(),
            melee: self.melee.as_memo(),
            occult: self.occult.as_memo(),
            performance: self.performance.as_memo(),
            presence: self.presence.as_memo(),
            resistance: self.resistance.as_memo(),
            ride: self.ride.as_memo(),
            sail: self.sail.as_memo(),
            socialize: self.socialize.as_memo(),
            stealth: self.stealth.as_memo(),
            survival: self.survival.as_memo(),
            thrown: self.thrown.as_memo(),
            war: self.war.as_memo(),
        }
    }

    pub(crate) fn ability(&self, ability_name: AbilityNameVanilla) -> &Ability {
        match ability_name {
            AbilityNameVanilla::Archery => &self.archery,
            AbilityNameVanilla::Athletics => &self.athletics,
            AbilityNameVanilla::Awareness => &self.awareness,
            AbilityNameVanilla::Brawl => &self.brawl,
            AbilityNameVanilla::Bureaucracy => &self.bureaucracy,
            AbilityNameVanilla::Dodge => &self.dodge,
            AbilityNameVanilla::Integrity => &self.integrity,
            AbilityNameVanilla::Investigation => &self.investigation,
            AbilityNameVanilla::Larceny => &self.larceny,
            AbilityNameVanilla::Linguistics => &self.linguistics,
            AbilityNameVanilla::Lore => &self.lore,
            AbilityNameVanilla::Medicine => &self.medicine,
            AbilityNameVanilla::Melee => &self.melee,
            AbilityNameVanilla::Occult => &self.occult,
            AbilityNameVanilla::Performance => &self.performance,
            AbilityNameVanilla::Presence => &self.presence,
            AbilityNameVanilla::Resistance => &self.resistance,
            AbilityNameVanilla::Ride => &self.ride,
            AbilityNameVanilla::Sail => &self.sail,
            AbilityNameVanilla::Socialize => &self.socialize,
            AbilityNameVanilla::Stealth => &self.stealth,
            AbilityNameVanilla::Survival => &self.survival,
            AbilityNameVanilla::Thrown => &self.thrown,
            AbilityNameVanilla::War => &self.war,
        }
    }

    pub(crate) fn ability_mut(
        &mut self,
        ability_name: AbilityNameVanilla,
    ) -> &mut Ability<'source> {
        match ability_name {
            AbilityNameVanilla::Archery => &mut self.archery,
            AbilityNameVanilla::Athletics => &mut self.athletics,
            AbilityNameVanilla::Awareness => &mut self.awareness,
            AbilityNameVanilla::Brawl => &mut self.brawl,
            AbilityNameVanilla::Bureaucracy => &mut self.bureaucracy,
            AbilityNameVanilla::Dodge => &mut self.dodge,
            AbilityNameVanilla::Integrity => &mut self.integrity,
            AbilityNameVanilla::Investigation => &mut self.investigation,
            AbilityNameVanilla::Larceny => &mut self.larceny,
            AbilityNameVanilla::Linguistics => &mut self.linguistics,
            AbilityNameVanilla::Lore => &mut self.lore,
            AbilityNameVanilla::Medicine => &mut self.medicine,
            AbilityNameVanilla::Melee => &mut self.melee,
            AbilityNameVanilla::Occult => &mut self.occult,
            AbilityNameVanilla::Performance => &mut self.performance,
            AbilityNameVanilla::Presence => &mut self.presence,
            AbilityNameVanilla::Resistance => &mut self.resistance,
            AbilityNameVanilla::Ride => &mut self.ride,
            AbilityNameVanilla::Sail => &mut self.sail,
            AbilityNameVanilla::Socialize => &mut self.socialize,
            AbilityNameVanilla::Stealth => &mut self.stealth,
            AbilityNameVanilla::Survival => &mut self.survival,
            AbilityNameVanilla::Thrown => &mut self.thrown,
            AbilityNameVanilla::War => &mut self.war,
        }
    }

    /// Get the dot rating for a specific (non-Craft, non-MA) ability.
    pub fn dots(&self, ability_name: AbilityNameVanilla) -> u8 {
        self.ability(ability_name).dots()
    }

    /// Get an iterator for all specialties associated with a specific ability.
    pub fn specialties(&self, ability_name: AbilityNameVanilla) -> impl Iterator<Item = &str> {
        self.ability(ability_name).specialties()
    }
}
