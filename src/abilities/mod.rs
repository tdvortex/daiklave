use serde::{Serialize, Deserialize};
pub(crate) mod update;
pub use update::AbilitiesDiff;
pub(crate) mod tables;
use crate::prerequisite::AbilityPrerequisite;
use eyre::{eyre, Report, Result};
use std::collections::hash_map::Keys;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::iter::FusedIterator;

#[derive(Debug, Serialize, Deserialize)]
enum AbilityRating {
    Zero,
    NonZero(NonZeroAbility),
}

impl Default for AbilityRating {
    fn default() -> Self {
        Self::Zero
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct NonZeroAbility {
    value: u8,
    specialties: HashSet<String>,
}

/// The name of an ability, excluding any Craft focus areas or Martial Arts styles.
/// This is useful for most Craft Charms and nonspecific combat merits like Quick Draw.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum AbilityNameNoSubskill {
    /// The Archery ability
    Archery,
    /// The Athletics ability
    Athletics,
    /// The Awareness ability
    Awareness,
    /// The Brawl ability
    Brawl,
    /// The Bureaucracy ability
    Bureaucracy,
    /// The Craft ability, irrespective of focus area
    Craft,
    /// The Dodge ability
    Dodge,
    /// The Integrity ability
    Integrity,
    /// The Investigation ability
    Investigation,
    /// The Larceny ability
    Larceny,
    /// The Linguistics ability
    Linguistics,
    /// The Lore ability
    Lore,
    /// The MartialArts ability, irrespective of style
    MartialArts,
    /// The Medicine ability
    Medicine,
    /// The Melee ability
    Melee,
    /// The Occult ability
    Occult,
    /// The Performance ability
    Performance,
    /// The Presence ability
    Presence,
    /// The Resistance ability
    Resistance,
    /// The Ride ability
    Ride,
    /// The Sail ability
    Sail,
    /// The Socialize ability
    Socialize,
    /// The Stealth ability
    Stealth,
    /// The Survival ability
    Survival,
    /// The Thrown ability
    Thrown,
    /// The War ability
    War,
}

impl AbilityNameNoSubskill {
    fn iter() -> AbilityNameNoFocusIter {
        AbilityNameNoFocusIter {
            next_ability_name: Some(AbilityNameNoSubskill::Archery),
        }
    }
}

struct AbilityNameNoFocusIter {
    next_ability_name: Option<AbilityNameNoSubskill>,
}

impl Iterator for AbilityNameNoFocusIter {
    type Item = AbilityNameNoSubskill;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.next_ability_name {
            Some(AbilityNameNoSubskill::Archery) => Some(AbilityNameNoSubskill::Athletics),
            Some(AbilityNameNoSubskill::Athletics) => Some(AbilityNameNoSubskill::Awareness),
            Some(AbilityNameNoSubskill::Awareness) => Some(AbilityNameNoSubskill::Brawl),
            Some(AbilityNameNoSubskill::Brawl) => Some(AbilityNameNoSubskill::Bureaucracy),
            Some(AbilityNameNoSubskill::Bureaucracy) => Some(AbilityNameNoSubskill::Craft),
            Some(AbilityNameNoSubskill::Craft) => Some(AbilityNameNoSubskill::Dodge),
            Some(AbilityNameNoSubskill::Dodge) => Some(AbilityNameNoSubskill::Integrity),
            Some(AbilityNameNoSubskill::Integrity) => Some(AbilityNameNoSubskill::Investigation),
            Some(AbilityNameNoSubskill::Investigation) => Some(AbilityNameNoSubskill::Larceny),
            Some(AbilityNameNoSubskill::Larceny) => Some(AbilityNameNoSubskill::Linguistics),
            Some(AbilityNameNoSubskill::Linguistics) => Some(AbilityNameNoSubskill::Lore),
            Some(AbilityNameNoSubskill::Lore) => Some(AbilityNameNoSubskill::MartialArts),
            Some(AbilityNameNoSubskill::MartialArts) => Some(AbilityNameNoSubskill::Medicine),
            Some(AbilityNameNoSubskill::Medicine) => Some(AbilityNameNoSubskill::Melee),
            Some(AbilityNameNoSubskill::Melee) => Some(AbilityNameNoSubskill::Occult),
            Some(AbilityNameNoSubskill::Occult) => Some(AbilityNameNoSubskill::Performance),
            Some(AbilityNameNoSubskill::Performance) => Some(AbilityNameNoSubskill::Presence),
            Some(AbilityNameNoSubskill::Presence) => Some(AbilityNameNoSubskill::Resistance),
            Some(AbilityNameNoSubskill::Resistance) => Some(AbilityNameNoSubskill::Ride),
            Some(AbilityNameNoSubskill::Ride) => Some(AbilityNameNoSubskill::Sail),
            Some(AbilityNameNoSubskill::Sail) => Some(AbilityNameNoSubskill::Socialize),
            Some(AbilityNameNoSubskill::Socialize) => Some(AbilityNameNoSubskill::Stealth),
            Some(AbilityNameNoSubskill::Stealth) => Some(AbilityNameNoSubskill::Survival),
            Some(AbilityNameNoSubskill::Survival) => Some(AbilityNameNoSubskill::Thrown),
            Some(AbilityNameNoSubskill::Thrown) => Some(AbilityNameNoSubskill::War),
            Some(AbilityNameNoSubskill::War) => None,
            None => None,
        };
        let out = self.next_ability_name;
        self.next_ability_name = next;
        out
    }
}

impl FusedIterator for AbilityNameNoFocusIter {}

/// The name of an Ability, including a specific Craft focus area or Martial Arts style if appropriate.
/// This is useful for querying a specific ability's dots (e.g. Craft(Masonry) vs Craft(Basketweaving))
/// or for specific Charm requirements (like most Martial Arts style Charms).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AbilityName<'a> {
    /// The Archery ability
    Archery,
    /// The Athletics ability
    Athletics,
    /// The Awareness ability
    Awareness,
    /// The Brawl ability
    Brawl,
    /// The Bureaucracy ability
    Bureaucracy,
    /// The Craft ability, augmented with a specific focus area (such as Craft("Artifacts"))
    Craft(&'a str),
    /// The Dodge ability
    Dodge,
    /// The Integrity ability
    Integrity,
    /// The Investigation ability
    Investigation,
    /// The Larceny ability
    Larceny,
    /// The Linguistics ability
    Linguistics,
    /// The Lore ability
    Lore,
    /// The MartialArts ability, augmented with a specific style (such as MartialArts("Crane Style"))
    MartialArts(&'a str),
    /// The Medicine ability
    Medicine,
    /// The Melee ability
    Melee,
    /// The Occult ability
    Occult,
    /// The Performance ability
    Performance,
    /// The Presence ability
    Presence,
    /// The Resistance ability
    Resistance,
    /// The Ride ability
    Ride,
    /// The Sail ability
    Sail,
    /// The Socialize ability
    Socialize,
    /// The Stealth ability
    Stealth,
    /// The Survival ability
    Survival,
    /// The Thrown ability
    Thrown,
    /// The War ability
    War,
}

impl<'a> AbilityName<'a> {
    pub fn subskill(&self) -> Option<&str> {
        match self {
            AbilityName::Craft(focus) => Some(*focus),
            AbilityName::MartialArts(style) => Some(*style),
            _ => None,
        }
    }

    pub fn without_subskill(&self) -> AbilityNameNoSubskill {
        match self {
            AbilityName::Archery => AbilityNameNoSubskill::Archery,
            AbilityName::Athletics => AbilityNameNoSubskill::Athletics,
            AbilityName::Awareness => AbilityNameNoSubskill::Awareness,
            AbilityName::Brawl => AbilityNameNoSubskill::Brawl,
            AbilityName::Bureaucracy => AbilityNameNoSubskill::Bureaucracy,
            AbilityName::Craft(_) => AbilityNameNoSubskill::Craft,
            AbilityName::Dodge => AbilityNameNoSubskill::Dodge,
            AbilityName::Integrity => AbilityNameNoSubskill::Integrity,
            AbilityName::Investigation => AbilityNameNoSubskill::Investigation,
            AbilityName::Larceny => AbilityNameNoSubskill::Larceny,
            AbilityName::Linguistics => AbilityNameNoSubskill::Linguistics,
            AbilityName::Lore => AbilityNameNoSubskill::Lore,
            AbilityName::MartialArts(_) => AbilityNameNoSubskill::MartialArts,
            AbilityName::Medicine => AbilityNameNoSubskill::Medicine,
            AbilityName::Melee => AbilityNameNoSubskill::Melee,
            AbilityName::Occult => AbilityNameNoSubskill::Occult,
            AbilityName::Performance => AbilityNameNoSubskill::Performance,
            AbilityName::Presence => AbilityNameNoSubskill::Presence,
            AbilityName::Resistance => AbilityNameNoSubskill::Resistance,
            AbilityName::Ride => AbilityNameNoSubskill::Ride,
            AbilityName::Sail => AbilityNameNoSubskill::Sail,
            AbilityName::Socialize => AbilityNameNoSubskill::Socialize,
            AbilityName::Stealth => AbilityNameNoSubskill::Stealth,
            AbilityName::Survival => AbilityNameNoSubskill::Survival,
            AbilityName::Thrown => AbilityNameNoSubskill::Thrown,
            AbilityName::War => AbilityNameNoSubskill::War,
        }
    }
}

impl<'a> TryFrom<AbilityNameNoSubskill> for AbilityName<'a> {
    type Error = Report;

    fn try_from(value: AbilityNameNoSubskill) -> Result<Self, Self::Error> {
        match value {
            AbilityNameNoSubskill::Archery => Ok(AbilityName::Archery),
            AbilityNameNoSubskill::Athletics => Ok(AbilityName::Athletics),
            AbilityNameNoSubskill::Awareness => Ok(AbilityName::Awareness),
            AbilityNameNoSubskill::Brawl => Ok(AbilityName::Brawl),
            AbilityNameNoSubskill::Bureaucracy => Ok(AbilityName::Bureaucracy),
            AbilityNameNoSubskill::Craft => Err(eyre!("craft ability requires focus")),
            AbilityNameNoSubskill::Dodge => Ok(AbilityName::Dodge),
            AbilityNameNoSubskill::Integrity => Ok(AbilityName::Integrity),
            AbilityNameNoSubskill::Investigation => Ok(AbilityName::Investigation),
            AbilityNameNoSubskill::Larceny => Ok(AbilityName::Larceny),
            AbilityNameNoSubskill::Linguistics => Ok(AbilityName::Linguistics),
            AbilityNameNoSubskill::Lore => Ok(AbilityName::Lore),
            AbilityNameNoSubskill::MartialArts => Err(eyre!("martial arts ability requires style")),
            AbilityNameNoSubskill::Medicine => Ok(AbilityName::Medicine),
            AbilityNameNoSubskill::Melee => Ok(AbilityName::Melee),
            AbilityNameNoSubskill::Occult => Ok(AbilityName::Occult),
            AbilityNameNoSubskill::Performance => Ok(AbilityName::Performance),
            AbilityNameNoSubskill::Presence => Ok(AbilityName::Presence),
            AbilityNameNoSubskill::Resistance => Ok(AbilityName::Resistance),
            AbilityNameNoSubskill::Ride => Ok(AbilityName::Ride),
            AbilityNameNoSubskill::Sail => Ok(AbilityName::Sail),
            AbilityNameNoSubskill::Socialize => Ok(AbilityName::Socialize),
            AbilityNameNoSubskill::Stealth => Ok(AbilityName::Stealth),
            AbilityNameNoSubskill::Survival => Ok(AbilityName::Survival),
            AbilityNameNoSubskill::Thrown => Ok(AbilityName::Thrown),
            AbilityNameNoSubskill::War => Ok(AbilityName::War),
        }
    }
}

impl<'a> std::fmt::Display for AbilityName<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbilityName::Archery => write!(f, "Archery"),
            AbilityName::Athletics => write!(f, "Athletics"),
            AbilityName::Awareness => write!(f, "Awareness"),
            AbilityName::Brawl => write!(f, "Brawl"),
            AbilityName::Bureaucracy => write!(f, "Bureaucracy"),
            AbilityName::Craft(focus) => write!(f, "Craft ({})", *focus),
            AbilityName::Dodge => write!(f, "Dodge"),
            AbilityName::Integrity => write!(f, "Integrity"),
            AbilityName::Investigation => write!(f, "Investigation"),
            AbilityName::Larceny => write!(f, "Larcency"),
            AbilityName::Linguistics => write!(f, "Linguistics"),
            AbilityName::Lore => write!(f, "Lore"),
            AbilityName::MartialArts(focus) => write!(f, "Martial Arts ({})", *focus),
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

#[derive(Default, Debug, Serialize, Deserialize)]
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
    name: AbilityName<'a>,
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

impl Abilities {
    pub fn get(
        &self,
        ability_name_no_subskill: AbilityNameNoSubskill,
        subskill: Option<&str>,
    ) -> Option<Ability> {
        if subskill.is_none()
            && (ability_name_no_subskill == AbilityNameNoSubskill::Craft
                || ability_name_no_subskill == AbilityNameNoSubskill::MartialArts)
        {
            return None;
        }

        if ability_name_no_subskill == AbilityNameNoSubskill::Craft {
            let (key, rating) = self.craft.get_key_value(subskill.unwrap())?;
            return Some(Ability {
                name: AbilityName::Craft(key.as_str()),
                rating,
            });
        }

        if ability_name_no_subskill == AbilityNameNoSubskill::MartialArts {
            let (key, rating) = self.martial_arts.get_key_value(subskill.unwrap())?;
            return Some(Ability {
                name: AbilityName::MartialArts(key.as_str()),
                rating,
            });
        }

        match ability_name_no_subskill {
            AbilityNameNoSubskill::Archery => Some(Ability {
                name: AbilityName::Archery,
                rating: &self.archery,
            }),
            AbilityNameNoSubskill::Athletics => Some(Ability {
                name: AbilityName::Athletics,
                rating: &self.athletics,
            }),
            AbilityNameNoSubskill::Awareness => Some(Ability {
                name: AbilityName::Awareness,
                rating: &self.awareness,
            }),
            AbilityNameNoSubskill::Brawl => Some(Ability {
                name: AbilityName::Brawl,
                rating: &self.brawl,
            }),
            AbilityNameNoSubskill::Bureaucracy => Some(Ability {
                name: AbilityName::Bureaucracy,
                rating: &self.bureaucracy,
            }),
            AbilityNameNoSubskill::Dodge => Some(Ability {
                name: AbilityName::Dodge,
                rating: &self.dodge,
            }),
            AbilityNameNoSubskill::Integrity => Some(Ability {
                name: AbilityName::Integrity,
                rating: &self.integrity,
            }),
            AbilityNameNoSubskill::Investigation => Some(Ability {
                name: AbilityName::Investigation,
                rating: &self.investigation,
            }),
            AbilityNameNoSubskill::Larceny => Some(Ability {
                name: AbilityName::Larceny,
                rating: &self.larcency,
            }),
            AbilityNameNoSubskill::Linguistics => Some(Ability {
                name: AbilityName::Linguistics,
                rating: &self.linguistics,
            }),
            AbilityNameNoSubskill::Lore => Some(Ability {
                name: AbilityName::Lore,
                rating: &self.lore,
            }),
            AbilityNameNoSubskill::Medicine => Some(Ability {
                name: AbilityName::Medicine,
                rating: &self.medicine,
            }),
            AbilityNameNoSubskill::Melee => Some(Ability {
                name: AbilityName::Melee,
                rating: &self.melee,
            }),
            AbilityNameNoSubskill::Occult => Some(Ability {
                name: AbilityName::Occult,
                rating: &self.occult,
            }),
            AbilityNameNoSubskill::Performance => Some(Ability {
                name: AbilityName::Performance,
                rating: &self.performance,
            }),
            AbilityNameNoSubskill::Presence => Some(Ability {
                name: AbilityName::Presence,
                rating: &self.presence,
            }),
            AbilityNameNoSubskill::Resistance => Some(Ability {
                name: AbilityName::Resistance,
                rating: &self.resistance,
            }),
            AbilityNameNoSubskill::Ride => Some(Ability {
                name: AbilityName::Ride,
                rating: &self.ride,
            }),
            AbilityNameNoSubskill::Sail => Some(Ability {
                name: AbilityName::Sail,
                rating: &self.sail,
            }),
            AbilityNameNoSubskill::Socialize => Some(Ability {
                name: AbilityName::Socialize,
                rating: &self.socialize,
            }),
            AbilityNameNoSubskill::Stealth => Some(Ability {
                name: AbilityName::Stealth,
                rating: &self.stealth,
            }),
            AbilityNameNoSubskill::Survival => Some(Ability {
                name: AbilityName::Survival,
                rating: &self.survival,
            }),
            AbilityNameNoSubskill::Thrown => Some(Ability {
                name: AbilityName::Thrown,
                rating: &self.thrown,
            }),
            AbilityNameNoSubskill::War => Some(Ability {
                name: AbilityName::War,
                rating: &self.war,
            }),
            // Covered by guard clauses above
            AbilityNameNoSubskill::Craft => unreachable!(),
            AbilityNameNoSubskill::MartialArts => unreachable!(),
        }
    }

    pub fn set_dots(
        &mut self,
        ability_name_no_subskill: AbilityNameNoSubskill,
        subskill: Option<&str>,
        dots: u8,
    ) -> Result<()> {
        if subskill.is_none()
            && (ability_name_no_subskill == AbilityNameNoSubskill::Craft
                || ability_name_no_subskill == AbilityNameNoSubskill::MartialArts)
        {
            return Err(eyre!("must specify a subskill for Craft or Martial arts"));
        }

        if ability_name_no_subskill == AbilityNameNoSubskill::Craft {
            if dots > 0 && !self.craft.contains_key(subskill.unwrap()) {
                self.craft.insert(
                    subskill.unwrap().to_owned(),
                    AbilityRating::NonZero(NonZeroAbility {
                        value: dots,
                        specialties: HashSet::new(),
                    }),
                );
            } else if dots == 0 && self.craft.contains_key(subskill.unwrap()) {
                self.craft.remove(subskill.unwrap());
            } else if dots > 0 && self.craft.contains_key(subskill.unwrap()) {
                if let AbilityRating::NonZero(non_zero_ability) =
                    self.craft.get_mut(subskill.unwrap()).unwrap()
                {
                    non_zero_ability.value = dots;
                }
            }
            return Ok(());
        }

        if ability_name_no_subskill == AbilityNameNoSubskill::MartialArts {
            if dots > 0 && !self.martial_arts.contains_key(subskill.unwrap()) {
                self.martial_arts.insert(
                    subskill.unwrap().to_owned(),
                    AbilityRating::NonZero(NonZeroAbility {
                        value: dots,
                        specialties: HashSet::new(),
                    }),
                );
            } else if dots == 0 && self.martial_arts.contains_key(subskill.unwrap()) {
                self.martial_arts.remove(subskill.unwrap());
            } else if dots > 0 && self.martial_arts.contains_key(subskill.unwrap()) {
                if let AbilityRating::NonZero(non_zero_ability) =
                    self.martial_arts.get_mut(subskill.unwrap()).unwrap()
                {
                    non_zero_ability.value = dots;
                }
            }
            return Ok(());
        }

        let ptr = match ability_name_no_subskill {
            AbilityNameNoSubskill::Archery => &mut self.archery,
            AbilityNameNoSubskill::Athletics => &mut self.athletics,
            AbilityNameNoSubskill::Awareness => &mut self.awareness,
            AbilityNameNoSubskill::Brawl => &mut self.brawl,
            AbilityNameNoSubskill::Bureaucracy => &mut self.bureaucracy,
            AbilityNameNoSubskill::Dodge => &mut self.dodge,
            AbilityNameNoSubskill::Integrity => &mut self.integrity,
            AbilityNameNoSubskill::Investigation => &mut self.investigation,
            AbilityNameNoSubskill::Larceny => &mut self.larcency,
            AbilityNameNoSubskill::Linguistics => &mut self.linguistics,
            AbilityNameNoSubskill::Lore => &mut self.lore,
            AbilityNameNoSubskill::Medicine => &mut self.medicine,
            AbilityNameNoSubskill::Melee => &mut self.melee,
            AbilityNameNoSubskill::Occult => &mut self.occult,
            AbilityNameNoSubskill::Performance => &mut self.performance,
            AbilityNameNoSubskill::Presence => &mut self.presence,
            AbilityNameNoSubskill::Resistance => &mut self.resistance,
            AbilityNameNoSubskill::Ride => &mut self.ride,
            AbilityNameNoSubskill::Sail => &mut self.sail,
            AbilityNameNoSubskill::Socialize => &mut self.socialize,
            AbilityNameNoSubskill::Stealth => &mut self.stealth,
            AbilityNameNoSubskill::Survival => &mut self.survival,
            AbilityNameNoSubskill::Thrown => &mut self.thrown,
            AbilityNameNoSubskill::War => &mut self.war,
            AbilityNameNoSubskill::Craft => unreachable!(),
            AbilityNameNoSubskill::MartialArts => unreachable!(),
        };

        if dots == 0 {
            *ptr = AbilityRating::Zero;
        } else {
            match ptr {
                AbilityRating::Zero => {
                    *ptr = AbilityRating::NonZero(NonZeroAbility {
                        value: dots,
                        specialties: HashSet::new(),
                    });
                }
                AbilityRating::NonZero(non_zero_ability) => {
                    non_zero_ability.value = dots;
                }
            }
        }
        Ok(())
    }

    pub fn add_specialty(
        &mut self,
        ability_name_no_subskill: AbilityNameNoSubskill,
        subskill: Option<&str>,
        specialty: String,
    ) -> Result<()> {
        if subskill.is_none()
            && (ability_name_no_subskill == AbilityNameNoSubskill::Craft
                || ability_name_no_subskill == AbilityNameNoSubskill::MartialArts)
        {
            return Err(eyre!("must specify a subskill for Craft or Martial arts"));
        }

        let rating_ptr = match ability_name_no_subskill {
            AbilityNameNoSubskill::Archery => &mut self.archery,
            AbilityNameNoSubskill::Athletics => &mut self.athletics,
            AbilityNameNoSubskill::Awareness => &mut self.awareness,
            AbilityNameNoSubskill::Brawl => &mut self.brawl,
            AbilityNameNoSubskill::Bureaucracy => &mut self.bureaucracy,
            AbilityNameNoSubskill::Dodge => &mut self.dodge,
            AbilityNameNoSubskill::Integrity => &mut self.integrity,
            AbilityNameNoSubskill::Investigation => &mut self.investigation,
            AbilityNameNoSubskill::Larceny => &mut self.larcency,
            AbilityNameNoSubskill::Linguistics => &mut self.linguistics,
            AbilityNameNoSubskill::Lore => &mut self.lore,
            AbilityNameNoSubskill::Medicine => &mut self.medicine,
            AbilityNameNoSubskill::Melee => &mut self.melee,
            AbilityNameNoSubskill::Occult => &mut self.occult,
            AbilityNameNoSubskill::Performance => &mut self.performance,
            AbilityNameNoSubskill::Presence => &mut self.presence,
            AbilityNameNoSubskill::Resistance => &mut self.resistance,
            AbilityNameNoSubskill::Ride => &mut self.ride,
            AbilityNameNoSubskill::Sail => &mut self.sail,
            AbilityNameNoSubskill::Socialize => &mut self.socialize,
            AbilityNameNoSubskill::Stealth => &mut self.stealth,
            AbilityNameNoSubskill::Survival => &mut self.survival,
            AbilityNameNoSubskill::Thrown => &mut self.thrown,
            AbilityNameNoSubskill::War => &mut self.war,
            AbilityNameNoSubskill::Craft => self
                .craft
                .get_mut(subskill.unwrap())
                .ok_or_else(|| eyre!("cannot have specialties on 0-rated abilities"))?,
            AbilityNameNoSubskill::MartialArts => self
                .martial_arts
                .get_mut(subskill.unwrap())
                .ok_or_else(|| eyre!("cannot have specialties on 0-rated abilities"))?,
        };

        match rating_ptr {
            AbilityRating::Zero => {
                return Err(eyre!("cannot have specialties on 0-rated abilities"));
            }
            AbilityRating::NonZero(non_zero_rating) => {
                non_zero_rating.specialties.insert(specialty);
            }
        }

        Ok(())
    }

    pub fn remove_specialty(
        &mut self,
        ability_name_no_subskill: AbilityNameNoSubskill,
        subskill: Option<&str>,
        specialty: &str,
    ) -> Result<()> {
        if subskill.is_none()
            && (ability_name_no_subskill == AbilityNameNoSubskill::Craft
                || ability_name_no_subskill == AbilityNameNoSubskill::MartialArts)
        {
            return Err(eyre!("must specify a subskill for Craft or Martial arts"));
        }

        let rating_ptr = match ability_name_no_subskill {
            AbilityNameNoSubskill::Archery => &mut self.archery,
            AbilityNameNoSubskill::Athletics => &mut self.athletics,
            AbilityNameNoSubskill::Awareness => &mut self.awareness,
            AbilityNameNoSubskill::Brawl => &mut self.brawl,
            AbilityNameNoSubskill::Bureaucracy => &mut self.bureaucracy,
            AbilityNameNoSubskill::Dodge => &mut self.dodge,
            AbilityNameNoSubskill::Integrity => &mut self.integrity,
            AbilityNameNoSubskill::Investigation => &mut self.investigation,
            AbilityNameNoSubskill::Larceny => &mut self.larcency,
            AbilityNameNoSubskill::Linguistics => &mut self.linguistics,
            AbilityNameNoSubskill::Lore => &mut self.lore,
            AbilityNameNoSubskill::Medicine => &mut self.medicine,
            AbilityNameNoSubskill::Melee => &mut self.melee,
            AbilityNameNoSubskill::Occult => &mut self.occult,
            AbilityNameNoSubskill::Performance => &mut self.performance,
            AbilityNameNoSubskill::Presence => &mut self.presence,
            AbilityNameNoSubskill::Resistance => &mut self.resistance,
            AbilityNameNoSubskill::Ride => &mut self.ride,
            AbilityNameNoSubskill::Sail => &mut self.sail,
            AbilityNameNoSubskill::Socialize => &mut self.socialize,
            AbilityNameNoSubskill::Stealth => &mut self.stealth,
            AbilityNameNoSubskill::Survival => &mut self.survival,
            AbilityNameNoSubskill::Thrown => &mut self.thrown,
            AbilityNameNoSubskill::War => &mut self.war,
            AbilityNameNoSubskill::Craft => self
                .craft
                .get_mut(subskill.unwrap())
                .ok_or_else(|| eyre!("cannot have specialties on 0-rated abilities"))?,
            AbilityNameNoSubskill::MartialArts => self
                .martial_arts
                .get_mut(subskill.unwrap())
                .ok_or_else(|| eyre!("cannot have specialties on 0-rated abilities"))?,
        };

        match rating_ptr {
            AbilityRating::Zero => {
                return Err(eyre!("cannot have specialties on 0-rated abilities"));
            }
            AbilityRating::NonZero(non_zero_rating) => {
                non_zero_rating.specialties.remove(specialty);
            }
        }

        Ok(())
    }

    fn craft_iter(&self) -> impl Iterator<Item = Ability> {
        CraftIter {
            craft_iter: self.craft.iter(),
        }
    }

    fn martial_arts_iter(&self) -> impl Iterator<Item = Ability> {
        MartialArtsIter {
            martial_arts_iter: self.martial_arts.iter(),
        }
    }

    fn ability_names_iter(&self) -> AbilityNamesIter {
        AbilityNamesIter {
            ability_name_no_focus_iter: AbilityNameNoSubskill::iter(),
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
            (AbilityNameNoSubskill::Craft, Some(focus)) => {
                if let Some(ability) = self.get(AbilityNameNoSubskill::Craft, Some(focus.as_str()))
                {
                    ability.dots() >= prerequisite.dots
                } else {
                    false
                }
            }
            (AbilityNameNoSubskill::Craft, None) => self
                .craft_iter()
                .any(|craft_ability| craft_ability.dots() >= prerequisite.dots),
            (AbilityNameNoSubskill::MartialArts, Some(style)) => {
                if let Some(ability) =
                    self.get(AbilityNameNoSubskill::MartialArts, Some(style.as_str()))
                {
                    ability.dots() >= prerequisite.dots
                } else {
                    false
                }
            }
            (AbilityNameNoSubskill::MartialArts, None) => self
                .martial_arts_iter()
                .any(|martial_arts_ability| martial_arts_ability.dots() >= prerequisite.dots),
            (other_ability, _) => {
                self.get(other_ability, None).unwrap().dots() >= prerequisite.dots
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
    type Item = AbilityName<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.on_craft {
            if let Some(focus) = self.craft_iter.next() {
                return Some(AbilityName::Craft(focus.as_str()));
            } else {
                self.on_craft = false;
            }
        }

        if self.on_martial_arts {
            if let Some(style) = self.martial_arts_iter.next() {
                return Some(AbilityName::MartialArts(style.as_str()));
            } else {
                self.on_martial_arts = false;
            }
        }

        match self.ability_name_no_focus_iter.next() {
            None => None,
            Some(AbilityNameNoSubskill::Craft) => {
                self.on_craft = true;
                self.next()
            }
            Some(AbilityNameNoSubskill::MartialArts) => {
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
        self.abilities
            .get(ability_name.without_subskill(), ability_name.subskill())
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
            Some(Ability {
                name: AbilityName::Craft(focus.as_str()),
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
            Some(Ability {
                name: AbilityName::MartialArts(style.as_str()),
                rating,
            })
        } else {
            None
        }
    }
}
