use std::ops::Div;

use crate::{hearthstones::{hearthstone::GeomancyLevel, HearthstoneId}, artifact::ArtifactId, unique_id::UniqueId, book_reference::{BookReference, Book}, martial_arts::MartialArtsStyleId, languages::language::MajorLanguage, sorcery::{SorceryArchetypeMeritId, SorceryArchetypeMerit}};

use super::{StackableMeritId, MeritId, template::MeritTemplateId, MeritType, constants::{artifact::{ARTIFACT_SHARED, ARTIFACT_TWO, ARTIFACT_FOUR, ARTIFACT_FIVE, ARTIFACT_NA, ARTIFACT_THREE}, demense::{DEMENSE_SHARED, DEMENSE_STANDARD, DEMENSE_GREATER}, exalted_healing::EXALTED_HEALING, hearthstone::{HEARTHSTONE_SHARED, HEARTHSTONE_STANDARD, HEARTHSTONE_GREATER, HEARTHSTONE_MANSE_STANDARD, HEARTHSTONE_MANSE_GREATER}, manse::{MANSE_SHARED, MANSE_STANDARD, MANSE_GREATER}, martial_arts::MARTIAL_ARTIST, mortal_sorcerer::MORTAL_SORCERY, languages::{LANGUAGE_SHARED, LANGUAGE_LOCAL_TONGUES, LANGUAGE_DRAGONTONGUE, LANGUAGE_FLAMETONGUE, LANGUAGE_FORESTTONGUE, LANGUAGE_GUILD_CANT, LANGUAGE_HIGH_REALM, LANGUAGE_LOW_REALM, LANGUAGE_OLD_REALM, LANGUAGE_RIVERSPEAK, LANGUAGE_SEATONGUE, LANGUAGE_SKYTONGUE}}, nonstackable::{NonStackableMeritView, NonStackableMeritId}, stackable::StackableMeritView};

pub(crate) enum MeritSource<'source> {
    Artifact(ArtifactId, &'source str, u8),
    DemenseNoManse(UniqueId, &'source str, GeomancyLevel),
    DemenseWithManse(HearthstoneId, &'source str, GeomancyLevel),
    ExaltedHealing(bool), // is_exalt
    HearthstoneNoManse(HearthstoneId, &'source str, GeomancyLevel),
    HearthstoneWithManse(HearthstoneId, &'source str, GeomancyLevel),
    LocalTongues(usize),
    MajorLanguage(MajorLanguage),
    Manse(HearthstoneId, &'source str, GeomancyLevel),
    MartialArtist(MartialArtsStyleId, &'source str),
    MortalSorcerer,
    NonStackable(NonStackableMeritId, NonStackableMeritView<'source>),
    SorceryArchetype(SorceryArchetypeMeritId, &'source SorceryArchetypeMerit),
    Stackable(StackableMeritId, StackableMeritView<'source>),
}

impl<'source> MeritSource<'source> {
    pub fn id(&self) -> MeritId {
        match self {
            MeritSource::Artifact(artifact_id, _, _) => MeritId::Artifact(*artifact_id),
            MeritSource::DemenseNoManse(unique_id, _, _) => MeritId::DemenseNoManse(*unique_id),
            MeritSource::DemenseWithManse(hearthstone_id, _, _) => MeritId::DemenseWithManse(*hearthstone_id),
            MeritSource::ExaltedHealing(_) => MeritId::ExaltedHealing,
            MeritSource::HearthstoneNoManse(hearthstone_id, _, _) => MeritId::HearthstoneNoManse(*hearthstone_id),
            MeritSource::HearthstoneWithManse(hearthstone_id, _, _) => MeritId::HearthstoneWithManse(*hearthstone_id),
            MeritSource::LocalTongues(_) => MeritId::LocalTongues,
            MeritSource::MajorLanguage(major) => MeritId::MajorLanguage(*major),
            MeritSource::Manse(hearthstone_id, _, _) => MeritId::Manse(*hearthstone_id),
            MeritSource::MartialArtist(style_id, _) => MeritId::MartialArtist(*style_id),
            MeritSource::MortalSorcerer => MeritId::MortalSorcerer,
            MeritSource::NonStackable(nonstackable_id, _) => MeritId::NonStackable(*nonstackable_id),
            MeritSource::Stackable(stackable_merit_id, _) => MeritId::Stackable(*stackable_merit_id),
            MeritSource::SorceryArchetype(merit_id, _) => MeritId::SorceryArchetype(*merit_id),
        }
    }

    pub fn template_id(&self) -> MeritTemplateId {
        match self {
            MeritSource::Artifact(_, _, _) => MeritTemplateId::Artifact,
            MeritSource::DemenseNoManse(_, _, _) => MeritTemplateId::Demense,
            MeritSource::DemenseWithManse(_, _, _) => MeritTemplateId::Demense,
            MeritSource::ExaltedHealing(_) => MeritTemplateId::ExaltedHealing,
            MeritSource::HearthstoneNoManse(_, _, _) => MeritTemplateId::Hearthstone,
            MeritSource::HearthstoneWithManse(_, _, _) => MeritTemplateId::Hearthstone,
            MeritSource::LocalTongues(_) => MeritTemplateId::Language,
            MeritSource::MajorLanguage(_) => MeritTemplateId::Language,
            MeritSource::Manse(_, _, _) => MeritTemplateId::Manse,
            MeritSource::MartialArtist(_, _) => MeritTemplateId::MartialArtist,
            MeritSource::MortalSorcerer => MeritTemplateId::MortalSorcerer,
            MeritSource::NonStackable(nonstackable_id, _) => MeritTemplateId::NonStackable(*nonstackable_id),
            MeritSource::Stackable(_, stackable_merit) => stackable_merit.template_id(),
            MeritSource::SorceryArchetype(merit_id, _) => MeritTemplateId::SorceryArchetype(*merit_id),
        }
    }

    pub fn template_name(&self) -> &'source str {
        match self {
            MeritSource::Artifact(_, _, _) => "Artifact",
            MeritSource::DemenseNoManse(_, _, _) => "Demense",
            MeritSource::DemenseWithManse(_, _, _) => "Demense",
            MeritSource::ExaltedHealing(_) => "Exalted Healing",
            MeritSource::HearthstoneNoManse(_, _, _) => "Hearthstone",
            MeritSource::HearthstoneWithManse(_, _, _) => "Hearthstone",
            MeritSource::LocalTongues(_) => "Language",
            MeritSource::MajorLanguage(_) => "Language",
            MeritSource::Manse(_, _, _) => "Manse",
            MeritSource::MartialArtist(_, _) => "Martial Artist",
            MeritSource::MortalSorcerer => "Terrestrial Circle Sorcerer (Mortal)",
            MeritSource::NonStackable(_, nonstackable) => nonstackable.template_name(),
            MeritSource::Stackable(_, stackable) => stackable.template_name(),
            MeritSource::SorceryArchetype(_, sorcery_merit) => (*sorcery_merit).name(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            MeritSource::Artifact(_, _, _) => Some(BookReference::new(Book::CoreRulebook, 159)),
            MeritSource::DemenseNoManse(_, _, _) => Some(BookReference::new(Book::CoreRulebook, 160)),
            MeritSource::DemenseWithManse(_, _, _) => Some(BookReference::new(Book::CoreRulebook, 160)),
            MeritSource::ExaltedHealing(_) => Some(BookReference::new(Book::CoreRulebook, 165)),
            MeritSource::HearthstoneNoManse(_, _, _) => Some(BookReference::new(Book::CoreRulebook, 161)),
            MeritSource::HearthstoneWithManse(_, _, _) => Some(BookReference::new(Book::CoreRulebook, 161)),
            MeritSource::LocalTongues(_) => Some(BookReference::new(Book::CoreRulebook, 162)),
            MeritSource::MajorLanguage(_) => Some(BookReference::new(Book::CoreRulebook, 162)),
            MeritSource::Manse(_, _, _) => Some(BookReference::new(Book::CoreRulebook, 163)),
            MeritSource::MartialArtist(_, _) => Some(BookReference::new(Book::CoreRulebook, 163)),
            MeritSource::MortalSorcerer => Some(BookReference::new(Book::CoreRulebook, 470)),
            MeritSource::NonStackable(_, nonstackable) => nonstackable.book_reference(),
            MeritSource::Stackable(_, stackable) => stackable.book_reference(),
            MeritSource::SorceryArchetype(_, sorcery_merit) => (*sorcery_merit).book_reference(),
        }
    }

    pub fn detail(&self) -> Option<&'source str> {
        match self {
            MeritSource::Artifact(_, name, _) => Some(*name),
            MeritSource::DemenseNoManse(_, name, _) => Some(*name),
            MeritSource::DemenseWithManse(_, name, _) => Some(*name),
            MeritSource::ExaltedHealing(_) => None,
            MeritSource::HearthstoneNoManse(_, name, _) => Some(*name),
            MeritSource::HearthstoneWithManse(_, name, _) => Some(*name),
            MeritSource::Manse(_, detail, _) => Some(*detail),
            MeritSource::MartialArtist(_, style_name) => Some(*style_name),
            MeritSource::MortalSorcerer => None,
            MeritSource::NonStackable(_, _) => None,
            MeritSource::Stackable(_, stackable) => Some(stackable.detail()),
            MeritSource::LocalTongues(_) => Some("Local Tongues"),
            MeritSource::MajorLanguage(major) => Some(
                match major {
                    MajorLanguage::Dragontongue => "Dragontongue",
                    MajorLanguage::Flametongue => "Flametongue",
                    MajorLanguage::ForestTongue => "Forest-Tongue",
                    MajorLanguage::GuildCant => "Guild Cant",
                    MajorLanguage::HighRealm => "High Realm",
                    MajorLanguage::LowRealm => "Low Realm",
                    MajorLanguage::OldRealm => "Old Realm",
                    MajorLanguage::Riverspeak => "Riverspeak",
                    MajorLanguage::Seatongue => "Seatongue",
                    MajorLanguage::Skytongue => "Skytongue",
                }
            ),
            MeritSource::SorceryArchetype(_, _) => None,
        }
    }

    pub fn dots(&self) -> u8 {
        match self {
            MeritSource::Artifact(_, _, dots) => *dots,
            MeritSource::DemenseNoManse(_, _, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => 2,
                GeomancyLevel::Greater => 4,
            },
            MeritSource::DemenseWithManse(_, _, _) => 0,
            MeritSource::ExaltedHealing(is_exalt) => 5 * (1-u8::from(*is_exalt)),
            MeritSource::HearthstoneNoManse(_, _, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => 2,
                GeomancyLevel::Greater => 4,
            },
            MeritSource::HearthstoneWithManse(_, _, _) => 0,
            MeritSource::LocalTongues(count) => {
                ((*count).min(u8::MAX as usize) + 3).div(4) as u8
            }
            MeritSource::MajorLanguage(_) => 1,
            MeritSource::Manse(_, _, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => 3,
                GeomancyLevel::Greater => 5,
            },
            MeritSource::MartialArtist(_, _) => 4,
            MeritSource::MortalSorcerer => 5,
            MeritSource::NonStackable(_, nonstackable) => nonstackable.dots(),
            MeritSource::Stackable(_, stackable) => stackable.dots(),
            MeritSource::SorceryArchetype(_, sorcery_merit) => (*sorcery_merit).dots(),
        }
    }

    pub fn merit_type(&self) -> MeritType {
        match self {
            MeritSource::Artifact(_, _, _) => MeritType::Story,
            MeritSource::DemenseNoManse(_, _, _) => MeritType::Story,
            MeritSource::DemenseWithManse(_, _, _) => MeritType::Story,
            MeritSource::ExaltedHealing(_) => MeritType::Supernatural,
            MeritSource::HearthstoneNoManse(_, _, _) => MeritType::Story,
            MeritSource::HearthstoneWithManse(_, _, _) => MeritType::Story,
            MeritSource::Manse(_, _, _) => MeritType::Story,
            MeritSource::LocalTongues(_) => MeritType::Purchased,
            MeritSource::MajorLanguage(_) => MeritType::Purchased,
            MeritSource::MartialArtist(_, _) => MeritType::Purchased,
            MeritSource::MortalSorcerer => MeritType::Story,
            MeritSource::NonStackable(_, nonstackable) => nonstackable.merit_type(),
            MeritSource::Stackable(_, stackable) => stackable.merit_type(),
            MeritSource::SorceryArchetype(_, _) => MeritType::Story,
        }
    }

    pub fn description(&self) -> (&'source str, Option<&'source str>) {
        match self {
            MeritSource::Artifact(_, _, dots) => {
                match dots {
                    2 => (ARTIFACT_SHARED, Some(ARTIFACT_TWO)),
                    3 => (ARTIFACT_SHARED, Some(ARTIFACT_THREE)),
                    4 => (ARTIFACT_SHARED, Some(ARTIFACT_FOUR)),
                    5 => (ARTIFACT_SHARED, Some(ARTIFACT_FIVE)),
                    _ => (ARTIFACT_SHARED, Some(ARTIFACT_NA)),
                }
            }
            MeritSource::DemenseNoManse(_, _, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => (DEMENSE_SHARED, Some(DEMENSE_STANDARD)),
                GeomancyLevel::Greater => (DEMENSE_SHARED, Some(DEMENSE_GREATER)),
            },
            MeritSource::DemenseWithManse(_, _, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => (DEMENSE_SHARED, Some(DEMENSE_STANDARD)),
                GeomancyLevel::Greater => (DEMENSE_SHARED, Some(DEMENSE_GREATER)),
            },
            MeritSource::ExaltedHealing(_) => (EXALTED_HEALING, None),
            MeritSource::HearthstoneNoManse(_, _, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => (HEARTHSTONE_SHARED, Some(HEARTHSTONE_STANDARD)),
                GeomancyLevel::Greater => (HEARTHSTONE_SHARED, Some(HEARTHSTONE_GREATER)),
            },
            MeritSource::HearthstoneWithManse(_, _, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => (HEARTHSTONE_SHARED, Some(HEARTHSTONE_MANSE_STANDARD)),
                GeomancyLevel::Greater => (HEARTHSTONE_SHARED, Some(HEARTHSTONE_MANSE_GREATER)),
            },
            MeritSource::LocalTongues(_) => (LANGUAGE_SHARED, Some(LANGUAGE_LOCAL_TONGUES)),
            MeritSource::MajorLanguage(major) => (LANGUAGE_SHARED, Some(
                match major {
                    MajorLanguage::Dragontongue => LANGUAGE_DRAGONTONGUE,
                    MajorLanguage::Flametongue => LANGUAGE_FLAMETONGUE,
                    MajorLanguage::ForestTongue => LANGUAGE_FORESTTONGUE,
                    MajorLanguage::GuildCant => LANGUAGE_GUILD_CANT,
                    MajorLanguage::HighRealm => LANGUAGE_HIGH_REALM,
                    MajorLanguage::LowRealm => LANGUAGE_LOW_REALM,
                    MajorLanguage::OldRealm => LANGUAGE_OLD_REALM,
                    MajorLanguage::Riverspeak => LANGUAGE_RIVERSPEAK,
                    MajorLanguage::Seatongue => LANGUAGE_SEATONGUE,
                    MajorLanguage::Skytongue => LANGUAGE_SKYTONGUE,
                }
            )),
            MeritSource::Manse(_, _, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => (MANSE_SHARED, Some(MANSE_STANDARD)),
                GeomancyLevel::Greater => (MANSE_SHARED, Some(MANSE_GREATER)),
            },
            MeritSource::MartialArtist(_, _) => (MARTIAL_ARTIST, None),
            MeritSource::MortalSorcerer => (MORTAL_SORCERY, None),
            MeritSource::NonStackable(_, nonstackable) => nonstackable.description(),
            MeritSource::Stackable(_, stackable) => stackable.description(),
            MeritSource::SorceryArchetype(_, sorcery_merit) => ((*sorcery_merit).description(), None),
        }
    }
}