use crate::{charms::Spell, data_source::{DataSource, BookReference}, id::Id};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SpellLevel {
    Terrestrial,
    Celestial,
    Solar,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TerrestrialCircleSpell(pub Spell);

impl TryFrom<Spell> for TerrestrialCircleSpell {
    type Error = eyre::Report;

    fn try_from(value: Spell) -> Result<Self, Self::Error> {
        if value.circle() != SpellLevel::Terrestrial {
            Err(eyre!("Spell is not Terrestrial-level"))
        } else {
            Ok(TerrestrialCircleSpell(value))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct CelestialCircleSpell(pub Spell);

impl TryFrom<Spell> for CelestialCircleSpell {
    type Error = eyre::Report;

    fn try_from(value: Spell) -> Result<Self, Self::Error> {
        if value.circle() != SpellLevel::Celestial {
            Err(eyre!("Spell is not Celestial-level"))
        } else {
            Ok(CelestialCircleSpell(value))
        }
    }
}
struct SolarCircleSpell(pub Spell);

impl TryFrom<Spell> for SolarCircleSpell {
    type Error = eyre::Report;

    fn try_from(value: Spell) -> Result<Self, Self::Error> {
        if value.circle() != SpellLevel::Solar {
            Err(eyre!("Spell is not Solar-level"))
        } else {
            Ok(SolarCircleSpell(value))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ShapingRitual {
    id: Id,
    name: String,
    description: String,
    data_source: DataSource,
}

impl ShapingRitual {
    pub fn from_book(id: Id, book_title: String, page_number: i16) -> ShapingRitualBuilder {
        ShapingRitualBuilder { id, data_source: DataSource::Book(BookReference{book_title, page_number}), name: None, description: None }
    }

    pub fn custom(id: Id, creator_id: Id) -> ShapingRitualBuilder {
        ShapingRitualBuilder { id, data_source: DataSource::Custom(creator_id), name: None, description: None }
    }
}

struct ShapingRitualBuilder {
    id: Id,
    data_source: DataSource,
    name: Option<String>,
    description: Option<String>,
}

impl ShapingRitualBuilder {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn build(self) -> Result<ShapingRitual> {
        if self.name.is_none() {
            Err(eyre!("Shaping Ritual name is required"))
        } else if self.description.is_none() {
            Err(eyre!("Shaping ritual description is required"))
        } else {
            Ok(ShapingRitual{
                id: self.id,
                name: self.name.unwrap(),
                description: self.description.unwrap(),
                data_source: self.data_source
            })
        }
    }
}

trait Sorcerer {
    fn shaping_rituals(&self) -> Option<Vec<&ShapingRitual>>;
    fn spells(&self) -> Option<Vec<(&Spell, bool)>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerrestrialCircleTraits {
    shaping_ritual: ShapingRitual,
    control_spell: TerrestrialCircleSpell,
    other_spells: Vec<TerrestrialCircleSpell>,
}

impl Sorcerer for TerrestrialCircleTraits {
    fn shaping_rituals(&self) -> Option<Vec<&ShapingRitual>> {
        Some(vec![&self.shaping_ritual])
    }

    fn spells(&self) -> Option<Vec<(&Spell, bool)>> {
        let mut output: Vec<(&Spell, bool)> = self
            .other_spells
            .iter()
            .map(|terrestrial| (&terrestrial.0, false))
            .collect();
        output.push((&self.control_spell.0, true));
        output.sort_by(|a, b| a.0.name().cmp(b.0.name()));
        Some(output)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CelestialCircleTraits {
    shaping_ritual: ShapingRitual,
    control_spell: CelestialCircleSpell,
    other_spells: Vec<CelestialCircleSpell>,
}

impl Sorcerer for CelestialCircleTraits {
    fn shaping_rituals(&self) -> Option<Vec<&ShapingRitual>> {
        Some(vec![&self.shaping_ritual])
    }

    fn spells(&self) -> Option<Vec<(&Spell, bool)>> {
        let mut output: Vec<(&Spell, bool)> = self
            .other_spells
            .iter()
            .map(|celestial| (&celestial.0, false))
            .collect();
        output.push((&self.control_spell.0, true));
        output.sort_by(|a, b| a.0.name().cmp(b.0.name()));
        Some(output)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolarCircleTraits {
    shaping_ritual: ShapingRitual,
    control_spell: CelestialCircleSpell,
    other_spells: Vec<CelestialCircleSpell>,
}

impl Sorcerer for SolarCircleTraits {
    fn shaping_rituals(&self) -> Option<Vec<&ShapingRitual>> {
        Some(vec![&self.shaping_ritual])
    }

    fn spells(&self) -> Option<Vec<(&Spell, bool)>> {
        let mut output: Vec<(&Spell, bool)> = self
            .other_spells
            .iter()
            .map(|solar| (&solar.0, false))
            .collect();
        output.push((&self.control_spell.0, true));
        output.sort_by(|a, b| a.0.name().cmp(b.0.name()));
        Some(output)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MortalSorcererLevel {
    None,
    Terrestrial(TerrestrialCircleTraits),
}

impl Default for MortalSorcererLevel {
    fn default() -> Self {
        Self::None
    }
}

impl Sorcerer for MortalSorcererLevel {
    fn shaping_rituals(&self) -> Option<Vec<&ShapingRitual>> {
        match self {
            MortalSorcererLevel::None => None,
            MortalSorcererLevel::Terrestrial(terrestrial_traits) => {
                terrestrial_traits.shaping_rituals()
            }
        }
    }

    fn spells(&self) -> Option<Vec<(&Spell, bool)>> {
        match self {
            MortalSorcererLevel::None => None,
            MortalSorcererLevel::Terrestrial(terrestrial_traits) => terrestrial_traits.spells(),
        }
    }
}

enum _LunarSorcererLevel {
    None,
    _Terrestrial(TerrestrialCircleTraits),
    _Celestial(TerrestrialCircleTraits, CelestialCircleTraits),
}

impl Default for _LunarSorcererLevel {
    fn default() -> Self {
        Self::None
    }
}

impl Sorcerer for _LunarSorcererLevel {
    fn shaping_rituals(&self) -> Option<Vec<&ShapingRitual>> {
        match self {
            _LunarSorcererLevel::None => None,
            _LunarSorcererLevel::_Terrestrial(terrestrial_traits) => {
                terrestrial_traits.shaping_rituals()
            }
            _LunarSorcererLevel::_Celestial(terrestrial_traits, celestial_traits) => {
                let mut output: Vec<&ShapingRitual> = terrestrial_traits
                    .shaping_rituals()
                    .unwrap()
                    .into_iter()
                    .chain(celestial_traits.shaping_rituals().unwrap().into_iter())
                    .collect();
                output.sort_by(|a, b| a.name.cmp(&b.name));
                Some(output)
            }
        }
    }

    fn spells(&self) -> Option<Vec<(&Spell, bool)>> {
        match self {
            _LunarSorcererLevel::None => None,
            _LunarSorcererLevel::_Terrestrial(terrestrial_traits) => terrestrial_traits.spells(),
            _LunarSorcererLevel::_Celestial(terrestrial_traits, celestial_traits) => {
                let mut output: Vec<(&Spell, bool)> = terrestrial_traits
                    .spells()
                    .unwrap()
                    .into_iter()
                    .chain(celestial_traits.spells().unwrap().into_iter())
                    .collect();
                output.sort_by(|a, b| a.0.name().cmp(b.0.name()));
                Some(output)
            }
        }
    }
}

enum DragonBloodedSorcererLevel {
    None,
    _Terrestrial(TerrestrialCircleTraits),
}

impl Default for DragonBloodedSorcererLevel {
    fn default() -> Self {
        Self::None
    }
}

impl Sorcerer for DragonBloodedSorcererLevel {
    fn shaping_rituals(&self) -> Option<Vec<&ShapingRitual>> {
        match self {
            DragonBloodedSorcererLevel::None => None,
            DragonBloodedSorcererLevel::_Terrestrial(terrestrial_traits) => {
                terrestrial_traits.shaping_rituals()
            }
        }
    }

    fn spells(&self) -> Option<Vec<(&Spell, bool)>> {
        match self {
            DragonBloodedSorcererLevel::None => None,
            DragonBloodedSorcererLevel::_Terrestrial(terrestrial_traits) => {
                terrestrial_traits.spells()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolarSorcererLevel {
    None,
    Terrestrial(TerrestrialCircleTraits),
    Celestial(TerrestrialCircleTraits, CelestialCircleTraits),
    Solar(
        TerrestrialCircleTraits,
        CelestialCircleTraits,
        SolarCircleTraits,
    ),
}

impl Default for SolarSorcererLevel {
    fn default() -> Self {
        Self::None
    }
}

impl Sorcerer for SolarSorcererLevel {
    fn shaping_rituals(&self) -> Option<Vec<&ShapingRitual>> {
        match self {
            SolarSorcererLevel::None => None,
            SolarSorcererLevel::Terrestrial(terrestrial_traits) => {
                terrestrial_traits.shaping_rituals()
            }
            SolarSorcererLevel::Celestial(terrestrial_traits, celestial_traits) => {
                let mut output: Vec<&ShapingRitual> = terrestrial_traits
                    .shaping_rituals()
                    .unwrap()
                    .into_iter()
                    .chain(celestial_traits.shaping_rituals().unwrap().into_iter())
                    .collect();
                output.sort_by(|a, b| a.name.cmp(&b.name));
                Some(output)
            }
            SolarSorcererLevel::Solar(terrestrial_traits, celestial_traits, solar_traits) => {
                let mut output: Vec<&ShapingRitual> = terrestrial_traits
                    .shaping_rituals()
                    .unwrap()
                    .into_iter()
                    .chain(celestial_traits.shaping_rituals().unwrap().into_iter())
                    .chain(solar_traits.shaping_rituals().unwrap().into_iter())
                    .collect();
                output.sort_by(|a, b| a.name.cmp(&b.name));
                Some(output)
            }
        }
    }

    fn spells(&self) -> Option<Vec<(&Spell, bool)>> {
        match self {
            SolarSorcererLevel::None => None,
            SolarSorcererLevel::Terrestrial(terrestrial_traits) => terrestrial_traits.spells(),
            SolarSorcererLevel::Celestial(terrestrial_traits, celestial_traits) => {
                let mut output: Vec<(&Spell, bool)> = terrestrial_traits
                    .spells()
                    .unwrap()
                    .into_iter()
                    .chain(celestial_traits.spells().unwrap().into_iter())
                    .collect();
                output.sort_by(|a, b| a.0.name().cmp(b.0.name()));
                Some(output)
            }
            SolarSorcererLevel::Solar(terrestrial_traits, celestial_traits, solar_traits) => {
                let mut output: Vec<(&Spell, bool)> = terrestrial_traits
                    .spells()
                    .unwrap()
                    .into_iter()
                    .chain(celestial_traits.spells().unwrap().into_iter())
                    .chain(solar_traits.spells().unwrap().into_iter())
                    .collect();
                output.sort_by(|a, b| a.0.name().cmp(b.0.name()));
                Some(output)
            }
        }
    }
}
