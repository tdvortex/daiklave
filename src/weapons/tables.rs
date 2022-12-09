use std::collections::{HashMap, HashSet};

use crate::character::CharacterBuilder;
use crate::weapons::{EquipHand, RangeBand, Weapon, WeaponTag};
use eyre::{eyre, Report, Result};
use sqlx::postgres::PgHasArrayType;

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "RANGEBAND", rename_all = "UPPERCASE")]
pub enum RangeBandPostgres {
    Close,
    Short,
    Medium,
    Long,
    Extreme,
}

impl From<RangeBandPostgres> for RangeBand {
    fn from(range: RangeBandPostgres) -> Self {
        match range {
            RangeBandPostgres::Close => Self::Close,
            RangeBandPostgres::Short => Self::Short,
            RangeBandPostgres::Medium => Self::Medium,
            RangeBandPostgres::Long => Self::Long,
            RangeBandPostgres::Extreme => Self::Extreme,
        }
    }
}

impl From<RangeBand> for RangeBandPostgres {
    fn from(range: RangeBand) -> Self {
        match range {
            RangeBand::Close => Self::Close,
            RangeBand::Short => Self::Short,
            RangeBand::Medium => Self::Medium,
            RangeBand::Long => Self::Long,
            RangeBand::Extreme => Self::Extreme,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "WEAPONTAGTYPE", rename_all = "UPPERCASE")]
pub enum WeaponTagTypePostgres {
    Archery,
    Artifact,
    Balanced,
    Bashing,
    Brawl,
    Chopping,
    Concealable,
    Crossbow,
    Cutting,
    Disarming,
    Exceptional,
    Flame,
    Flexible,
    Grappling,
    Heavy,
    Improvised,
    Lethal,
    Light,
    MartialArts,
    Medium,
    Melee,
    Mounted,
    OneHanded,
    Natural,
    Piercing,
    Poisonable,
    Powerful,
    Reaching,
    Shield,
    Slow,
    Smashing,
    Special,
    Subtle,
    Thrown,
    TwoHanded,
    Worn,
}

impl From<WeaponTag> for WeaponTagTypePostgres {
    fn from(tag: WeaponTag) -> Self {
        match tag {
            WeaponTag::Archery(_) => Self::Archery,
            WeaponTag::Artifact => Self::Artifact,
            WeaponTag::Balanced => Self::Balanced,
            WeaponTag::Bashing => Self::Bashing,
            WeaponTag::Brawl => Self::Brawl,
            WeaponTag::Chopping => Self::Chopping,
            WeaponTag::Concealable => Self::Concealable,
            WeaponTag::Crossbow => Self::Crossbow,
            WeaponTag::Cutting => Self::Cutting,
            WeaponTag::Disarming => Self::Disarming,
            WeaponTag::Exceptional => Self::Exceptional,
            WeaponTag::Flame => Self::Flame,
            WeaponTag::Flexible => Self::Flexible,
            WeaponTag::Grappling => Self::Grappling,
            WeaponTag::Heavy => Self::Heavy,
            WeaponTag::Improvised => Self::Improvised,
            WeaponTag::Lethal => Self::Lethal,
            WeaponTag::Light => Self::Light,
            WeaponTag::MartialArts(_) => Self::MartialArts,
            WeaponTag::Medium => Self::Medium,
            WeaponTag::Melee => Self::Melee,
            WeaponTag::Mounted => Self::Mounted,
            WeaponTag::OneHanded => Self::OneHanded,
            WeaponTag::Natural => Self::Natural,
            WeaponTag::Piercing => Self::Piercing,
            WeaponTag::Poisonable => Self::Poisonable,
            WeaponTag::Powerful => Self::Powerful,
            WeaponTag::Reaching => Self::Reaching,
            WeaponTag::Shield => Self::Shield,
            WeaponTag::Slow => Self::Slow,
            WeaponTag::Smashing => Self::Smashing,
            WeaponTag::Special => Self::Special,
            WeaponTag::Subtle => Self::Subtle,
            WeaponTag::Thrown(_) => Self::Thrown,
            WeaponTag::TwoHanded => Self::TwoHanded,
            WeaponTag::Worn => Self::Worn,
        }
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "WEAPONTAG")]
pub struct WeaponTagPostgres {
    tag_type: WeaponTagTypePostgres,
    max_range: Option<RangeBandPostgres>,
    martial_arts_style: Option<String>,
}

impl WeaponTagPostgres {
    fn from_archery(range: RangeBandPostgres) -> Self {
        Self {
            tag_type: WeaponTagTypePostgres::Archery,
            max_range: Some(range),
            martial_arts_style: None,
        }
    }

    fn from_martial_arts(style: String) -> Self {
        Self {
            tag_type: WeaponTagTypePostgres::MartialArts,
            max_range: None,
            martial_arts_style: Some(style),
        }
    }

    fn from_thrown(range: RangeBandPostgres) -> Self {
        Self {
            tag_type: WeaponTagTypePostgres::Thrown,
            max_range: Some(range),
            martial_arts_style: None,
        }
    }
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

impl From<WeaponTag> for WeaponTagPostgres {
    fn from(value: WeaponTag) -> Self {
        match value {
            WeaponTag::Archery(range) => Self::from_archery(range.into()),
            WeaponTag::MartialArts(style) => Self::from_martial_arts(style),
            WeaponTag::Thrown(range) => Self::from_thrown(range.into()),
            other => Self {
                tag_type: other.into(),
                max_range: None,
                martial_arts_style: None,
            },
        }
    }
}

impl PgHasArrayType for WeaponTagPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_WEAPONTAG")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "EQUIPHAND", rename_all = "UPPERCASE")]
pub enum EquipHandPostgres {
    Main,
    Off,
}

impl PgHasArrayType for EquipHandPostgres {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_EQUIPHAND")
    }
}

#[derive(Debug)]
pub struct WeaponRow {
    pub id: i32,
    pub name: String,
    pub tags: Vec<WeaponTagPostgres>,
    pub creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for WeaponRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("weapons")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for WeaponRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let name = decoder.try_decode::<String>()?;
        let tags = decoder.try_decode::<Vec<WeaponTagPostgres>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            id,
            name,
            tags,
            creator_id,
        })
    }
}

#[derive(Debug)]
pub struct WeaponEquippedRow {
    pub character_id: i32,
    pub weapon_id: i32,
    pub equip_hand: Option<EquipHandPostgres>,
    pub creator_id: Option<i32>,
}

impl sqlx::Type<sqlx::Postgres> for WeaponEquippedRow {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("character_weapons")
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for WeaponEquippedRow {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let character_id = decoder.try_decode::<i32>()?;
        let weapon_id = decoder.try_decode::<i32>()?;
        let equip_hand = decoder.try_decode::<Option<EquipHandPostgres>>()?;
        let creator_id = decoder.try_decode::<Option<i32>>()?;

        Ok(Self {
            character_id,
            weapon_id,
            equip_hand,
            creator_id,
        })
    }
}

impl CharacterBuilder {
    pub(crate) fn apply_weapon_rows(
        self,
        weapon_rows: Vec<WeaponRow>,
        weapon_equipped_rows: Option<Vec<WeaponEquippedRow>>,
    ) -> Result<Self> {
        let mut weapons_hashmap = HashMap::new();

        for weapon_row in weapon_rows.into_iter() {
            let mut tags = HashSet::new();
            for tag in weapon_row.tags {
                tags.insert(tag.try_into()?);
            }

            let weapon = Weapon::new(
                weapon_row.name,
                tags,
                Some(weapon_row.id),
                weapon_row.creator_id,
            )?;
            weapons_hashmap.insert(weapon_row.id, (weapon, None));
        }

        if weapon_equipped_rows.is_none() {
            return Ok(self);
        }

        let equips = weapon_equipped_rows.unwrap();

        for weapon_equipped_row in equips.into_iter() {
            if weapon_equipped_row.equip_hand.is_none() {
                continue;
            }

            let (_, equipped) = weapons_hashmap
                .get_mut(&weapon_equipped_row.weapon_id)
                .ok_or_else(|| {
                    eyre!(
                        "cannot equip weapon {} which is not owned",
                        weapon_equipped_row.weapon_id
                    )
                })?;

            *equipped = match (&equipped, weapon_equipped_row.equip_hand.unwrap()) {
                (None, EquipHandPostgres::Main) => Some(EquipHand::Main),
                (None, EquipHandPostgres::Off) => Some(EquipHand::Off),
                (Some(EquipHand::Main), EquipHandPostgres::Main) => {
                    return Err(eyre!("cannot equip two weapons in Main hand"));
                }
                (Some(EquipHand::Off), EquipHandPostgres::Off) => {
                    return Err(eyre!("cannot equip two weapons in Off hand"));
                }
                (Some(EquipHand::Both), EquipHandPostgres::Main) => {
                    return Err(eyre!("cannot equip two weapons in Main hand"));
                }
                (Some(EquipHand::Both), EquipHandPostgres::Off) => {
                    return Err(eyre!("cannot equip two weapons in Off hand"));
                }
                (Some(EquipHand::Main), EquipHandPostgres::Off) => Some(EquipHand::Both),
                (Some(EquipHand::Off), EquipHandPostgres::Main) => Some(EquipHand::Both),
            };
        }

        let mut applied = Ok(self);

        for (_, (weapon, maybe_equip_hand)) in weapons_hashmap.into_iter() {
            applied = applied.and_then(|character| character.with_weapon(weapon, maybe_equip_hand));
        }

        applied
    }
}
