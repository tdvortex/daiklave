use crate::{
    charms::Spell,
    data_source::{BookReference, DataSource},
    id::{CharacterId, Id, SpellId},
};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SorceryCircle {
    Terrestrial,
    Celestial,
    Solar,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TerrestrialCircleSpell(pub Spell);

impl TryFrom<Spell> for TerrestrialCircleSpell {
    type Error = eyre::Report;

    fn try_from(value: Spell) -> Result<Self, Self::Error> {
        if value.circle() != SorceryCircle::Terrestrial {
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
        if value.circle() != SorceryCircle::Celestial {
            Err(eyre!("Spell is not Celestial-level"))
        } else {
            Ok(CelestialCircleSpell(value))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SolarCircleSpell(pub Spell);

impl TryFrom<Spell> for SolarCircleSpell {
    type Error = eyre::Report;

    fn try_from(value: Spell) -> Result<Self, Self::Error> {
        if value.circle() != SorceryCircle::Solar {
            Err(eyre!("Spell is not Solar-level"))
        } else {
            Ok(SolarCircleSpell(value))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapingRitual {
    id: Id,
    name: String,
    description: String,
    data_source: DataSource,
}

impl ShapingRitual {
    pub fn from_book(id: Id, book_title: String, page_number: i16) -> ShapingRitualBuilder {
        ShapingRitualBuilder {
            id,
            data_source: DataSource::Book(BookReference {
                book_title,
                page_number,
            }),
            name: None,
            description: None,
        }
    }

    pub fn custom(id: Id, creator_id: CharacterId) -> ShapingRitualBuilder {
        ShapingRitualBuilder {
            id,
            data_source: DataSource::Custom(creator_id),
            name: None,
            description: None,
        }
    }
}

pub struct ShapingRitualBuilder {
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
            Ok(ShapingRitual {
                id: self.id,
                name: self.name.unwrap(),
                description: self.description.unwrap(),
                data_source: self.data_source,
            })
        }
    }
}

pub trait Sorcerer {
    fn shaping_rituals(&self) -> Option<Vec<&ShapingRitual>>;
    fn spells(&self) -> Option<Vec<(&Spell, bool)>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerrestrialCircleTraits {
    shaping_ritual: ShapingRitual,
    control_spell: TerrestrialCircleSpell,
    other_spells: Vec<TerrestrialCircleSpell>,
}

impl TerrestrialCircleTraits {
    pub fn new(
        shaping_ritual: ShapingRitual,
        control_spell: Spell,
    ) -> Result<TerrestrialCircleTraits> {
        Ok(TerrestrialCircleTraits {
            shaping_ritual,
            control_spell: control_spell.try_into()?,
            other_spells: Vec::new(),
        })
    }

    pub fn add_spell(&mut self, spell: Spell) -> Result<()> {
        if spell == self.control_spell.0 {
            return Ok(());
        }

        let terrestrial_spell = spell.try_into()?;
        self.other_spells.push(terrestrial_spell);
        self.other_spells.sort_by(|a, b| a.0.name().cmp(b.0.name()));
        self.other_spells.dedup();
        Ok(())
    }

    pub fn remove_spell(&mut self, spell_id: SpellId) -> Result<()> {
        if self.control_spell.0.id() == spell_id {
            return Err(eyre!("Cannot remove control spell"));
        }

        self.other_spells
            .retain(|terrestrial| terrestrial.0.id() != spell_id);
        Ok(())
    }

    pub fn swap_control_spell(&mut self, new_control_spell_id: SpellId) -> Result<()> {
        if self.control_spell.0.id() == new_control_spell_id {
            return Ok(());
        }

        let remove_index = self
            .other_spells
            .iter()
            .enumerate()
            .find_map(|(index, terrestrial)| {
                if terrestrial.0.id() == new_control_spell_id {
                    Some(index)
                } else {
                    None
                }
            })
            .ok_or_else(|| eyre!("Spell id {} is not known", **new_control_spell_id))?;

        let mut swap_spell = self.other_spells.remove(remove_index);
        std::mem::swap(&mut self.control_spell, &mut swap_spell);
        self.add_spell(swap_spell.0)
    }

    pub fn swap_shaping_ritual(&mut self, new_shaping_ritual: ShapingRitual) {
        self.shaping_ritual = new_shaping_ritual;
    }
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

impl CelestialCircleTraits {
    pub fn new(
        shaping_ritual: ShapingRitual,
        control_spell: Spell,
    ) -> Result<CelestialCircleTraits> {
        Ok(CelestialCircleTraits {
            shaping_ritual,
            control_spell: control_spell.try_into()?,
            other_spells: Vec::new(),
        })
    }

    pub fn add_spell(&mut self, spell: Spell) -> Result<()> {
        if spell == self.control_spell.0 {
            return Ok(());
        }

        let celestial_spell = spell.try_into()?;
        self.other_spells.push(celestial_spell);
        self.other_spells.sort_by(|a, b| a.0.name().cmp(b.0.name()));
        self.other_spells.dedup();
        Ok(())
    }

    pub fn remove_spell(&mut self, spell_id: SpellId) -> Result<()> {
        if self.control_spell.0.id() == spell_id {
            return Err(eyre!("Cannot remove control spell"));
        }

        self.other_spells
            .retain(|celestial| celestial.0.id() != spell_id);
        Ok(())
    }

    pub fn swap_control_spell(&mut self, new_control_spell_id: SpellId) -> Result<()> {
        if self.control_spell.0.id() == new_control_spell_id {
            return Ok(());
        }
        let remove_index = self
            .other_spells
            .iter()
            .enumerate()
            .find_map(|(index, celestial)| {
                if celestial.0.id() == new_control_spell_id {
                    Some(index)
                } else {
                    None
                }
            })
            .ok_or_else(|| eyre!("Spell id {} is not known", **new_control_spell_id))?;

        let mut swap_spell = self.other_spells.remove(remove_index);
        std::mem::swap(&mut self.control_spell, &mut swap_spell);
        self.add_spell(swap_spell.0)
    }

    pub fn swap_shaping_ritual(&mut self, new_shaping_ritual: ShapingRitual) {
        self.shaping_ritual = new_shaping_ritual;
    }
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
    control_spell: SolarCircleSpell,
    other_spells: Vec<SolarCircleSpell>,
}

impl SolarCircleTraits {
    pub fn new(shaping_ritual: ShapingRitual, control_spell: Spell) -> Result<SolarCircleTraits> {
        Ok(SolarCircleTraits {
            shaping_ritual,
            control_spell: control_spell.try_into()?,
            other_spells: Vec::new(),
        })
    }

    pub fn add_spell(&mut self, spell: Spell) -> Result<()> {
        if spell == self.control_spell.0 {
            return Ok(());
        }

        let solar_spell = spell.try_into()?;
        self.other_spells.push(solar_spell);
        self.other_spells.sort_by(|a, b| a.0.name().cmp(b.0.name()));
        self.other_spells.dedup();
        Ok(())
    }

    pub fn remove_spell(&mut self, spell_id: SpellId) -> Result<()> {
        if self.control_spell.0.id() == spell_id {
            return Err(eyre!("Cannot remove control spell"));
        }

        self.other_spells.retain(|solar| solar.0.id() != spell_id);
        Ok(())
    }

    pub fn swap_control_spell(&mut self, new_control_spell_id: SpellId) -> Result<()> {
        if self.control_spell.0.id() == new_control_spell_id {
            return Ok(());
        }

        let remove_index = self
            .other_spells
            .iter()
            .enumerate()
            .find_map(|(index, solar)| {
                if solar.0.id() == new_control_spell_id {
                    Some(index)
                } else {
                    None
                }
            })
            .ok_or_else(|| eyre!("Spell id {} is not known", **new_control_spell_id))?;

        let mut swap_spell = self.other_spells.remove(remove_index);
        std::mem::swap(&mut self.control_spell, &mut swap_spell);
        self.add_spell(swap_spell.0)
    }

    pub fn swap_shaping_ritual(&mut self, new_shaping_ritual: ShapingRitual) {
        self.shaping_ritual = new_shaping_ritual;
    }
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

impl MortalSorcererLevel {
    pub fn add_spell(&mut self, spell: Spell) -> Result<()> {
        match (&spell.circle(), self) {
            (_, MortalSorcererLevel::None) => Err(eyre!("Not a sorcerer, cannot learn spells")),
            (SorceryCircle::Terrestrial, MortalSorcererLevel::Terrestrial(terrestrial_traits)) => {
                terrestrial_traits.add_spell(spell)
            }
            (_, _) => Err(eyre!("Spell is too high level to be learned")),
        }
    }
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

enum LunarSorcererLevel {
    None,
    _Terrestrial(TerrestrialCircleTraits),
    _Celestial(TerrestrialCircleTraits, CelestialCircleTraits),
}

impl LunarSorcererLevel {
    pub fn _add_spell(&mut self, spell: Spell) -> Result<()> {
        match (&spell.circle(), self) {
            (_, LunarSorcererLevel::None) => Err(eyre!("Not a sorcerer, cannot learn spells")),
            (SorceryCircle::Terrestrial, LunarSorcererLevel::_Terrestrial(terrestrial_traits))
            | (SorceryCircle::Terrestrial, LunarSorcererLevel::_Celestial(terrestrial_traits, _)) => {
                terrestrial_traits.add_spell(spell)
            }
            (SorceryCircle::Celestial, LunarSorcererLevel::_Celestial(_, celestial_traits)) => {
                celestial_traits.add_spell(spell)
            }
            (_, _) => Err(eyre!("Spell is too high level to be learned")),
        }
    }
}

impl Default for LunarSorcererLevel {
    fn default() -> Self {
        Self::None
    }
}

impl Sorcerer for LunarSorcererLevel {
    fn shaping_rituals(&self) -> Option<Vec<&ShapingRitual>> {
        match self {
            LunarSorcererLevel::None => None,
            LunarSorcererLevel::_Terrestrial(terrestrial_traits) => {
                terrestrial_traits.shaping_rituals()
            }
            LunarSorcererLevel::_Celestial(terrestrial_traits, celestial_traits) => {
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
            LunarSorcererLevel::None => None,
            LunarSorcererLevel::_Terrestrial(terrestrial_traits) => terrestrial_traits.spells(),
            LunarSorcererLevel::_Celestial(terrestrial_traits, celestial_traits) => {
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

impl DragonBloodedSorcererLevel {
    pub fn _add_spell(&mut self, spell: Spell) -> Result<()> {
        match (&spell.circle(), self) {
            (_, DragonBloodedSorcererLevel::None) => {
                Err(eyre!("Not a sorcerer, cannot learn spells"))
            }
            (
                SorceryCircle::Terrestrial,
                DragonBloodedSorcererLevel::_Terrestrial(terrestrial_traits),
            ) => terrestrial_traits.add_spell(spell),
            (_, _) => Err(eyre!("Spell is too high level to be learned")),
        }
    }
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

impl SolarSorcererLevel {
    pub fn add_spell(&mut self, spell: Spell) -> Result<()> {
        match (&spell.circle(), self) {
            (_, SolarSorcererLevel::None) => Err(eyre!("Not a sorcerer, cannot learn spells")),
            (SorceryCircle::Terrestrial, SolarSorcererLevel::Terrestrial(terrestrial_traits))
            | (SorceryCircle::Terrestrial, SolarSorcererLevel::Celestial(terrestrial_traits, _))
            | (SorceryCircle::Terrestrial, SolarSorcererLevel::Solar(terrestrial_traits, _, _)) => {
                terrestrial_traits.add_spell(spell)
            }
            (SorceryCircle::Celestial, SolarSorcererLevel::Celestial(_, celestial_traits))
            | (SorceryCircle::Celestial, SolarSorcererLevel::Solar(_, celestial_traits, _)) => {
                celestial_traits.add_spell(spell)
            }
            (SorceryCircle::Solar, SolarSorcererLevel::Solar(_, _, solar_traits)) => {
                solar_traits.add_spell(spell)
            }
            (_, _) => Err(eyre!("Spell is too high level to be learned")),
        }
    }
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
