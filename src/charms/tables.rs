use sqlx::postgres::PgHasArrayType;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMDURATIONTYPE", rename_all = "UPPERCASE")]
pub enum CharmDurationTypePostgres {
    Instant,
    Tick,
    Turn,
    Round,
    Scene,
    Indefinite,
    Permanent,
    Special,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "CHARMACTIONTYPE", rename_all = "UPPERCASE")]
pub enum CharmActionTypePostgres {
    Simple,
    Supplemental,
    Reflexive,
    Permanent,
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
    pub duration: CharmDurationTypePostgres,
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
