use serde::{Serialize, Deserialize};

// The name of an ability, excluding any Craft focus areas or Martial Arts styles.
/// This is useful for most Craft Charms and nonspecific combat merits like Quick Draw.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
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

/// The name of an Ability, including a specific Craft focus area or Martial Arts style if appropriate.
/// This is useful for querying a specific ability's dots (e.g. Craft(Masonry) vs Craft(Basketweaving))
/// or its specialties.
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

// The name of any non-Craft, non-Martial Arts ability.
/// This is used to identify all ability ratings that must exist for a character.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
pub enum AbilityNameVanilla {
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