use eyre::{eyre, Report};
use sqlx::postgres::PgHasArrayType;

use crate::character::traits::weapons::WeaponTag;

use super::enums::{CharmCostTypePostgres, RangeBandPostgres, WeaponTagTypePostgres};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "WEAPONTAG")]
pub struct WeaponTagPostgres {
    tag_type: WeaponTagTypePostgres,
    max_range: Option<RangeBandPostgres>,
    martial_arts_style: Option<String>,
}

impl TryFrom<WeaponTagPostgres> for WeaponTag {
    type Error = Report;

    fn try_from(value: WeaponTagPostgres) -> Result<Self, Self::Error> {
        match value.tag_type {
            WeaponTagTypePostgres::Archery => match value.max_range {
                Some(range) => Ok(Self::Archery(range.into())),
                None => Err(eyre!("archery must have a range band")),
            },
            WeaponTagTypePostgres::Artifact => Ok(Self::Artifact),
            WeaponTagTypePostgres::Balanced => Ok(Self::Balanced),
            WeaponTagTypePostgres::Bashing => Ok(Self::Bashing),
            WeaponTagTypePostgres::Brawl => Ok(Self::Brawl),
            WeaponTagTypePostgres::Chopping => Ok(Self::Chopping),
            WeaponTagTypePostgres::Concealable => Ok(Self::Concealable),
            WeaponTagTypePostgres::Crossbow => Ok(Self::Crossbow),
            WeaponTagTypePostgres::Cutting => Ok(Self::Cutting),
            WeaponTagTypePostgres::Disarming => Ok(Self::Disarming),
            WeaponTagTypePostgres::Exceptional => Ok(Self::Exceptional),
            WeaponTagTypePostgres::Flame => Ok(Self::Flame),
            WeaponTagTypePostgres::Flexible => Ok(Self::Flexible),
            WeaponTagTypePostgres::Grappling => Ok(Self::Grappling),
            WeaponTagTypePostgres::Heavy => Ok(Self::Heavy),
            WeaponTagTypePostgres::Improvised => Ok(Self::Improvised),
            WeaponTagTypePostgres::Lethal => Ok(Self::Lethal),
            WeaponTagTypePostgres::Light => Ok(Self::Light),
            WeaponTagTypePostgres::MartialArts => match value.martial_arts_style {
                Some(style) => Ok(Self::MartialArts(style)),
                None => Err(eyre!("martial arts must have a style")),
            },
            WeaponTagTypePostgres::Medium => Ok(Self::Medium),
            WeaponTagTypePostgres::Melee => Ok(Self::Melee),
            WeaponTagTypePostgres::Mounted => Ok(Self::Mounted),
            WeaponTagTypePostgres::OneHanded => Ok(Self::OneHanded),
            WeaponTagTypePostgres::Natural => Ok(Self::Natural),
            WeaponTagTypePostgres::Piercing => Ok(Self::Piercing),
            WeaponTagTypePostgres::Poisonable => Ok(Self::Poisonable),
            WeaponTagTypePostgres::Powerful => Ok(Self::Powerful),
            WeaponTagTypePostgres::Reaching => Ok(Self::Reaching),
            WeaponTagTypePostgres::Shield => Ok(Self::Shield),
            WeaponTagTypePostgres::Slow => Ok(Self::Slow),
            WeaponTagTypePostgres::Smashing => Ok(Self::Smashing),
            WeaponTagTypePostgres::Special => Ok(Self::Special),
            WeaponTagTypePostgres::Subtle => Ok(Self::Subtle),
            WeaponTagTypePostgres::Thrown => match value.max_range {
                Some(range) => Ok(Self::Thrown(range.into())),
                None => Err(eyre!("thrown must have a range band")),
            },
            WeaponTagTypePostgres::TwoHanded => Ok(Self::TwoHanded),
            WeaponTagTypePostgres::Worn => Ok(Self::Worn),
        }
    }
}

impl PgHasArrayType for WeaponTagPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_WEAPONTAG")
    }
}

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
