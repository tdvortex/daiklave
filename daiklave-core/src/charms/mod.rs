use crate::{
    abilities::AbilityNameNoSubskill,
    attributes::AttributeName,
    data_source::{BookReference, DataSource},
    id::{CharacterId, MartialArtsCharmId, MartialArtsStyleId, SolarCharmId, SpellId},
    sorcery::SorceryCircle,
};
use eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum CharmKeyword {
    Air,
    Aggravated,
    Archetype,
    Aura,
    Balanced,
    Bridge,
    Clash,
    Counterattack,
    DecisiveOnly,
    Dual,
    Excellency,
    Fire,
    Earth,
    Mute,
    Pilot,
    Protean,
    Psyche,
    Perilous,
    Ritual,
    Salient,
    Signature,
    Stackable,
    Uniform,
    Water,
    WitheringOnly,
    Wood,
    WrittenOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharmActionType {
    Simple,
    Supplemental,
    Reflexive,
    Permanent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum CharmCostType {
    Motes,
    SorcerousMotes,
    Willpower,
    BashingHealth,
    LethalHealth,
    AggravatedHealth,
    AnimaLevels,
    Initiative,
    Experience,
    SilverCraftExperience,
    GoldCraftExperience,
    WhiteCraftExperience,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CharmTraits {
    data_source: DataSource,
    name: String,
    summary: Option<String>,
    duration: String,
    keywords: Vec<CharmKeyword>,
    costs: Vec<(CharmCostType, u8)>,
    description: String,
}

impl CharmTraits {
    fn from_book(book_title: String, page_number: i16) -> CharmTraitsBuilder {
        CharmTraitsBuilder {
            data_source: DataSource::Book(BookReference {
                book_title,
                page_number,
            }),
            name: None,
            summary: None,
            duration: None,
            keywords: Vec::new(),
            costs: Vec::new(),
            description: None,
        }
    }

    fn custom(creator_id: CharacterId) -> CharmTraitsBuilder {
        CharmTraitsBuilder {
            data_source: DataSource::Custom(creator_id),
            name: None,
            summary: None,
            duration: None,
            keywords: Vec::new(),
            costs: Vec::new(),
            description: None,
        }
    }

    fn data_source(&self) -> &DataSource {
        &self.data_source
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn summary(&self) -> Option<&str> {
        self.summary.as_deref()
    }

    fn duration(&self) -> &str {
        self.duration.as_str()
    }

    fn keywords(&self) -> &Vec<CharmKeyword> {
        &self.keywords
    }

    fn costs(&self) -> &Vec<(CharmCostType, u8)> {
        &self.costs
    }

    fn description(&self) -> &str {
        self.description.as_str()
    }
}

struct CharmTraitsBuilder {
    data_source: DataSource,
    name: Option<String>,
    summary: Option<String>,
    duration: Option<String>,
    keywords: Vec<CharmKeyword>,
    costs: Vec<(CharmCostType, u8)>,
    description: Option<String>,
}

impl CharmTraitsBuilder {
    fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    fn with_summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
        self
    }

    fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    fn with_duration(mut self, duration: String) -> Self {
        self.duration = Some(duration);
        self
    }

    fn with_keyword(mut self, keyword: CharmKeyword) -> Self {
        self.keywords.push(keyword);
        self
    }

    fn with_cost(mut self, cost: CharmCostType, amount: u8) -> Self {
        if amount == 0 {
            return self;
        }

        if let Some((_, old_amount)) = self
            .costs
            .iter_mut()
            .find(|(existing_cost, _)| *existing_cost == cost)
        {
            *old_amount += amount;
        } else {
            self.costs.push((cost, amount));
            self.costs.sort();
        }
        self
    }

    fn build(mut self) -> Result<CharmTraits> {
        self.keywords.sort();
        self.keywords.dedup();

        Ok(CharmTraits {
            data_source: self.data_source,
            name: self.name.ok_or_else(|| eyre!("Charm name is required"))?,
            summary: self.summary,
            duration: self
                .duration
                .ok_or_else(|| eyre!("Charm name is required"))?,
            keywords: self.keywords,
            costs: self.costs,
            description: self
                .description
                .ok_or_else(|| eyre!("Charm name is required"))?,
        })
    }
}

struct _DragonBloodedCharm {
    action_type: CharmActionType,
    ability: AbilityNameNoSubskill,
    ability_requirement: u8,
    essence_requirement: u8,
    traits: CharmTraits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarCharm {
    id: SolarCharmId,
    action_type: CharmActionType,
    ability: AbilityNameNoSubskill,
    ability_requirement: u8,
    essence_requirement: u8,
    traits: CharmTraits,
    prerequisite_charms: Vec<SolarCharmId>,
}

impl PartialEq for SolarCharm {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SolarCharm {}

impl SolarCharm {
    pub fn action_type(&self) -> CharmActionType {
        self.action_type
    }

    pub fn ability(&self) -> AbilityNameNoSubskill {
        self.ability
    }

    pub fn ability_requirement(&self) -> (AbilityNameNoSubskill, u8) {
        (self.ability, self.ability_requirement)
    }

    pub fn essence_requirement(&self) -> u8 {
        self.essence_requirement
    }

    pub fn prerequisite_charm_ids(&self) -> impl Iterator<Item = SolarCharmId> + '_ {
        self.prerequisite_charms.iter().copied()
    }

    pub fn id(&self) -> SolarCharmId {
        self.id
    }

    pub fn data_source(&self) -> &DataSource {
        self.traits.data_source()
    }

    pub fn name(&self) -> &str {
        self.traits.name()
    }

    pub fn summary(&self) -> Option<&str> {
        self.traits.summary()
    }

    pub fn duration(&self) -> &str {
        self.traits.duration()
    }

    pub fn keywords(&self) -> &Vec<CharmKeyword> {
        self.traits.keywords()
    }

    pub fn costs(&self) -> &Vec<(CharmCostType, u8)> {
        self.traits.costs()
    }

    pub fn description(&self) -> &str {
        self.traits.description()
    }
}

struct _LunarCharm {
    action_type: CharmActionType,
    attribute: Option<AttributeName>,
    attribute_requirement: u8,
    essence_requirement: u8,
    traits: CharmTraits,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MartialArtsCharm {
    charm_id: MartialArtsCharmId,
    style_id: MartialArtsStyleId,
    action_type: CharmActionType,
    martial_arts_requirement: u8,
    essence_requirement: u8,
    traits: CharmTraits,
    prerequisite_charms: Vec<MartialArtsCharmId>,
}

impl PartialEq for MartialArtsCharm {
    fn eq(&self, other: &Self) -> bool {
        self.charm_id == other.charm_id
    }
}

impl Eq for MartialArtsCharm {}

impl MartialArtsCharm {
    pub fn from_book(
        id: MartialArtsCharmId,
        book_title: String,
        page_number: i16,
    ) -> MartialArtsCharmBuilder {
        MartialArtsCharmBuilder {
            style_id: None,
            charm_id: id,
            action_type: None,
            martial_arts_requirement: None,
            essence_requirement: None,
            traits: CharmTraits::from_book(book_title, page_number),
            prerequisite_charms: Vec::new(),
        }
    }

    pub fn custom(id: MartialArtsCharmId, creator_id: CharacterId) -> MartialArtsCharmBuilder {
        MartialArtsCharmBuilder {
            style_id: None,
            charm_id: id,
            action_type: None,
            martial_arts_requirement: None,
            essence_requirement: None,
            traits: CharmTraits::custom(creator_id),
            prerequisite_charms: Vec::new(),
        }
    }

    pub fn id(&self) -> MartialArtsCharmId {
        self.charm_id
    }

    pub fn style_id(&self) -> MartialArtsStyleId {
        self.style_id
    }

    pub fn data_source(&self) -> &DataSource {
        self.traits.data_source()
    }

    pub fn name(&self) -> &str {
        self.traits.name()
    }

    pub fn summary(&self) -> Option<&str> {
        self.traits.summary()
    }

    pub fn duration(&self) -> &str {
        self.traits.duration()
    }

    pub fn keywords(&self) -> &Vec<CharmKeyword> {
        self.traits.keywords()
    }

    pub fn description(&self) -> &str {
        self.traits.description()
    }

    pub fn martial_arts_requirement(&self) -> u8 {
        self.martial_arts_requirement
    }

    pub fn essence_requirement(&self) -> u8 {
        self.essence_requirement
    }

    pub fn action_type(&self) -> CharmActionType {
        self.action_type
    }

    pub fn costs(&self) -> &Vec<(CharmCostType, u8)> {
        self.traits.costs()
    }

    pub fn prerequisite_charm_ids(&self) -> &Vec<MartialArtsCharmId> {
        &self.prerequisite_charms
    }
}

pub struct MartialArtsCharmBuilder {
    style_id: Option<MartialArtsStyleId>,
    charm_id: MartialArtsCharmId,
    action_type: Option<CharmActionType>,
    martial_arts_requirement: Option<u8>,
    essence_requirement: Option<u8>,
    traits: CharmTraitsBuilder,
    prerequisite_charms: Vec<MartialArtsCharmId>,
}

impl MartialArtsCharmBuilder {
    pub fn for_martial_arts_style(mut self, style_id: MartialArtsStyleId) -> Self {
        self.style_id = Some(style_id);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.traits = self.traits.with_name(name);
        self
    }

    pub fn with_summary(mut self, summary: String) -> Self {
        self.traits = self.traits.with_summary(summary);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.traits = self.traits.with_description(description);
        self
    }

    pub fn with_duration(mut self, duration: String) -> Self {
        self.traits = self.traits.with_duration(duration);
        self
    }

    pub fn with_keyword(mut self, keyword: CharmKeyword) -> Self {
        self.traits = self.traits.with_keyword(keyword);
        self
    }

    pub fn with_action_type(mut self, action_type: CharmActionType) -> Self {
        self.action_type = Some(action_type);
        self
    }

    pub fn with_cost(mut self, cost_type: CharmCostType, amount: u8) -> Self {
        self.traits = self.traits.with_cost(cost_type, amount);
        self
    }

    pub fn requiring_martial_arts_dots(mut self, dots: u8) -> Self {
        self.martial_arts_requirement = Some(dots);
        self
    }

    pub fn requiring_essence(mut self, rating: u8) -> Self {
        self.essence_requirement = Some(rating);
        self
    }

    pub fn with_charm_prerequisite(mut self, martial_arts_charm_id: MartialArtsCharmId) -> Self {
        self.prerequisite_charms.push(martial_arts_charm_id);
        self.prerequisite_charms.sort();
        self.prerequisite_charms.dedup();
        self
    }

    pub fn build(self) -> Result<MartialArtsCharm> {
        Ok(MartialArtsCharm {
            style_id: self
                .style_id
                .ok_or_else(|| eyre!("Martial Arts style required for Martial Arts charms"))?,
            charm_id: self.charm_id,
            action_type: self
                .action_type
                .ok_or_else(|| eyre!("Action type required for Martial Arts charms"))?,
            martial_arts_requirement: self
                .martial_arts_requirement
                .ok_or_else(|| eyre!("Martial Arts dots level required for Martial Arts charms"))?,
            essence_requirement: self
                .essence_requirement
                .ok_or_else(|| eyre!("Essence dots level required for Martial Arts charms"))?,
            traits: self.traits.build()?,
            prerequisite_charms: self.prerequisite_charms,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    id: SpellId,
    circle: SorceryCircle,
    traits: CharmTraits,
}

impl PartialEq for Spell {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Spell {}

impl Spell {
    pub fn from_book(id: SpellId, book_title: String, page_number: i16) -> SpellBuilder {
        SpellBuilder {
            id,
            level: None,
            traits: CharmTraits::from_book(book_title, page_number),
        }
    }

    pub fn custom(id: SpellId, creator_id: CharacterId) -> SpellBuilder {
        SpellBuilder {
            id,
            level: None,
            traits: CharmTraits::custom(creator_id),
        }
    }

    pub fn circle(&self) -> SorceryCircle {
        self.circle
    }

    pub fn id(&self) -> SpellId {
        self.id
    }

    pub fn data_source(&self) -> &DataSource {
        self.traits.data_source()
    }

    pub fn name(&self) -> &str {
        self.traits.name()
    }

    pub fn summary(&self) -> Option<&str> {
        self.traits.summary()
    }

    pub fn duration(&self) -> &str {
        self.traits.duration()
    }

    pub fn keywords(&self) -> &Vec<CharmKeyword> {
        self.traits.keywords()
    }

    pub fn costs(&self) -> &Vec<(CharmCostType, u8)> {
        self.traits.costs()
    }

    pub fn description(&self) -> &str {
        self.traits.description()
    }
}

pub struct SpellBuilder {
    id: SpellId,
    level: Option<SorceryCircle>,
    traits: CharmTraitsBuilder,
}

impl SpellBuilder {
    pub fn with_spell_level(mut self, spell_level: SorceryCircle) -> Self {
        self.level = Some(spell_level);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.traits = self.traits.with_name(name);
        self
    }

    pub fn with_summary(mut self, summary: String) -> Self {
        self.traits = self.traits.with_summary(summary);
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.traits = self.traits.with_description(description);
        self
    }

    pub fn with_duration(mut self, duration: String) -> Self {
        self.traits = self.traits.with_duration(duration);
        self
    }

    pub fn with_keyword(mut self, keyword: CharmKeyword) -> Self {
        self.traits = self.traits.with_keyword(keyword);
        self
    }

    pub fn with_cost(mut self, cost: CharmCostType, amount: u8) -> Self {
        self.traits = self.traits.with_cost(cost, amount);
        self
    }

    pub fn build(self) -> Result<Spell> {
        if self.level.is_none() {
            Err(eyre!(
                "Spell level (Terrestrial, Celestial, Solar) is required"
            ))
        } else {
            let traits = self.traits.build()?;

            Ok(Spell {
                id: self.id,
                circle: self.level.unwrap(),
                traits,
            })
        }
    }
}