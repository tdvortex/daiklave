use eyre::{eyre, Result, Report};
use std::collections::hash_map::Keys;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::iter::FusedIterator;

use super::prerequisite::AbilityPrerequisite;

type Specialty = String;

#[derive(Debug)]
enum AbilityRating {
    Zero,
    NonZero(NonZeroAbility),
}

impl Default for AbilityRating {
    fn default() -> Self {
        Self::Zero
    }
}

#[derive(Debug)]
struct NonZeroAbility {
    value: u8,
    specialties: HashSet<Specialty>,
}

impl NonZeroAbility {
    fn add_specialty(&mut self, specialty: String) -> Result<()> {
        if self.specialties.insert(specialty) {
            Ok(())
        } else {
            Err(eyre!("specialty already exists"))
        }
    }

    fn remove_specialty(&mut self, specialty: &str) -> Result<()> {
        if self.specialties.remove(specialty) {
            Ok(())
        } else {
            Err(eyre!("specialty \"{}\" does not exist", specialty))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AbilityNameNoFocus {
    Archery,
    Athletics,
    Awareness,
    Brawl,
    Bureaucracy,
    Craft,
    Dodge,
    Integrity,
    Investigation,
    Larceny,
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
            Some(AbilityNameNoFocus::Bureaucracy) => Some(AbilityNameNoFocus::Craft),
            Some(AbilityNameNoFocus::Craft) => Some(AbilityNameNoFocus::Dodge),
            Some(AbilityNameNoFocus::Dodge) => Some(AbilityNameNoFocus::Integrity),
            Some(AbilityNameNoFocus::Integrity) => Some(AbilityNameNoFocus::Investigation),
            Some(AbilityNameNoFocus::Investigation) => Some(AbilityNameNoFocus::Larceny),
            Some(AbilityNameNoFocus::Larceny) => Some(AbilityNameNoFocus::Linguistics),
            Some(AbilityNameNoFocus::Linguistics) => Some(AbilityNameNoFocus::Lore),
            Some(AbilityNameNoFocus::Lore) => Some(AbilityNameNoFocus::MartialArts),
            Some(AbilityNameNoFocus::MartialArts) => Some(AbilityNameNoFocus::Medicine),
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

impl FusedIterator for AbilityNameNoFocusIter {}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    Larceny,
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

impl TryFrom<AbilityNameNoFocus> for AbilityName {
    type Error = Report;

    fn try_from(value: AbilityNameNoFocus) -> Result<Self, Self::Error> {
        match value {
            AbilityNameNoFocus::Archery => Ok(AbilityName::Archery),
            AbilityNameNoFocus::Athletics => Ok(AbilityName::Athletics),
            AbilityNameNoFocus::Awareness => Ok(AbilityName::Awareness),
            AbilityNameNoFocus::Brawl => Ok(AbilityName::Brawl),
            AbilityNameNoFocus::Bureaucracy => Ok(AbilityName::Bureaucracy),
            AbilityNameNoFocus::Craft => Err(eyre!("craft ability requires focus")),
            AbilityNameNoFocus::Dodge => Ok(AbilityName::Dodge),
            AbilityNameNoFocus::Integrity => Ok(AbilityName::Integrity),
            AbilityNameNoFocus::Investigation => Ok(AbilityName::Investigation),
            AbilityNameNoFocus::Larceny => Ok(AbilityName::Larceny),
            AbilityNameNoFocus::Linguistics => Ok(AbilityName::Linguistics),
            AbilityNameNoFocus::Lore => Ok(AbilityName::Lore),
            AbilityNameNoFocus::MartialArts => Err(eyre!("martial arts ability requires style")),
            AbilityNameNoFocus::Medicine => Ok(AbilityName::Medicine),
            AbilityNameNoFocus::Melee => Ok(AbilityName::Melee),
            AbilityNameNoFocus::Occult => Ok(AbilityName::Occult),
            AbilityNameNoFocus::Performance => Ok(AbilityName::Performance),
            AbilityNameNoFocus::Presence => Ok(AbilityName::Presence),
            AbilityNameNoFocus::Resistance => Ok(AbilityName::Resistance),
            AbilityNameNoFocus::Ride => Ok(AbilityName::Ride),
            AbilityNameNoFocus::Sail => Ok(AbilityName::Sail),
            AbilityNameNoFocus::Socialize => Ok(AbilityName::Socialize),
            AbilityNameNoFocus::Stealth => Ok(AbilityName::Stealth),
            AbilityNameNoFocus::Survival => Ok(AbilityName::Survival),
            AbilityNameNoFocus::Thrown => Ok(AbilityName::Thrown),
            AbilityNameNoFocus::War => Ok(AbilityName::War),
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
            AbilityName::Larceny => write!(f, "Larcency"),
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
    archery: AbilityRating,
    athletics: AbilityRating,
    awareness: AbilityRating,
    brawl: AbilityRating,
    bureaucracy: AbilityRating,
    craft: HashMap<String, AbilityRating>,
    dodge: AbilityRating,
    integrity: AbilityRating,
    investigation: AbilityRating,
    larcency: AbilityRating,
    linguistics: AbilityRating,
    lore: AbilityRating,
    martial_arts: HashMap<String, AbilityRating>,
    medicine: AbilityRating,
    melee: AbilityRating,
    occult: AbilityRating,
    performance: AbilityRating,
    presence: AbilityRating,
    resistance: AbilityRating,
    ride: AbilityRating,
    sail: AbilityRating,
    socialize: AbilityRating,
    stealth: AbilityRating,
    survival: AbilityRating,
    thrown: AbilityRating,
    war: AbilityRating,
}

pub struct Ability<'a> {
    name: AbilityName,
    rating: &'a AbilityRating,
}

impl<'a> Ability<'a> {
    pub fn name(&self) -> &AbilityName {
        &self.name
    }

    pub fn dots(&self) -> u8 {
        match &self.rating {
            AbilityRating::Zero => 0,
            AbilityRating::NonZero(non_zero_ability) => non_zero_ability.value,
        }
    }

    pub fn specialties(&self) -> Option<&HashSet<String>> {
        match &self.rating {
            AbilityRating::Zero => None,
            AbilityRating::NonZero(non_zero_rating) => {
                if non_zero_rating.specialties.is_empty() {
                    None
                } else {
                    Some(&non_zero_rating.specialties)
                }
            }
        }
    }
}

pub struct AbilityMut<'a> {
    name: AbilityName,
    rating: &'a mut AbilityRating,
}

impl<'a> AbilityMut<'a> {
    pub fn name(&self) -> &AbilityName {
        &self.name
    }

    pub fn dots(&self) -> u8 {
        match &self.rating {
            AbilityRating::Zero => 0,
            AbilityRating::NonZero(non_zero_ability) => non_zero_ability.value,
        }
    }

    pub fn specialties(&self) -> Option<&HashSet<String>> {
        match &self.rating {
            AbilityRating::Zero => None,
            AbilityRating::NonZero(non_zero_rating) => {
                if non_zero_rating.specialties.is_empty() {
                    None
                } else {
                    Some(&non_zero_rating.specialties)
                }
            }
        }
    }

    pub fn set_dots(&mut self, new_dots: u8) {
        match (&mut self.rating, new_dots) {
            (AbilityRating::Zero, 0) => {}
            (AbilityRating::Zero, new_dots) => {
                *self.rating = AbilityRating::NonZero(NonZeroAbility {
                    value: new_dots,
                    specialties: HashSet::new(),
                })
            }
            (AbilityRating::NonZero(_), 0) => *self.rating = AbilityRating::Zero,
            (AbilityRating::NonZero(non_zero_rating), new_dots) => non_zero_rating.value = new_dots,
        }
    }

    pub fn add_specialty(&mut self, specialty: String) -> Result<()> {
        match &mut self.rating {
            AbilityRating::Zero => Err(eyre!("zero-rated abilities cannot have specialties")),
            AbilityRating::NonZero(non_zero_rating) => non_zero_rating.add_specialty(specialty),
        }
    }

    pub fn remove_specialty(&mut self, specialty: &str) -> Result<()> {
        match &mut self.rating {
            AbilityRating::Zero => Err(eyre!("zero-rated abilities have no specialties")),
            AbilityRating::NonZero(non_zero_rating) => non_zero_rating.remove_specialty(specialty),
        }
    }
}

impl Abilities {
    pub fn get(&self, ability_name: &AbilityName) -> Option<Ability> {
        match ability_name {
            AbilityName::Archery => Some(Ability {
                name: ability_name.clone(),
                rating: &self.archery,
            }),
            AbilityName::Athletics => Some(Ability {
                name: ability_name.clone(),
                rating: &self.athletics,
            }),
            AbilityName::Awareness => Some(Ability {
                name: ability_name.clone(),
                rating: &self.awareness,
            }),
            AbilityName::Brawl => Some(Ability {
                name: ability_name.clone(),
                rating: &self.brawl,
            }),
            AbilityName::Bureaucracy => Some(Ability {
                name: ability_name.clone(),
                rating: &self.bureaucracy,
            }),
            AbilityName::Dodge => Some(Ability {
                name: ability_name.clone(),
                rating: &self.dodge,
            }),
            AbilityName::Integrity => Some(Ability {
                name: ability_name.clone(),
                rating: &self.integrity,
            }),
            AbilityName::Investigation => Some(Ability {
                name: ability_name.clone(),
                rating: &self.investigation,
            }),
            AbilityName::Larceny => Some(Ability {
                name: ability_name.clone(),
                rating: &self.larcency,
            }),
            AbilityName::Linguistics => Some(Ability {
                name: ability_name.clone(),
                rating: &self.linguistics,
            }),
            AbilityName::Lore => Some(Ability {
                name: ability_name.clone(),
                rating: &self.lore,
            }),
            AbilityName::Melee => Some(Ability {
                name: ability_name.clone(),
                rating: &self.melee,
            }),
            AbilityName::Medicine => Some(Ability {
                name: ability_name.clone(),
                rating: &self.medicine,
            }),
            AbilityName::Occult => Some(Ability {
                name: ability_name.clone(),
                rating: &self.occult,
            }),
            AbilityName::Performance => Some(Ability {
                name: ability_name.clone(),
                rating: &self.performance,
            }),
            AbilityName::Presence => Some(Ability {
                name: ability_name.clone(),
                rating: &self.presence,
            }),
            AbilityName::Resistance => Some(Ability {
                name: ability_name.clone(),
                rating: &self.resistance,
            }),
            AbilityName::Ride => Some(Ability {
                name: ability_name.clone(),
                rating: &self.ride,
            }),
            AbilityName::Sail => Some(Ability {
                name: ability_name.clone(),
                rating: &self.sail,
            }),
            AbilityName::Socialize => Some(Ability {
                name: ability_name.clone(),
                rating: &self.socialize,
            }),
            AbilityName::Stealth => Some(Ability {
                name: ability_name.clone(),
                rating: &self.stealth,
            }),
            AbilityName::Survival => Some(Ability {
                name: ability_name.clone(),
                rating: &self.survival,
            }),
            AbilityName::Thrown => Some(Ability {
                name: ability_name.clone(),
                rating: &self.thrown,
            }),
            AbilityName::War => Some(Ability {
                name: ability_name.clone(),
                rating: &self.war,
            }),
            AbilityName::Craft(focus) => Some(Ability {
                name: ability_name.clone(),
                rating: self.craft.get(focus)?,
            }),
            AbilityName::MartialArts(style) => Some(Ability {
                name: ability_name.clone(),
                rating: self.martial_arts.get(style)?,
            }),
        }
    }

    pub fn get_mut(&mut self, ability_name: &AbilityName) -> Option<AbilityMut> {
        match ability_name {
            AbilityName::Archery => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.archery,
            }),
            AbilityName::Athletics => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.athletics,
            }),
            AbilityName::Awareness => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.awareness,
            }),
            AbilityName::Brawl => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.brawl,
            }),
            AbilityName::Bureaucracy => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.bureaucracy,
            }),
            AbilityName::Dodge => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.dodge,
            }),
            AbilityName::Integrity => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.integrity,
            }),
            AbilityName::Investigation => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.investigation,
            }),
            AbilityName::Larceny => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.larcency,
            }),
            AbilityName::Linguistics => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.linguistics,
            }),
            AbilityName::Lore => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.lore,
            }),
            AbilityName::Melee => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.melee,
            }),
            AbilityName::Medicine => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.medicine,
            }),
            AbilityName::Occult => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.occult,
            }),
            AbilityName::Performance => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.performance,
            }),
            AbilityName::Presence => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.presence,
            }),
            AbilityName::Resistance => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.resistance,
            }),
            AbilityName::Ride => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.ride,
            }),
            AbilityName::Sail => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.sail,
            }),
            AbilityName::Socialize => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.socialize,
            }),
            AbilityName::Stealth => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.stealth,
            }),
            AbilityName::Survival => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.survival,
            }),
            AbilityName::Thrown => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.thrown,
            }),
            AbilityName::War => Some(AbilityMut {
                name: ability_name.clone(),
                rating: &mut self.war,
            }),
            AbilityName::Craft(focus) => Some(AbilityMut {
                name: ability_name.clone(),
                rating: self.craft.get_mut(focus)?,
            }),
            AbilityName::MartialArts(style) => Some(AbilityMut {
                name: ability_name.clone(),
                rating: self.martial_arts.get_mut(style)?,
            }),
        }
    }

    pub fn add_craft(&mut self, focus: String) -> AbilityMut {
        if !self.craft.contains_key(&focus) {
            self.craft.insert(focus.clone(), AbilityRating::Zero);
        }
        self.get_mut(&AbilityName::Craft(focus)).unwrap()
    }

    pub fn remove_craft(&mut self, focus: &String) {
        self.craft.remove(focus);
    }

    pub fn craft_iter(&self) -> impl Iterator<Item = Ability> {
        CraftIter {
            craft_iter: self.craft.iter(),
        }
    }

    pub fn add_martial_arts(&mut self, style: String) -> AbilityMut {
        if !self.martial_arts.contains_key(&style) {
            self.martial_arts.insert(style.clone(), AbilityRating::Zero);
        }
        self.get_mut(&AbilityName::MartialArts(style)).unwrap()
    }

    pub fn remove_martial_arts(&mut self, style: &String) {
        self.martial_arts.remove(style);
    }

    pub fn martial_arts_iter(&self) -> impl Iterator<Item = Ability> {
        MartialArtsIter {
            martial_arts_iter: self.martial_arts.iter(),
        }
    }

    fn ability_names_iter(&self) -> AbilityNamesIter {
        AbilityNamesIter {
            ability_name_no_focus_iter: AbilityNameNoFocus::iter(),
            on_craft: false,
            craft_iter: self.craft.keys(),
            on_martial_arts: false,
            martial_arts_iter: self.martial_arts.keys(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Ability> {
        AbilitiesIter {
            abilities: self,
            ability_names_iter: self.ability_names_iter(),
        }
    }

    pub fn meets_prerequisite(&self, prerequisite: &AbilityPrerequisite) -> bool {
        match (prerequisite.ability_name, &prerequisite.subskill) {
            (AbilityNameNoFocus::Craft, Some(focus)) => {
                let ability_name: AbilityName = AbilityName::Craft(focus.clone());
                if let Some(ability) = self.get(&ability_name) {
                    ability.dots() >= prerequisite.dots
                } else {
                    false
                }
            }
            (AbilityNameNoFocus::Craft, None) => {
                self.craft_iter().any(|craft_ability| craft_ability.dots() >= prerequisite.dots)
            }
            (AbilityNameNoFocus::MartialArts, Some(style)) => {
                let ability_name: AbilityName = AbilityName::MartialArts(style.clone());
                if let Some(ability) = self.get(&ability_name) {
                    ability.dots() >= prerequisite.dots
                } else {
                    false
                }
            }
            (AbilityNameNoFocus::MartialArts, None) => {
                self.martial_arts_iter().any(|martial_arts_ability| martial_arts_ability.dots() >= prerequisite.dots)
            }
            (other_ability, _) => {
                let ability_name: AbilityName = other_ability.try_into().unwrap();
                self.get(&ability_name).unwrap().dots() >= prerequisite.dots
            }
        }
    }
}

struct AbilityNamesIter<'a> {
    ability_name_no_focus_iter: AbilityNameNoFocusIter,
    on_craft: bool,
    craft_iter: Keys<'a, String, AbilityRating>,
    on_martial_arts: bool,
    martial_arts_iter: Keys<'a, String, AbilityRating>,
}

impl<'a> Iterator for AbilityNamesIter<'a> {
    type Item = AbilityName;

    fn next(&mut self) -> Option<Self::Item> {
        if self.on_craft {
            if let Some(focus) = self.craft_iter.next() {
                return Some(AbilityName::Craft(focus.clone()));
            } else {
                self.on_craft = false;
            }
        }

        if self.on_martial_arts {
            if let Some(style) = self.martial_arts_iter.next() {
                return Some(AbilityName::MartialArts(style.clone()));
            } else {
                self.on_martial_arts = false;
            }
        }

        match self.ability_name_no_focus_iter.next() {
            None => None,
            Some(AbilityNameNoFocus::Craft) => {
                self.on_craft = true;
                self.next()
            }
            Some(AbilityNameNoFocus::MartialArts) => {
                self.on_martial_arts = true;
                self.next()
            }
            Some(other_name) => Some(other_name.try_into().unwrap()),
        }
    }
}

struct AbilitiesIter<'a> {
    abilities: &'a Abilities,
    ability_names_iter: AbilityNamesIter<'a>,
}

impl<'a> Iterator for AbilitiesIter<'a> {
    type Item = Ability<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let ability_name = self.ability_names_iter.next()?;
        self.abilities.get(&ability_name)
    }
}

impl<'a> FusedIterator for AbilitiesIter<'a> {}

struct CraftIter<'a> {
    craft_iter: std::collections::hash_map::Iter<'a, String, AbilityRating>,
}

impl<'a> Iterator for CraftIter<'a> {
    type Item = Ability<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((focus, rating)) = self.craft_iter.next() {
            Some(Ability{
                name: AbilityName::Craft(focus.clone()),
                rating,
            })
        } else {
            None
        }
    }
}

struct MartialArtsIter<'a> {
    martial_arts_iter: std::collections::hash_map::Iter<'a, String, AbilityRating>,
}

impl<'a> Iterator for MartialArtsIter<'a> {
    type Item = Ability<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((style, rating)) = self.martial_arts_iter.next() {
            Some(Ability{
                name: AbilityName::MartialArts(style.clone()),
                rating,
            })
        } else {
            None
        }
    }
}