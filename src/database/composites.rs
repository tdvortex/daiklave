use sqlx::postgres::PgHasArrayType;
use eyre::{eyre, Report};

use super::enums::{CharmCostType, RangeBand, WeaponTagType};

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "WEAPONTAG")]
pub struct WeaponTag {
    tag_type: WeaponTagType,
    max_range: Option<RangeBand>,
    martial_arts_style: Option<String>,
}

impl TryFrom<WeaponTag> for crate::character::traits::weapons::WeaponTag {
    type Error = Report;

    fn try_from(value: WeaponTag) -> Result<Self, Self::Error> {
        match value.tag_type {
            WeaponTagType::Archery => {
                match value.max_range {
                    Some(range) => Ok(Self::Archery(range.into())),
                    None => Err(eyre!("archery must have a range band")),
                }
            }
            WeaponTagType::Artifact => Ok(Self::Artifact),
            WeaponTagType::Balanced => Ok(Self::Balanced),
            WeaponTagType::Bashing => Ok(Self::Bashing),
            WeaponTagType::Brawl => Ok(Self::Brawl),
            WeaponTagType::Chopping => Ok(Self::Chopping),
            WeaponTagType::Concealable => Ok(Self::Concealable),
            WeaponTagType::Crossbow => Ok(Self::Crossbow),
            WeaponTagType::Cutting => Ok(Self::Cutting),
            WeaponTagType::Disarming => Ok(Self::Disarming),
            WeaponTagType::Exceptional => Ok(Self::Exceptional),
            WeaponTagType::Flame => Ok(Self::Flame),
            WeaponTagType::Flexible => Ok(Self::Flexible),
            WeaponTagType::Grappling => Ok(Self::Grappling),
            WeaponTagType::Heavy => Ok(Self::Heavy),
            WeaponTagType::Improvised => Ok(Self::Improvised),
            WeaponTagType::Lethal => Ok(Self::Lethal),
            WeaponTagType::Light => Ok(Self::Light),
            WeaponTagType::MartialArts => {
                match value.martial_arts_style {
                    Some(style) => Ok(Self::MartialArts(style)),
                    None => Err(eyre!("martial arts must have a style")),
                }
            }
            WeaponTagType::Medium => Ok(Self::Medium),
            WeaponTagType::Melee => Ok(Self::Melee),
            WeaponTagType::Mounted => Ok(Self::Mounted),
            WeaponTagType::OneHanded => Ok(Self::OneHanded),
            WeaponTagType::Natural => Ok(Self::Natural),
            WeaponTagType::Piercing => Ok(Self::Piercing),
            WeaponTagType::Poisonable => Ok(Self::Poisonable),
            WeaponTagType::Powerful => Ok(Self::Powerful),
            WeaponTagType::Reaching => Ok(Self::Reaching),
            WeaponTagType::Shield => Ok(Self::Shield),
            WeaponTagType::Slow => Ok(Self::Slow),
            WeaponTagType::Smashing => Ok(Self::Smashing),
            WeaponTagType::Special => Ok(Self::Special),
            WeaponTagType::Subtle => Ok(Self::Subtle),
            WeaponTagType::Thrown => {
                match value.max_range {
                    Some(range) => Ok(Self::Thrown(range.into())),
                    None => Err(eyre!("thrown must have a range band")),
                }
            }
            WeaponTagType::TwoHanded => Ok(Self::TwoHanded),
            WeaponTagType::Worn => Ok(Self::Worn),
        }
    }
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
