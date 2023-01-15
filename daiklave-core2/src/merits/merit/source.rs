use crate::{hearthstones::{hearthstone::GeomancyLevel, HearthstoneId}, artifact::ArtifactId, unique_id::UniqueId, book_reference::{BookReference, Book}, martial_arts::MartialArtsStyleId};

use super::{StackableMeritId, MeritId, template::MeritTemplateId, MeritType, constants::{artifact::{ARTIFACT_SHARED, ARTIFACT_TWO, ARTIFACT_FOUR, ARTIFACT_FIVE, ARTIFACT_NA, ARTIFACT_THREE}, demense::{DEMENSE_SHARED, DEMENSE_STANDARD, DEMENSE_GREATER}, exalted_healing::EXALTED_HEALING, hearthstone::{HEARTHSTONE_SHARED, HEARTHSTONE_STANDARD, HEARTHSTONE_GREATER, HEARTHSTONE_MANSE_STANDARD, HEARTHSTONE_MANSE_GREATER}, manse::{MANSE_SHARED, MANSE_STANDARD, MANSE_GREATER}, martial_arts::MARTIAL_ARTIST, mortal_sorcerer::MORTAL_SORCERY}, nonstackable::{NonStackableMeritView, NonStackableMeritId}, stackable::StackableMeritView};

pub(crate) enum MeritSource<'source> {
    Artifact(ArtifactId, &'source str, u8),
    DemenseNoManse(UniqueId, &'source str, GeomancyLevel),
    DemenseWithManse(HearthstoneId, &'source str, GeomancyLevel),
    ExaltedHealing(bool), // is_exalt
    HearthstoneNoManse(HearthstoneId, &'source str, GeomancyLevel),
    HearthstoneWithManse(HearthstoneId, &'source str, GeomancyLevel),
    Manse(HearthstoneId, &'source str, GeomancyLevel),
    MartialArtist(MartialArtsStyleId, &'source str),
    MortalSorcerer,
    NonStackableMerit(NonStackableMeritId, NonStackableMeritView<'source>),
    StackableMerit(StackableMeritId, StackableMeritView<'source>),
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
            MeritSource::Manse(hearthstone_id, _, _) => MeritId::Manse(*hearthstone_id),
            MeritSource::MartialArtist(style_id, _) => MeritId::MartialArtist(*style_id),
            MeritSource::MortalSorcerer => MeritId::MortalSorcerer,
            MeritSource::NonStackableMerit(nonstackable_id, _) => MeritId::NonStackableMerit(*nonstackable_id),
            MeritSource::StackableMerit(stackable_merit_id, _) => MeritId::StackableMerit(*stackable_merit_id),
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
            MeritSource::Manse(_, _, _) => MeritTemplateId::Manse,
            MeritSource::MartialArtist(_, _) => MeritTemplateId::MartialArtist,
            MeritSource::MortalSorcerer => MeritTemplateId::MortalSorcerer,
            MeritSource::NonStackableMerit(nonstackable_id, _) => MeritTemplateId::NonStackableMerit(*nonstackable_id),
            MeritSource::StackableMerit(_, stackable_merit) => stackable_merit.template_id(),
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
            MeritSource::Manse(_, _, _) => "Manse",
            MeritSource::MartialArtist(_, _) => "Martial Artist",
            MeritSource::MortalSorcerer => "Terrestrial Circle Sorcerer (Mortal)",
            MeritSource::NonStackableMerit(_, nonstackable) => nonstackable.template_name(),
            MeritSource::StackableMerit(_, stackable) => stackable.template_name(),
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
            MeritSource::Manse(_, _, _) => Some(BookReference::new(Book::CoreRulebook, 163)),
            MeritSource::MartialArtist(_, _) => Some(BookReference::new(Book::CoreRulebook, 163)),
            MeritSource::MortalSorcerer => Some(BookReference::new(Book::CoreRulebook, 470)),
            MeritSource::NonStackableMerit(_, nonstackable) => nonstackable.book_reference(),
            MeritSource::StackableMerit(_, stackable) => stackable.book_reference(),
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
            MeritSource::NonStackableMerit(_, _) => None,
            MeritSource::StackableMerit(_, stackable) => Some(stackable.detail()),
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
            MeritSource::Manse(_, _, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => 3,
                GeomancyLevel::Greater => 5,
            },
            MeritSource::MartialArtist(_, _) => 4,
            MeritSource::MortalSorcerer => 5,
            MeritSource::NonStackableMerit(_, nonstackable) => nonstackable.dots(),
            MeritSource::StackableMerit(_, stackable) => stackable.dots(),
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
            MeritSource::MartialArtist(_, _) => MeritType::Purchased,
            MeritSource::MortalSorcerer => MeritType::Story,
            MeritSource::NonStackableMerit(_, nonstackable) => nonstackable.merit_type(),
            MeritSource::StackableMerit(_, stackable) => stackable.merit_type(),
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
            MeritSource::Manse(_, _, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => (MANSE_SHARED, Some(MANSE_STANDARD)),
                GeomancyLevel::Greater => (MANSE_SHARED, Some(MANSE_GREATER)),
            },
            MeritSource::MartialArtist(_, _) => (MARTIAL_ARTIST, None),
            MeritSource::MortalSorcerer => (MORTAL_SORCERY, None),
            MeritSource::NonStackableMerit(_, nonstackable) => nonstackable.description(),
            MeritSource::StackableMerit(_, stackable) => stackable.description(),
        }
    }
}