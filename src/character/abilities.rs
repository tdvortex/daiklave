use eyre::{eyre, Result};
use std::collections::hash_map::Keys;
use std::collections::{HashMap, HashSet};

pub type AbilityValue = u8;
type Specialty = String;

// Abilities rated as zero may not have specialties
#[derive(Debug)]
pub enum Ability {
    Zero,
    NonZero(NonZeroAbility),
}

impl Default for Ability {
    fn default() -> Self {
        Self::Zero
    }
}

#[derive(Debug)]
pub struct NonZeroAbility {
    value: AbilityValue,
    specialties: HashSet<Specialty>,
}

impl NonZeroAbility {
    fn add_specialty(&mut self, specialty: &str) -> Result<()> {
        if self.specialties.insert(specialty.to_owned()) {
            Ok(())
        } else {
            Err(eyre!("specialty already exists"))
        }
    }

    fn remove_specialty(&mut self, specialty: &str) -> Result<()> {
        if self.specialties.remove(specialty) {
            Ok(())
        } else {
            Err(eyre!("specialy \"{}\" does not exist", specialty))
        }
    }
}

impl Ability {
    pub fn value(&self) -> AbilityValue {
        match &self {
            Self::Zero => 0,
            Self::NonZero(nonzero) => nonzero.value,
        }
    }

    pub fn specialties(&self) -> Option<&HashSet<Specialty>> {
        match &self {
            Self::Zero => None,
            Self::NonZero(nonzero) => Some(&nonzero.specialties),
        }
    }

    pub fn set_value(&mut self, new_value: AbilityValue) {
        if new_value == 0 {
            *self = Self::Zero;
        } else if let Self::NonZero(nonzero) = self {
            nonzero.value = new_value;
        } else {
            *self = Self::NonZero(NonZeroAbility {
                value: new_value,
                specialties: HashSet::new(),
            });
        }
    }

    pub fn add_specialty(&mut self, specialty: String) -> Result<()> {
        if let Self::NonZero(nonzero) = self {
            nonzero.add_specialty(&specialty)
        } else {
            Err(eyre!("cannot add specialty to ability with zero dots"))
        }
    }

    pub fn remove_specialty(&mut self, specialty: String) -> Result<()> {
        if let Self::NonZero(nonzero) = self {
            nonzero.remove_specialty(&specialty)
        } else {
            Err(eyre!("cannot remove specialty from ability with zero dots"))
        }
    }
}

#[derive(Clone, Copy)]
pub enum AbilityNameNoFocus {
    Archery,
    Athletics,
    Awareness,
    Brawl,
    Bureaucracy,
    Dodge,
    Integrity,
    Investigation,
    Larcency,
    Linguistics,
    Lore,
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

impl AbilityNameNoFocus {
    fn iter() -> AbilityNameNoFocusIter {
        AbilityNameNoFocusIter {
            next_ability_name: Some(AbilityNameNoFocus::Archery),
        }
    }
}

struct AbilityNameNoFocusIter {
    next_ability_name: Option<AbilityNameNoFocus>,
}

impl Iterator for AbilityNameNoFocusIter {
    type Item = AbilityNameNoFocus;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.next_ability_name {
            Some(AbilityNameNoFocus::Archery) => Some(AbilityNameNoFocus::Athletics),
            Some(AbilityNameNoFocus::Athletics) => Some(AbilityNameNoFocus::Awareness),
            Some(AbilityNameNoFocus::Awareness) => Some(AbilityNameNoFocus::Brawl),
            Some(AbilityNameNoFocus::Brawl) => Some(AbilityNameNoFocus::Bureaucracy),
            Some(AbilityNameNoFocus::Bureaucracy) => Some(AbilityNameNoFocus::Dodge),
            Some(AbilityNameNoFocus::Dodge) => Some(AbilityNameNoFocus::Integrity),
            Some(AbilityNameNoFocus::Integrity) => Some(AbilityNameNoFocus::Investigation),
            Some(AbilityNameNoFocus::Investigation) => Some(AbilityNameNoFocus::Larcency),
            Some(AbilityNameNoFocus::Larcency) => Some(AbilityNameNoFocus::Linguistics),
            Some(AbilityNameNoFocus::Linguistics) => Some(AbilityNameNoFocus::Lore),
            Some(AbilityNameNoFocus::Lore) => Some(AbilityNameNoFocus::Medicine),
            Some(AbilityNameNoFocus::Medicine) => Some(AbilityNameNoFocus::Melee),
            Some(AbilityNameNoFocus::Melee) => Some(AbilityNameNoFocus::Occult),
            Some(AbilityNameNoFocus::Occult) => Some(AbilityNameNoFocus::Performance),
            Some(AbilityNameNoFocus::Performance) => Some(AbilityNameNoFocus::Presence),
            Some(AbilityNameNoFocus::Presence) => Some(AbilityNameNoFocus::Resistance),
            Some(AbilityNameNoFocus::Resistance) => Some(AbilityNameNoFocus::Ride),
            Some(AbilityNameNoFocus::Ride) => Some(AbilityNameNoFocus::Sail),
            Some(AbilityNameNoFocus::Sail) => Some(AbilityNameNoFocus::Socialize),
            Some(AbilityNameNoFocus::Socialize) => Some(AbilityNameNoFocus::Stealth),
            Some(AbilityNameNoFocus::Stealth) => Some(AbilityNameNoFocus::Survival),
            Some(AbilityNameNoFocus::Survival) => Some(AbilityNameNoFocus::Thrown),
            Some(AbilityNameNoFocus::Thrown) => Some(AbilityNameNoFocus::War),
            Some(AbilityNameNoFocus::War) => None,
            None => None,
        };
        let out = self.next_ability_name;
        self.next_ability_name = next;
        out
    }
}

pub enum AbilityName {
    Archery,
    Athletics,
    Awareness,
    Brawl,
    Bureaucracy,
    Craft(String),
    Dodge,
    Integrity,
    Investigation,
    Larcency,
    Linguistics,
    Lore,
    MartialArts(String),
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

impl AbilityName {
    pub fn craft(focus: String) -> AbilityName {
        AbilityName::Craft(focus)
    }

    pub fn martial_arts(focus: String) -> AbilityName {
        AbilityName::MartialArts(focus)
    }
}

impl From<AbilityNameNoFocus> for AbilityName {
    fn from(no_focus_name: AbilityNameNoFocus) -> Self {
        match no_focus_name {
            AbilityNameNoFocus::Archery => AbilityName::Archery,
            AbilityNameNoFocus::Athletics => AbilityName::Athletics,
            AbilityNameNoFocus::Awareness => AbilityName::Awareness,
            AbilityNameNoFocus::Brawl => AbilityName::Brawl,
            AbilityNameNoFocus::Bureaucracy => AbilityName::Bureaucracy,
            AbilityNameNoFocus::Dodge => AbilityName::Dodge,
            AbilityNameNoFocus::Integrity => AbilityName::Integrity,
            AbilityNameNoFocus::Investigation => AbilityName::Investigation,
            AbilityNameNoFocus::Larcency => AbilityName::Larcency,
            AbilityNameNoFocus::Linguistics => AbilityName::Linguistics,
            AbilityNameNoFocus::Lore => AbilityName::Lore,
            AbilityNameNoFocus::Medicine => AbilityName::Medicine,
            AbilityNameNoFocus::Melee => AbilityName::Melee,
            AbilityNameNoFocus::Occult => AbilityName::Occult,
            AbilityNameNoFocus::Performance => AbilityName::Performance,
            AbilityNameNoFocus::Presence => AbilityName::Presence,
            AbilityNameNoFocus::Resistance => AbilityName::Resistance,
            AbilityNameNoFocus::Ride => AbilityName::Ride,
            AbilityNameNoFocus::Sail => AbilityName::Sail,
            AbilityNameNoFocus::Socialize => AbilityName::Socialize,
            AbilityNameNoFocus::Stealth => AbilityName::Stealth,
            AbilityNameNoFocus::Survival => AbilityName::Survival,
            AbilityNameNoFocus::Thrown => AbilityName::Thrown,
            AbilityNameNoFocus::War => AbilityName::War,
        }
    }
}

impl std::fmt::Display for AbilityName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbilityName::Archery => write!(f, "Archery"),
            AbilityName::Athletics => write!(f, "Athletics"),
            AbilityName::Awareness => write!(f, "Awareness"),
            AbilityName::Brawl => write!(f, "Brawl"),
            AbilityName::Bureaucracy => write!(f, "Bureaucracy"),
            AbilityName::Craft(focus) => write!(f, "Craft ({})", focus),
            AbilityName::Dodge => write!(f, "Dodge"),
            AbilityName::Integrity => write!(f, "Integrity"),
            AbilityName::Investigation => write!(f, "Investigation"),
            AbilityName::Larcency => write!(f, "Larcency"),
            AbilityName::Linguistics => write!(f, "Linguistics"),
            AbilityName::Lore => write!(f, "Lore"),
            AbilityName::MartialArts(focus) => write!(f, "Martial Arts ({})", focus),
            AbilityName::Medicine => write!(f, "Medicine"),
            AbilityName::Melee => write!(f, "Melee"),
            AbilityName::Occult => write!(f, "Occult"),
            AbilityName::Performance => write!(f, "Performance"),
            AbilityName::Presence => write!(f, "Presence"),
            AbilityName::Resistance => write!(f, "LoResistancere"),
            AbilityName::Ride => write!(f, "Ride"),
            AbilityName::Sail => write!(f, "Sail"),
            AbilityName::Socialize => write!(f, "Socialize"),
            AbilityName::Stealth => write!(f, "Stealth"),
            AbilityName::Survival => write!(f, "Survival"),
            AbilityName::Thrown => write!(f, "Thrown"),
            AbilityName::War => write!(f, "War"),
        }
    }
}

#[derive(Default, Debug)]
pub struct Abilities {
    archery: Ability,
    athletics: Ability,
    awareness: Ability,
    brawl: Ability,
    bureaucracy: Ability,
    craft: HashMap<String, Ability>,
    dodge: Ability,
    integrity: Ability,
    investigation: Ability,
    larcency: Ability,
    linguistics: Ability,
    lore: Ability,
    martial_arts: HashMap<String, Ability>,
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
    pub fn get(&self, ability: &AbilityName) -> Option<&Ability> {
        match ability {
            AbilityName::Archery => Some(&self.archery),
            AbilityName::Athletics => Some(&self.athletics),
            AbilityName::Awareness => Some(&self.awareness),
            AbilityName::Brawl => Some(&self.brawl),
            AbilityName::Bureaucracy => Some(&self.bureaucracy),
            AbilityName::Craft(focus) => self.craft.get(focus),
            AbilityName::Dodge => Some(&self.dodge),
            AbilityName::Integrity => Some(&self.integrity),
            AbilityName::Investigation => Some(&self.investigation),
            AbilityName::Larcency => Some(&self.larcency),
            AbilityName::Linguistics => Some(&self.linguistics),
            AbilityName::Lore => Some(&self.lore),
            AbilityName::MartialArts(focus) => self.martial_arts.get(focus),
            AbilityName::Medicine => Some(&self.medicine),
            AbilityName::Melee => Some(&self.melee),
            AbilityName::Occult => Some(&self.occult),
            AbilityName::Performance => Some(&self.performance),
            AbilityName::Presence => Some(&self.presence),
            AbilityName::Resistance => Some(&self.resistance),
            AbilityName::Ride => Some(&self.ride),
            AbilityName::Sail => Some(&self.sail),
            AbilityName::Socialize => Some(&self.socialize),
            AbilityName::Stealth => Some(&self.stealth),
            AbilityName::Survival => Some(&self.survival),
            AbilityName::Thrown => Some(&self.thrown),
            AbilityName::War => Some(&self.war),
        }
    }

    pub fn get_mut(&mut self, ability: &AbilityName) -> Option<&mut Ability> {
        match ability {
            AbilityName::Archery => Some(&mut self.archery),
            AbilityName::Athletics => Some(&mut self.athletics),
            AbilityName::Awareness => Some(&mut self.awareness),
            AbilityName::Brawl => Some(&mut self.brawl),
            AbilityName::Bureaucracy => Some(&mut self.bureaucracy),
            AbilityName::Craft(focus) => self.craft.get_mut(focus),
            AbilityName::Dodge => Some(&mut self.dodge),
            AbilityName::Integrity => Some(&mut self.integrity),
            AbilityName::Investigation => Some(&mut self.investigation),
            AbilityName::Larcency => Some(&mut self.larcency),
            AbilityName::Linguistics => Some(&mut self.linguistics),
            AbilityName::Lore => Some(&mut self.lore),
            AbilityName::MartialArts(focus) => self.martial_arts.get_mut(focus),
            AbilityName::Medicine => Some(&mut self.medicine),
            AbilityName::Melee => Some(&mut self.melee),
            AbilityName::Occult => Some(&mut self.occult),
            AbilityName::Performance => Some(&mut self.performance),
            AbilityName::Presence => Some(&mut self.presence),
            AbilityName::Resistance => Some(&mut self.resistance),
            AbilityName::Ride => Some(&mut self.ride),
            AbilityName::Sail => Some(&mut self.sail),
            AbilityName::Socialize => Some(&mut self.socialize),
            AbilityName::Stealth => Some(&mut self.stealth),
            AbilityName::Survival => Some(&mut self.survival),
            AbilityName::Thrown => Some(&mut self.thrown),
            AbilityName::War => Some(&mut self.war),
        }
    }

    pub fn set_value(&mut self, ability: &AbilityName, new_value: AbilityValue) {
        if let Some(a) = self.get_mut(ability) {
            a.set_value(new_value);
        } else if new_value > 0 {
            match ability {
                AbilityName::Craft(focus) => {
                    self.craft.insert(
                        focus.clone(),
                        Ability::NonZero(NonZeroAbility {
                            value: new_value,
                            specialties: HashSet::new(),
                        }),
                    );
                }
                AbilityName::MartialArts(focus) => {
                    self.martial_arts.insert(
                        focus.clone(),
                        Ability::NonZero(NonZeroAbility {
                            value: new_value,
                            specialties: HashSet::new(),
                        }),
                    );
                }
                // Safety: all other abilities are required and will never return None from borrow_mut()
                _ => unreachable!(),
            }
        }
    }

    fn ability_names_iter(&self) -> AbilityNamesIter {
        AbilityNamesIter {
            ability_name_no_focus_iter: AbilityNameNoFocus::iter(),
            craft_iter: self.craft.keys(),
            martial_arts_iter: self.martial_arts.keys(),
        }
    }

    pub fn iter(&self) -> AbilitiesIter {
        AbilitiesIter {
            abilities: self,
            ability_names_iter: self.ability_names_iter(),
        }
    }
}

struct AbilityNamesIter<'a> {
    ability_name_no_focus_iter: AbilityNameNoFocusIter,
    craft_iter: Keys<'a, String, Ability>,
    martial_arts_iter: Keys<'a, String, Ability>,
}

impl<'a> Iterator for AbilityNamesIter<'a> {
    type Item = AbilityName;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ability_name_no_focus) = self.ability_name_no_focus_iter.next() {
            Some(ability_name_no_focus.into())
        } else if let Some(craft_focus) = self.craft_iter.next() {
            Some(AbilityName::Craft(craft_focus.clone()))
        } else {
            self.martial_arts_iter
                .next()
                .map(|martial_arts_focus| AbilityName::MartialArts(martial_arts_focus.clone()))
        }
    }
}

pub struct AbilitiesIter<'a> {
    abilities: &'a Abilities,
    ability_names_iter: AbilityNamesIter<'a>,
}

impl<'a> Iterator for AbilitiesIter<'a> {
    type Item = &'a Ability;

    fn next(&mut self) -> Option<Self::Item> {
        let ability_name = self.ability_names_iter.next()?;

        self.abilities.get(&ability_name)
    }
}
