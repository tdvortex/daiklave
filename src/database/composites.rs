use sqlx::postgres::PgHasArrayType;

use super::enums::{WeaponTagType, RangeBand, CharmCostType};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "WEAPONTAG")]
pub struct WeaponTag {
    tag_type: WeaponTagType,
    max_range: Option<RangeBand>,
    martial_arts_style: Option<String>,
}

impl PgHasArrayType for WeaponTag {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_WEAPONTAG")
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "CHARMCOST")]
pub struct CharmCost {
    cost_type: CharmCostType,
    amount: i16,
}

impl PgHasArrayType for CharmCost {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_CHARMCOST")
    }
}