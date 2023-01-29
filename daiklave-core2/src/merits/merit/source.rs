use std::ops::Div;

use crate::{
    artifact::ArtifactName,
    book_reference::{Book, BookReference},
    hearthstones::hearthstone::GeomancyLevel,
    languages::language::MajorLanguage,
    sorcery::{SorceryArchetypeMerit},
};

use super::{
    constants::{
        artifact::{
            ARTIFACT_FIVE, ARTIFACT_FOUR, ARTIFACT_NA, ARTIFACT_SHARED, ARTIFACT_THREE,
            ARTIFACT_TWO,
        },
        demense::{DEMENSE_GREATER, DEMENSE_SHARED, DEMENSE_STANDARD},
        exalted_healing::EXALTED_HEALING,
        hearthstone::{
            HEARTHSTONE_GREATER, HEARTHSTONE_MANSE_GREATER, HEARTHSTONE_MANSE_STANDARD,
            HEARTHSTONE_SHARED, HEARTHSTONE_STANDARD,
        },
        languages::{
            LANGUAGE_DRAGONTONGUE, LANGUAGE_FLAMETONGUE, LANGUAGE_FORESTTONGUE,
            LANGUAGE_GUILD_CANT, LANGUAGE_HIGH_REALM, LANGUAGE_LOCAL_TONGUES, LANGUAGE_LOW_REALM,
            LANGUAGE_OLD_REALM, LANGUAGE_RIVERSPEAK, LANGUAGE_SEATONGUE, LANGUAGE_SHARED,
            LANGUAGE_SKYTONGUE,
        },
        manse::{MANSE_GREATER, MANSE_SHARED, MANSE_STANDARD},
        martial_arts::MARTIAL_ARTIST,
        mortal_sorcerer::MORTAL_SORCERY,
    },
    nonstackable::{NonStackableMeritView},
    stackable::StackableMeritView,
    MeritInstanceName, MeritType,
};

pub(crate) enum MeritSource<'source> {
    Artifact(ArtifactName<'source>, u8),
    DemenseNoManse(&'source str, GeomancyLevel),
    DemenseWithManse(&'source str, &'source str, GeomancyLevel), // Hearthstone name, demense name, geomancy level
    ExaltedHealing(bool),                                        // is_exalt
    HearthstoneNoManse(&'source str, GeomancyLevel),
    HearthstoneWithManse(&'source str, GeomancyLevel),
    LocalTongues(usize),
    MajorLanguage(MajorLanguage),
    Manse(&'source str, &'source str, GeomancyLevel), // Hearthstone name, manse name, geomancy level
    MartialArtist(&'source str),
    MortalSorcerer,
    NonStackable(&'source str, NonStackableMeritView<'source>),
    SorceryArchetype(&'source str, &'source SorceryArchetypeMerit),
    Stackable(&'source str, &'source str, StackableMeritView<'source>), // Template name, instance detail
}

impl<'source> MeritSource<'source> {
    pub fn name(&self) -> MeritInstanceName {
        match self {
            MeritSource::Artifact(artifact_name, _) => MeritInstanceName::Artifact(*artifact_name),
            MeritSource::DemenseNoManse(name, _) => MeritInstanceName::Demense(name),
            MeritSource::DemenseWithManse(_, demense_name, _) => {
                MeritInstanceName::Demense(*demense_name)
            }
            MeritSource::ExaltedHealing(_) => MeritInstanceName::ExaltedHealing,
            MeritSource::HearthstoneNoManse(hearthstone_name, _) => {
                MeritInstanceName::Hearthstone(*hearthstone_name)
            }
            MeritSource::HearthstoneWithManse(hearthstone_name, _) => {
                MeritInstanceName::Hearthstone(*hearthstone_name)
            }
            MeritSource::LocalTongues(_) => MeritInstanceName::LocalTongues,
            MeritSource::MajorLanguage(major) => MeritInstanceName::MajorLanguage(*major),
            MeritSource::Manse(_, manse_name, _) => MeritInstanceName::Manse(*manse_name),
            MeritSource::MartialArtist(style_name) => MeritInstanceName::MartialArtist(*style_name),
            MeritSource::MortalSorcerer => MeritInstanceName::MortalSorcerer,
            MeritSource::NonStackable(nonstackable_name, _) => {
                MeritInstanceName::NonStackable(*nonstackable_name)
            }
            MeritSource::Stackable(template_name, instance_name, _) => {
                MeritInstanceName::Stackable(*template_name, *&instance_name)
            }
            MeritSource::SorceryArchetype(merit_name, _) => MeritInstanceName::SorceryArchetype(*merit_name),
        }
    }

    // pub fn template_name(&self) -> &'source str {
    //     match self {
    //         MeritSource::Artifact(_, _) => "Artifact",
    //         MeritSource::DemenseNoManse(_, _) => "Demense",
    //         MeritSource::DemenseWithManse(_, _, _) => "Demense",
    //         MeritSource::ExaltedHealing(_) => "Exalted Healing",
    //         MeritSource::HearthstoneNoManse(_, _) => "Hearthstone",
    //         MeritSource::HearthstoneWithManse(_, _) => "Hearthstone",
    //         MeritSource::LocalTongues(_) => "Language",
    //         MeritSource::MajorLanguage(_) => "Language",
    //         MeritSource::Manse(_, _, _) => "Manse",
    //         MeritSource::MartialArtist(_) => "Martial Artist",
    //         MeritSource::MortalSorcerer => "Terrestrial Circle Sorcerer (Mortal)",
    //         MeritSource::NonStackable(_, nonstackable) => nonstackable.template_name(),
    //         MeritSource::Stackable(_, stackable) => stackable.template_name(),
    //         MeritSource::SorceryArchetype(_, sorcery_merit) => (*sorcery_merit).name(),
    //     }
    // }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            MeritSource::Artifact(_, _) => Some(BookReference::new(Book::CoreRulebook, 159)),
            MeritSource::DemenseNoManse(_, _) => Some(BookReference::new(Book::CoreRulebook, 160)),
            MeritSource::DemenseWithManse(_, _, _) => {
                Some(BookReference::new(Book::CoreRulebook, 160))
            }
            MeritSource::ExaltedHealing(_) => Some(BookReference::new(Book::CoreRulebook, 165)),
            MeritSource::HearthstoneNoManse(_, _) => {
                Some(BookReference::new(Book::CoreRulebook, 161))
            }
            MeritSource::HearthstoneWithManse(_, _) => {
                Some(BookReference::new(Book::CoreRulebook, 161))
            }
            MeritSource::LocalTongues(_) => Some(BookReference::new(Book::CoreRulebook, 162)),
            MeritSource::MajorLanguage(_) => Some(BookReference::new(Book::CoreRulebook, 162)),
            MeritSource::Manse(_, _, _) => Some(BookReference::new(Book::CoreRulebook, 163)),
            MeritSource::MartialArtist(_) => Some(BookReference::new(Book::CoreRulebook, 163)),
            MeritSource::MortalSorcerer => Some(BookReference::new(Book::CoreRulebook, 470)),
            MeritSource::NonStackable(_, nonstackable) => nonstackable.book_reference(),
            MeritSource::Stackable(_, _, stackable) => stackable.book_reference(),
            MeritSource::SorceryArchetype(_, sorcery_merit) => (*sorcery_merit).book_reference(),
        }
    }

    // pub fn detail(&self) -> Option<&'source str> {
    //     match self {
    //         MeritSource::Artifact(name, _) => match name {
    //             ArtifactName::Weapon(name)
    //             | ArtifactName::Armor(name)
    //             | ArtifactName::Wonder(name) => Some(*name),
    //         },
    //         MeritSource::DemenseNoManse(name, _) => Some(*name),
    //         MeritSource::DemenseWithManse(_, name, _) => Some(*name),
    //         MeritSource::ExaltedHealing(_) => None,
    //         MeritSource::HearthstoneNoManse(name, _) => Some(*name),
    //         MeritSource::HearthstoneWithManse(name, _) => Some(*name),
    //         MeritSource::Manse(_, detail, _) => Some(*detail),
    //         MeritSource::MartialArtist(style_name) => Some(*style_name),
    //         MeritSource::MortalSorcerer => None,
    //         MeritSource::NonStackable(_, _) => None,
    //         MeritSource::Stackable(_, stackable) => Some(stackable.detail()),
    //         MeritSource::LocalTongues(_) => Some("Local Tongues"),
    //         MeritSource::MajorLanguage(major) => Some(match major {
    //             MajorLanguage::Dragontongue => "Dragontongue",
    //             MajorLanguage::Flametongue => "Flametongue",
    //             MajorLanguage::ForestTongue => "Forest-Tongue",
    //             MajorLanguage::GuildCant => "Guild Cant",
    //             MajorLanguage::HighRealm => "High Realm",
    //             MajorLanguage::LowRealm => "Low Realm",
    //             MajorLanguage::OldRealm => "Old Realm",
    //             MajorLanguage::Riverspeak => "Riverspeak",
    //             MajorLanguage::Seatongue => "Seatongue",
    //             MajorLanguage::Skytongue => "Skytongue",
    //         }),
    //         MeritSource::SorceryArchetype(_, _) => None,
    //     }
    // }

    pub fn dots(&self) -> u8 {
        match self {
            MeritSource::Artifact(_, dots) => *dots,
            MeritSource::DemenseNoManse(_, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => 2,
                GeomancyLevel::Greater => 4,
            },
            MeritSource::DemenseWithManse(_, _, _) => 0,
            MeritSource::ExaltedHealing(is_exalt) => 5 * (1 - u8::from(*is_exalt)),
            MeritSource::HearthstoneNoManse(_, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => 2,
                GeomancyLevel::Greater => 4,
            },
            MeritSource::HearthstoneWithManse(_, _) => 0,
            MeritSource::LocalTongues(count) => ((*count).min(u8::MAX as usize) + 3).div(4) as u8,
            MeritSource::MajorLanguage(_) => 1,
            MeritSource::Manse(_, _, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => 3,
                GeomancyLevel::Greater => 5,
            },
            MeritSource::MartialArtist(_) => 4,
            MeritSource::MortalSorcerer => 5,
            MeritSource::NonStackable(_, nonstackable) => nonstackable.dots(),
            MeritSource::Stackable(_, _, stackable) => stackable.dots(),
            MeritSource::SorceryArchetype(_, sorcery_merit) => (*sorcery_merit).dots(),
        }
    }

    pub fn merit_type(&self) -> MeritType {
        match self {
            MeritSource::Artifact(_, _) => MeritType::Story,
            MeritSource::DemenseNoManse(_, _) => MeritType::Story,
            MeritSource::DemenseWithManse(_, _, _) => MeritType::Story,
            MeritSource::ExaltedHealing(_) => MeritType::Supernatural,
            MeritSource::HearthstoneNoManse(_, _) => MeritType::Story,
            MeritSource::HearthstoneWithManse(_, _) => MeritType::Story,
            MeritSource::Manse(_, _, _) => MeritType::Story,
            MeritSource::LocalTongues(_) => MeritType::Purchased,
            MeritSource::MajorLanguage(_) => MeritType::Purchased,
            MeritSource::MartialArtist(_) => MeritType::Purchased,
            MeritSource::MortalSorcerer => MeritType::Story,
            MeritSource::NonStackable(_, nonstackable) => nonstackable.merit_type(),
            MeritSource::Stackable(_, _, stackable) => stackable.merit_type(),
            MeritSource::SorceryArchetype(_, _) => MeritType::Story,
        }
    }

    pub fn description(&self) -> (&'source str, Option<&'source str>) {
        match self {
            MeritSource::Artifact(_, dots) => match dots {
                2 => (ARTIFACT_SHARED, Some(ARTIFACT_TWO)),
                3 => (ARTIFACT_SHARED, Some(ARTIFACT_THREE)),
                4 => (ARTIFACT_SHARED, Some(ARTIFACT_FOUR)),
                5 => (ARTIFACT_SHARED, Some(ARTIFACT_FIVE)),
                _ => (ARTIFACT_SHARED, Some(ARTIFACT_NA)),
            },
            MeritSource::DemenseNoManse(_, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => (DEMENSE_SHARED, Some(DEMENSE_STANDARD)),
                GeomancyLevel::Greater => (DEMENSE_SHARED, Some(DEMENSE_GREATER)),
            },
            MeritSource::DemenseWithManse(_, _, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => (DEMENSE_SHARED, Some(DEMENSE_STANDARD)),
                GeomancyLevel::Greater => (DEMENSE_SHARED, Some(DEMENSE_GREATER)),
            },
            MeritSource::ExaltedHealing(_) => (EXALTED_HEALING, None),
            MeritSource::HearthstoneNoManse(_, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => (HEARTHSTONE_SHARED, Some(HEARTHSTONE_STANDARD)),
                GeomancyLevel::Greater => (HEARTHSTONE_SHARED, Some(HEARTHSTONE_GREATER)),
            },
            MeritSource::HearthstoneWithManse(_, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => (HEARTHSTONE_SHARED, Some(HEARTHSTONE_MANSE_STANDARD)),
                GeomancyLevel::Greater => (HEARTHSTONE_SHARED, Some(HEARTHSTONE_MANSE_GREATER)),
            },
            MeritSource::LocalTongues(_) => (LANGUAGE_SHARED, Some(LANGUAGE_LOCAL_TONGUES)),
            MeritSource::MajorLanguage(major) => (
                LANGUAGE_SHARED,
                Some(match major {
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
                }),
            ),
            MeritSource::Manse(_, _, geomancy_level) => match geomancy_level {
                GeomancyLevel::Standard => (MANSE_SHARED, Some(MANSE_STANDARD)),
                GeomancyLevel::Greater => (MANSE_SHARED, Some(MANSE_GREATER)),
            },
            MeritSource::MartialArtist(_) => (MARTIAL_ARTIST, None),
            MeritSource::MortalSorcerer => (MORTAL_SORCERY, None),
            MeritSource::NonStackable(_, nonstackable) => nonstackable.description(),
            MeritSource::Stackable(_, _, stackable) => stackable.description(),
            MeritSource::SorceryArchetype(_, sorcery_merit) => {
                ((*sorcery_merit).description(), None)
            }
        }
    }
}
