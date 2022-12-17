use serde::{Deserialize, Serialize};
pub(crate) mod update;
pub use update::AbilitiesDiff;
pub(crate) mod tables;
use eyre::{eyre, Result};
use std::fmt::Debug;
use std::iter::FusedIterator;
pub(crate) mod enums;
pub use enums::{AbilityName, AbilityNameNoSubskill, AbilityNameVanilla};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]

pub(crate) enum AbilityRating {
    Zero,
    NonZero(NonZeroAbility),
}

impl Default for AbilityRating {
    fn default() -> Self {
        Self::Zero
    }
}

impl AbilityRating {
    pub fn dots(&self) -> u8 {
        match self {
            AbilityRating::Zero => 0,
            AbilityRating::NonZero(non_zero_ability) => non_zero_ability.dots,
        }
    }

    pub fn specialties(&self) -> Option<&Vec<String>> {
        match self {
            AbilityRating::Zero => None,
            AbilityRating::NonZero(non_zero_ability) => {
                if non_zero_ability.specialties.is_empty() {
                    None
                } else {
                    Some(&non_zero_ability.specialties)
                }
            }
        }
    }

    pub fn set_dots(&mut self, dots: u8) {
        if self.dots() != dots {
            if dots == 0 {
                *self = AbilityRating::Zero;
            } else if self.dots() == 0 {
                *self = AbilityRating::NonZero(NonZeroAbility {
                    dots,
                    specialties: Vec::new(),
                });
            } else if let Self::NonZero(non_zero_ability) = self {
                non_zero_ability.dots = dots;
            }
        }
    }

    pub fn add_specialty(&mut self, specialty: String) -> Result<()> {
        if let AbilityRating::NonZero(non_zero_ability) = self {
            if non_zero_ability.specialties.contains(&specialty) {
                Err(eyre!("Specialty {} already exists", specialty))
            } else {
                non_zero_ability.specialties.push(specialty);
                non_zero_ability.specialties.sort();
                Ok(())
            }
        } else {
            Err(eyre!("Cannot add specialty to zero-rated abilities"))
        }
    }

    pub fn remove_specialty(&mut self, specialty: &str) -> Result<()> {
        if let AbilityRating::NonZero(non_zero_ability) = self {
            let index = non_zero_ability
                .specialties
                .iter()
                .enumerate()
                .find(|(_, s)| s.as_str() == specialty);

            if let Some((i, _)) = index {
                non_zero_ability.specialties.remove(i);
                Ok(())
            } else {
                Err(eyre!("Specialty {} not found", specialty))
            }
        } else {
            Err(eyre!("Cannot add specialty to zero-rated abilities"))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub(crate) struct NonZeroAbility {
    pub dots: u8,
    pub specialties: Vec<String>,
}

impl AbilityNameVanilla {
    fn iter() -> AbilityNameVanillaIter {
        AbilityNameVanillaIter {
            next_ability_name: Some(AbilityNameVanilla::Archery),
        }
    }
}

struct AbilityNameVanillaIter {
    next_ability_name: Option<AbilityNameVanilla>,
}

impl Iterator for AbilityNameVanillaIter {
    type Item = AbilityNameVanilla;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.next_ability_name {
            Some(AbilityNameVanilla::Archery) => Some(AbilityNameVanilla::Athletics),
            Some(AbilityNameVanilla::Athletics) => Some(AbilityNameVanilla::Awareness),
            Some(AbilityNameVanilla::Awareness) => Some(AbilityNameVanilla::Brawl),
            Some(AbilityNameVanilla::Brawl) => Some(AbilityNameVanilla::Bureaucracy),
            Some(AbilityNameVanilla::Bureaucracy) => Some(AbilityNameVanilla::Dodge),
            Some(AbilityNameVanilla::Dodge) => Some(AbilityNameVanilla::Integrity),
            Some(AbilityNameVanilla::Integrity) => Some(AbilityNameVanilla::Investigation),
            Some(AbilityNameVanilla::Investigation) => Some(AbilityNameVanilla::Larceny),
            Some(AbilityNameVanilla::Larceny) => Some(AbilityNameVanilla::Linguistics),
            Some(AbilityNameVanilla::Linguistics) => Some(AbilityNameVanilla::Lore),
            Some(AbilityNameVanilla::Lore) => Some(AbilityNameVanilla::Medicine),
            Some(AbilityNameVanilla::Medicine) => Some(AbilityNameVanilla::Melee),
            Some(AbilityNameVanilla::Melee) => Some(AbilityNameVanilla::Occult),
            Some(AbilityNameVanilla::Occult) => Some(AbilityNameVanilla::Performance),
            Some(AbilityNameVanilla::Performance) => Some(AbilityNameVanilla::Presence),
            Some(AbilityNameVanilla::Presence) => Some(AbilityNameVanilla::Resistance),
            Some(AbilityNameVanilla::Resistance) => Some(AbilityNameVanilla::Ride),
            Some(AbilityNameVanilla::Ride) => Some(AbilityNameVanilla::Sail),
            Some(AbilityNameVanilla::Sail) => Some(AbilityNameVanilla::Socialize),
            Some(AbilityNameVanilla::Socialize) => Some(AbilityNameVanilla::Stealth),
            Some(AbilityNameVanilla::Stealth) => Some(AbilityNameVanilla::Survival),
            Some(AbilityNameVanilla::Survival) => Some(AbilityNameVanilla::Thrown),
            Some(AbilityNameVanilla::Thrown) => Some(AbilityNameVanilla::War),
            Some(AbilityNameVanilla::War) => None,
            None => None,
        };
        let out = self.next_ability_name;
        self.next_ability_name = next;
        out
    }
}

impl FusedIterator for AbilityNameVanillaIter {}

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

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Abilities {
    archery: AbilityRating,
    athletics: AbilityRating,
    awareness: AbilityRating,
    brawl: AbilityRating,
    bureaucracy: AbilityRating,
    craft: Vec<(String, AbilityRating)>,
    dodge: AbilityRating,
    integrity: AbilityRating,
    investigation: AbilityRating,
    larcency: AbilityRating,
    linguistics: AbilityRating,
    lore: AbilityRating,
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
    pub(crate) name: AbilityName<'a>,
    pub(crate) rating: &'a AbilityRating,
}

impl<'a> Ability<'a> {
    pub fn name(&self) -> &AbilityName {
        &self.name
    }

    pub fn dots(&self) -> u8 {
        self.rating.dots()
    }

    pub fn specialties(&self) -> Option<&Vec<String>> {
        self.rating.specialties()
    }
}

impl Abilities {
    pub fn get(&self, ability_name_vanilla: AbilityNameVanilla) -> Ability {
        match ability_name_vanilla {
            AbilityNameVanilla::Archery => Ability {
                name: AbilityName::Archery,
                rating: &self.archery,
            },
            AbilityNameVanilla::Athletics => Ability {
                name: AbilityName::Athletics,
                rating: &self.athletics,
            },
            AbilityNameVanilla::Awareness => Ability {
                name: AbilityName::Awareness,
                rating: &self.awareness,
            },
            AbilityNameVanilla::Brawl => Ability {
                name: AbilityName::Brawl,
                rating: &self.brawl,
            },
            AbilityNameVanilla::Bureaucracy => Ability {
                name: AbilityName::Bureaucracy,
                rating: &self.bureaucracy,
            },
            AbilityNameVanilla::Dodge => Ability {
                name: AbilityName::Dodge,
                rating: &self.dodge,
            },
            AbilityNameVanilla::Integrity => Ability {
                name: AbilityName::Integrity,
                rating: &self.integrity,
            },
            AbilityNameVanilla::Investigation => Ability {
                name: AbilityName::Investigation,
                rating: &self.investigation,
            },
            AbilityNameVanilla::Larceny => Ability {
                name: AbilityName::Larceny,
                rating: &self.larcency,
            },
            AbilityNameVanilla::Linguistics => Ability {
                name: AbilityName::Linguistics,
                rating: &self.linguistics,
            },
            AbilityNameVanilla::Lore => Ability {
                name: AbilityName::Lore,
                rating: &self.lore,
            },
            AbilityNameVanilla::Medicine => Ability {
                name: AbilityName::Medicine,
                rating: &self.medicine,
            },
            AbilityNameVanilla::Melee => Ability {
                name: AbilityName::Melee,
                rating: &self.melee,
            },
            AbilityNameVanilla::Occult => Ability {
                name: AbilityName::Occult,
                rating: &self.occult,
            },
            AbilityNameVanilla::Performance => Ability {
                name: AbilityName::Performance,
                rating: &self.performance,
            },
            AbilityNameVanilla::Presence => Ability {
                name: AbilityName::Presence,
                rating: &self.presence,
            },
            AbilityNameVanilla::Resistance => Ability {
                name: AbilityName::Resistance,
                rating: &self.resistance,
            },
            AbilityNameVanilla::Ride => Ability {
                name: AbilityName::Ride,
                rating: &self.ride,
            },
            AbilityNameVanilla::Sail => Ability {
                name: AbilityName::Sail,
                rating: &self.sail,
            },
            AbilityNameVanilla::Socialize => Ability {
                name: AbilityName::Socialize,
                rating: &self.socialize,
            },
            AbilityNameVanilla::Stealth => Ability {
                name: AbilityName::Stealth,
                rating: &self.stealth,
            },
            AbilityNameVanilla::Survival => Ability {
                name: AbilityName::Survival,
                rating: &self.survival,
            },
            AbilityNameVanilla::Thrown => Ability {
                name: AbilityName::Thrown,
                rating: &self.thrown,
            },
            AbilityNameVanilla::War => Ability {
                name: AbilityName::War,
                rating: &self.war,
            },
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
            let focus_index = self
                .craft
                .iter()
                .enumerate()
                .find_map(|(index, (focus, _))| {
                    if Some(focus.as_str()) == subskill {
                        Some(index)
                    } else {
                        None
                    }
                });

            if let Some(index) = focus_index {
                if dots == 0 {
                    self.craft.remove(index);
                } else if let AbilityRating::NonZero(non_zero_ability) = &mut self.craft[index].1 {
                    non_zero_ability.dots = dots;
                }
            } else if dots > 0 {
                self.craft.push((
                    subskill.unwrap().to_owned(),
                    AbilityRating::NonZero(NonZeroAbility {
                        dots,
                        specialties: Vec::new(),
                    }),
                ));
                self.craft.sort_by(|a, b| a.0.cmp(&b.0));
            }

            return Ok(());
        }

        if ability_name_no_subskill == AbilityNameNoSubskill::MartialArts {
            return Err(eyre!(
                "Add martial arts dots to MartialArtist, not Abilities"
            ));
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

        ptr.set_dots(dots);
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
                .iter_mut()
                .find_map(|(focus, rating)| {
                    if Some(focus.as_str()) == subskill {
                        Some(rating)
                    } else {
                        None
                    }
                })
                .ok_or_else(|| {
                    eyre!(
                        "Cannot add specialty to zero-rated ability: Craft ({})",
                        subskill.unwrap()
                    )
                })?,
            AbilityNameNoSubskill::MartialArts => {
                return Err(eyre!(
                    "Add martial arts specialties to MartialArtist, not Abilities"
                ));
            }
        };

        rating_ptr.add_specialty(specialty)
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
                .iter_mut()
                .find_map(|(focus, rating)| {
                    if Some(focus.as_str()) == subskill {
                        Some(rating)
                    } else {
                        None
                    }
                })
                .ok_or_else(|| {
                    eyre!(
                        "Cannot have specialties on 0-rated ability: Craft ({})",
                        subskill.unwrap()
                    )
                })?,
            AbilityNameNoSubskill::MartialArts => {
                return Err(eyre!(
                    "Remove martial arts specialties from MartialArtist, not Abilities"
                ));
            }
        };

        rating_ptr.remove_specialty(specialty)
    }

    pub fn iter(&self) -> impl Iterator<Item = Ability> + '_ {
        AbilitiesVanillaIter {
            abilities: self,
            vanilla_names_iter: AbilityNameVanilla::iter(),
        }
    }
}

struct AbilitiesVanillaIter<'a> {
    abilities: &'a Abilities,
    vanilla_names_iter: AbilityNameVanillaIter,
}

impl<'a> Iterator for AbilitiesVanillaIter<'a> {
    type Item = Ability<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let name = self.vanilla_names_iter.next()?;

        Some(self.abilities.get(name))
    }
}
