use std::collections::HashSet;

type AbilityValue = u8;
type Specialty = String;
type Specialties = HashSet<Specialty>;

// Abilities rated as zero may not have specialties
enum Ability {
    Zero,
    NonZero(NonZeroAbility),
}

impl Default for Ability {
    fn default() -> Self {
        Self::Zero
    }
}

struct NonZeroAbility {
    value: AbilityValue,
    specialties: Option<Specialties>,
}

impl NonZeroAbility {
    fn add_specialty(&mut self, specialty: String) -> bool {
        if let Some(hashset) = &mut self.specialties {
            hashset.insert(specialty)
        } else {
            let mut hashset = HashSet::new();
            hashset.insert(specialty);
            self.specialties = Some(hashset);
            true
        }
    }

    fn remove_specialty(&mut self, specialty: String) -> bool {
        if let Some(hashset) = &mut self.specialties {
            let removed = hashset.remove(&specialty);
            if hashset.is_empty() {
                self.specialties = None;
            }
            removed
        } else {
            false
        }
    }
}

impl Ability {
    fn value(&self) -> AbilityValue {
        match &self {
            Self::Zero => 0,
            Self::NonZero(nonzero) => nonzero.value,
        }
    }

    fn specialties(&self) -> Option<&Specialties> {
        match &self {
            Self::Zero => None,
            Self::NonZero(nonzero) => nonzero.specialties.as_ref(),
        }
    }

    fn set_value(&mut self, new_value: AbilityValue) {
        if new_value == 0 {
            *self = Self::Zero;
        } else if let Self::NonZero(nonzero) = self {
            nonzero.value = new_value;
        } else {
            *self = Self::NonZero(NonZeroAbility {
                value: new_value,
                specialties: None,
            });
        }
    }

    fn add_specialty(&mut self, specialty: String) -> bool {
        if let Self::NonZero(nonzero) = self {
            nonzero.add_specialty(specialty)
        } else {
            false
        }
    }

    fn remove_specialty(&mut self, specialty: String) -> bool {
        if let Self::NonZero(nonzero) = self {
            nonzero.remove_specialty(specialty)
        } else {
            false
        }
    }
}

enum AbilityName {
    Archery,
    Athletics,
    Awareness,
    Brawl,
    Bureaucracy,
    Craft,
    Dodge,
    Integrity,
    Investigation,
    Larcency,
    Linguistics,
    Lore,
    MartialArts,
    Medicine,
    Melee,
    Occult,
    Performance,
    Presence,
    Resistance,
    Ride,
    Sail,
    Socialize,
    Stealth,
    Survival,
    Thrown,
    War,
}

#[derive(Default)]
struct Abilities {
    archery: Ability,
    athletics: Ability,
    awareness: Ability,
    brawl: Ability,
    bureaucracy: Ability,
    craft: Ability,
    dodge: Ability,
    integrity: Ability,
    investigation: Ability,
    larcency: Ability,
    linguistics: Ability,
    lore: Ability,
    martial_arts: Ability,
    medicine: Ability,
    melee: Ability,
    occult: Ability,
    performance: Ability,
    presence: Ability,
    resistance: Ability,
    ride: Ability,
    sail: Ability,
    socialize: Ability,
    stealth: Ability,
    survival: Ability,
    thrown: Ability,
    war: Ability,
}

impl Abilities {
    fn borrow(&self, ability: AbilityName) -> &Ability {
        match ability {
            AbilityName::Archery => &self.archery,
            AbilityName::Athletics => &self.athletics,
            AbilityName::Awareness => &self.awareness,
            AbilityName::Brawl => &self.brawl,
            AbilityName::Bureaucracy => &self.bureaucracy,
            AbilityName::Craft => &self.craft,
            AbilityName::Dodge => &self.dodge,
            AbilityName::Integrity => &self.integrity,
            AbilityName::Investigation => &self.investigation,
            AbilityName::Larcency => &self.larcency,
            AbilityName::Linguistics => &self.linguistics,
            AbilityName::Lore => &self.lore,
            AbilityName::MartialArts => &self.martial_arts,
            AbilityName::Medicine => &self.medicine,
            AbilityName::Melee => &self.melee,
            AbilityName::Occult => &self.occult,
            AbilityName::Performance => &self.performance,
            AbilityName::Presence => &self.presence,
            AbilityName::Resistance => &self.resistance,
            AbilityName::Ride => &self.ride,
            AbilityName::Sail => &self.sail,
            AbilityName::Socialize => &self.socialize,
            AbilityName::Stealth => &self.stealth,
            AbilityName::Survival => &self.survival,
            AbilityName::Thrown => &self.thrown,
            AbilityName::War => &self.war,
        }
    }

    fn borrow_mut(&mut self, ability: AbilityName) -> &mut Ability {
        match ability {
            AbilityName::Archery => &mut self.archery,
            AbilityName::Athletics => &mut self.athletics,
            AbilityName::Awareness => &mut self.awareness,
            AbilityName::Brawl => &mut self.brawl,
            AbilityName::Bureaucracy => &mut self.bureaucracy,
            AbilityName::Craft => &mut self.craft,
            AbilityName::Dodge => &mut self.dodge,
            AbilityName::Integrity => &mut self.integrity,
            AbilityName::Investigation => &mut self.investigation,
            AbilityName::Larcency => &mut self.larcency,
            AbilityName::Linguistics => &mut self.linguistics,
            AbilityName::Lore => &mut self.lore,
            AbilityName::MartialArts => &mut self.martial_arts,
            AbilityName::Medicine => &mut self.medicine,
            AbilityName::Melee => &mut self.melee,
            AbilityName::Occult => &mut self.occult,
            AbilityName::Performance => &mut self.performance,
            AbilityName::Presence => &mut self.presence,
            AbilityName::Resistance => &mut self.resistance,
            AbilityName::Ride => &mut self.ride,
            AbilityName::Sail => &mut self.sail,
            AbilityName::Socialize => &mut self.socialize,
            AbilityName::Stealth => &mut self.stealth,
            AbilityName::Survival => &mut self.survival,
            AbilityName::Thrown => &mut self.thrown,
            AbilityName::War => &mut self.war,
        }
    }

    fn value(&self, ability: AbilityName) -> AbilityValue {
        self.borrow(ability).value()
    }

    fn specialties(&self, ability: AbilityName) -> Option<&Specialties> {
        self.borrow(ability).specialties()
    }

    fn set_value(&mut self, ability: AbilityName, new_value: AbilityValue) {
        self.borrow_mut(ability).set_value(new_value);
    }

    fn add_specialty(&mut self, ability: AbilityName, specialty: String) -> bool {
        self.borrow_mut(ability).add_specialty(specialty)
    }

    fn remove_specialty(&mut self, ability: AbilityName, specialty: String) -> bool {
        self.borrow_mut(ability).remove_specialty(specialty)
    }
}
