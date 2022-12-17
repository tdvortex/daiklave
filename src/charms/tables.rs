use sqlx::postgres::PgHasArrayType;

use super::{CharmActionType, CharmKeyword};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "CHARMCOST")]
pub struct CharmCostPostgres {
    cost_type: CharmCostTypePostgres,
    amount: i16,
}

impl PgHasArrayType for CharmCostPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_CHARMCOST")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMKEYWORD", rename_all = "UPPERCASE")]
pub enum CharmKeywordPostgres {
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
    Salient,
    Signature,
    Stackable,
    Uniform,
    Water,
    WitheringOnly,
    Wood,
    WrittenOnly,
}

impl From<CharmKeywordPostgres> for CharmKeyword {
    fn from(keyword_pg: CharmKeywordPostgres) -> Self {
        match keyword_pg {
            CharmKeywordPostgres::Air => Self::Air,
            CharmKeywordPostgres::Aggravated => Self::Aggravated,
            CharmKeywordPostgres::Archetype => Self::Archetype,
            CharmKeywordPostgres::Aura => Self::Aura,
            CharmKeywordPostgres::Balanced => Self::Balanced,
            CharmKeywordPostgres::Bridge => Self::Bridge,
            CharmKeywordPostgres::Clash => Self::Clash,
            CharmKeywordPostgres::Counterattack => Self::Counterattack,
            CharmKeywordPostgres::DecisiveOnly => Self::DecisiveOnly,
            CharmKeywordPostgres::Dual => Self::Dual,
            CharmKeywordPostgres::Excellency => Self::Excellency,
            CharmKeywordPostgres::Fire => Self::Fire,
            CharmKeywordPostgres::Earth => Self::Earth,
            CharmKeywordPostgres::Mute => Self::Mute,
            CharmKeywordPostgres::Pilot => Self::Pilot,
            CharmKeywordPostgres::Protean => Self::Protean,
            CharmKeywordPostgres::Psyche => Self::Psyche,
            CharmKeywordPostgres::Perilous => Self::Perilous,
            CharmKeywordPostgres::Salient => Self::Salient,
            CharmKeywordPostgres::Signature => Self::Signature,
            CharmKeywordPostgres::Stackable => Self::Stackable,
            CharmKeywordPostgres::Uniform => Self::Uniform,
            CharmKeywordPostgres::Water => Self::Water,
            CharmKeywordPostgres::WitheringOnly => Self::WitheringOnly,
            CharmKeywordPostgres::Wood => Self::Wood,
            CharmKeywordPostgres::WrittenOnly => Self::WrittenOnly,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMACTIONTYPE", rename_all = "UPPERCASE")]
pub enum CharmActionTypePostgres {
    Simple,
    Supplemental,
    Reflexive,
    Permanent,
}

impl From<CharmActionTypePostgres> for CharmActionType {
    fn from(value: CharmActionTypePostgres) -> Self {
        match value {
            CharmActionTypePostgres::Simple => Self::Simple,
            CharmActionTypePostgres::Supplemental => Self::Supplemental,
            CharmActionTypePostgres::Reflexive => Self::Reflexive,
            CharmActionTypePostgres::Permanent => Self::Permanent,
        }
    }
}

impl From<CharmActionType> for CharmActionTypePostgres {
    fn from(value: CharmActionType) -> Self {
        match value {
            CharmActionType::Simple => Self::Simple,
            CharmActionType::Supplemental => Self::Supplemental,
            CharmActionType::Reflexive => Self::Reflexive,
            CharmActionType::Permanent => Self::Permanent,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMCOSTTYPE", rename_all = "UPPERCASE")]
pub enum CharmCostTypePostgres {
    Motes,
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
    SorcerousMotes,
}

#[derive(Debug)]
pub struct CharmRow {
    pub id: i32,
    pub name: String,
    pub costs: Vec<CharmCostPostgres>,
    pub action_type: CharmActionTypePostgres,
    pub keywords: Vec<CharmKeywordPostgres>,
    pub duration: String,
    pub special_duration: Option<String>,
    pub book_name: Option<String>,
    pub page_number: Option<i32>,
    pub creator_id: Option<i32>,
    pub summary: String,
    pub description: String,
}

#[derive(Debug)]
pub struct CharmPrerequisiteSetRow {
    pub id: i32,
    pub charm_id: i32,
}
