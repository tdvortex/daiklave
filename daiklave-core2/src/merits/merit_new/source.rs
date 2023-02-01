use crate::{
    book_reference::{Book, BookReference},
    hearthstones::hearthstone::GeomancyLevel,
    languages::language::MajorLanguage,
};

use super::{
    instance::{
        SorceryArchetypeMerit, ARTIFACT_FIVE, ARTIFACT_FOUR, ARTIFACT_NA, ARTIFACT_SHARED,
        ARTIFACT_THREE, ARTIFACT_TWO, DEMENSE_GREATER, DEMENSE_SHARED, DEMENSE_STANDARD,
        EXALTED_HEALING, EXALTED_HEALING_EXALT, LANGUAGE_DRAGONTONGUE, LANGUAGE_FLAMETONGUE,
        LANGUAGE_FORESTTONGUE, LANGUAGE_GUILD_CANT, LANGUAGE_HIGH_REALM, LANGUAGE_LOCAL_TONGUES,
        LANGUAGE_LOW_REALM, LANGUAGE_OLD_REALM, LANGUAGE_RIVERSPEAK, LANGUAGE_SEATONGUE,
        LANGUAGE_SHARED, LANGUAGE_SKYTONGUE, MARTIAL_ARTIST, MORTAL_SORCERY, NonStackableMerit, StackableMerit, HEARTHSTONE_SHARED, HEARTHSTONE_MANSE_STANDARD, HEARTHSTONE_STANDARD, HEARTHSTONE_MANSE_GREATER, HEARTHSTONE_GREATER,
    },
    manse::{MANSE_GREATER, MANSE_SHARED, MANSE_STANDARD},
};

pub(crate) enum MeritSource<'source> {
    Artifact {
        name: &'source str,
        dots: u8,
    },
    Demense {
        name: &'source str,
        has_manse: bool,
        geomancy_level: GeomancyLevel,
    },
    ExaltedHealing {
        is_exalt: bool,
    },
    Hearthstone {
        name: &'source str,
        has_manse: bool,
        geomancy_level: GeomancyLevel,
    },
    LocalTongues {
        count: usize,
    },
    MajorLanguage(MajorLanguage),
    Manse {
        name: &'source str,
        geomancy_level: GeomancyLevel,
    },
    MartialArtist {
        style_name: &'source str,
    },
    MortalSorcerer,
    NonStackable(NonStackableMerit<'source>),
    SorceryArchetype(SorceryArchetypeMerit<'source>),
    Stackable(StackableMerit<'source>),
}

impl<'source> MeritSource<'source> {
    pub fn name(&self) -> &'source str {
        match self {
            MeritSource::Artifact { name, dots } => "Artifact",
            MeritSource::Demense {
                name,
                has_manse,
                geomancy_level,
            } => "Demense",
            MeritSource::ExaltedHealing { is_exalt } => "Exalted Healing",
            MeritSource::Hearthstone { name, has_manse, geomancy_level } => "Hearthstone",
            MeritSource::LocalTongues { count } => "Language",
            MeritSource::MajorLanguage(major_language) => "Language",
            MeritSource::Manse {
                name,
                geomancy_level,
            } => "Manse",
            MeritSource::MartialArtist { style_name } => "Martial Artist",
            MeritSource::MortalSorcerer => "Mortal Sorcerer",
            MeritSource::NonStackable(nonstackable) => nonstackable.name(),
            MeritSource::SorceryArchetype(sorcery_archetype_merit) => {
                sorcery_archetype_merit.name()
            }
            MeritSource::Stackable(stackable) => stackable.name(),
        }
    }

    pub fn detail(&self) -> Option<&'source str> {
        match self {
            MeritSource::Artifact { name, dots } => Some(*name),
            MeritSource::Demense {
                name,
                has_manse,
                geomancy_level,
            } => Some(*name),
            MeritSource::ExaltedHealing { is_exalt } => None,
            MeritSource::Hearthstone { name, has_manse, geomancy_level } => Some(*name),
            MeritSource::LocalTongues { count } => None,
            MeritSource::MajorLanguage(major_language) => Some(match major_language {
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
            }),
            MeritSource::Manse {
                name,
                geomancy_level,
            } => Some(*name),
            MeritSource::MartialArtist { style_name } => Some(*style_name),
            MeritSource::MortalSorcerer => None,
            MeritSource::NonStackable(nonstackable) => None,
            MeritSource::SorceryArchetype(sorcery_archetype_merit) => {
                sorcery_archetype_merit.detail()
            }
            MeritSource::Stackable(stackable) => stackable.detail(),
        }
    }

    pub fn book_reference(&self) -> Option<BookReference> {
        match self {
            MeritSource::Artifact { name, dots } => Some(BookReference {
                book: Book::CoreRulebook,
                page_number: 159,
            }),
            MeritSource::Demense {
                name,
                has_manse,
                geomancy_level,
            } => Some(BookReference {
                book: Book::CoreRulebook,
                page_number: 160,
            }),
            MeritSource::Hearthstone { name, has_manse, geomancy_level } => Some(BookReference {
                book: Book::CoreRulebook,
                page_number: 161,
            }),
            MeritSource::ExaltedHealing { is_exalt } => Some(BookReference {
                book: Book::CoreRulebook,
                page_number: 165,
            }),
            MeritSource::LocalTongues { count } => Some(BookReference {
                book: Book::CoreRulebook,
                page_number: 162,
            }),
            MeritSource::MajorLanguage(_) => Some(BookReference {
                book: Book::CoreRulebook,
                page_number: 162,
            }),
            MeritSource::Manse {
                name,
                geomancy_level,
            } => Some(BookReference {
                book: Book::CoreRulebook,
                page_number: 163,
            }),
            MeritSource::MartialArtist { style_name } => Some(BookReference {
                book: Book::CoreRulebook,
                page_number: 163,
            }),
            MeritSource::MortalSorcerer => Some(BookReference {
                book: Book::CoreRulebook,
                page_number: 470,
            }),
            MeritSource::NonStackable(nonstackable) => nonstackable.book_reference(),
            MeritSource::SorceryArchetype(sorcery_archetype_merit) => {
                sorcery_archetype_merit.book_reference()
            }
            MeritSource::Stackable(stackable) => stackable.book_reference(),
        }
    }

    pub fn dots(&self) -> u8 {
        match self {
            MeritSource::Artifact { name, dots } => *dots,
            MeritSource::Demense {
                name,
                has_manse,
                geomancy_level,
            } => {
                if *has_manse {
                    0
                } else {
                    match geomancy_level {
                        GeomancyLevel::Standard => 2,
                        GeomancyLevel::Greater => 4,
                    }
                }
            }
            MeritSource::ExaltedHealing { is_exalt } => {
                if *is_exalt {
                    0
                } else {
                    5
                }
            }
            MeritSource::Hearthstone {
                name,
                has_manse,
                geomancy_level,
            } => {
                if *has_manse {
                    0
                } else {
                    match geomancy_level {
                        GeomancyLevel::Standard => 2,
                        GeomancyLevel::Greater => 4,
                    }
                }
            }
            MeritSource::LocalTongues { count } => {
                let count = (*count).min(u8::MAX as usize) as u8;
                (count >> 2) + u8::from(count & 3 > 0)
            }
            MeritSource::MajorLanguage(_) => 1,
            MeritSource::Manse {
                name,
                geomancy_level,
            } => match geomancy_level {
                GeomancyLevel::Standard => 3,
                GeomancyLevel::Greater => 5,
            },
            MeritSource::MartialArtist { style_name } => 4,
            MeritSource::MortalSorcerer => 5,
            MeritSource::NonStackable(nonstackable) => nonstackable.dots(),
            MeritSource::SorceryArchetype(sorcery_archetype_merit) => {
                sorcery_archetype_merit.dots()
            }
            MeritSource::Stackable(stackable) => stackable.dots(),
        }
    }

    pub fn description(&self) -> &'source str {
        match self {
            MeritSource::Artifact { name, dots } => ARTIFACT_SHARED,
            MeritSource::Demense {
                name,
                has_manse,
                geomancy_level,
            } => DEMENSE_SHARED,
            MeritSource::ExaltedHealing { is_exalt } => EXALTED_HEALING,
            MeritSource::Hearthstone {
                name,
                has_manse,
                geomancy_level,
            } => HEARTHSTONE_SHARED,
            MeritSource::LocalTongues { count } => LANGUAGE_SHARED,
            MeritSource::MajorLanguage(_) => LANGUAGE_SHARED,
            MeritSource::Manse {
                name,
                geomancy_level,
            } => MANSE_SHARED,
            MeritSource::MartialArtist { style_name } => MARTIAL_ARTIST,
            MeritSource::MortalSorcerer => MORTAL_SORCERY,
            MeritSource::NonStackable(nonstackable) => nonstackable.description(),
            MeritSource::SorceryArchetype(sorcery_archetype_merit) => {
                sorcery_archetype_merit.description()
            }
            MeritSource::Stackable(stackable) => stackable.description(),
        }
    }

    pub fn dot_description(&self) -> Option<&'source str> {
        match self {
            MeritSource::Artifact { name, dots } => Some(match dots {
                2 => ARTIFACT_TWO,
                3 => ARTIFACT_THREE,
                4 => ARTIFACT_FOUR,
                5 => ARTIFACT_FIVE,
                _ => ARTIFACT_NA,
            }),
            MeritSource::Demense {
                name,
                has_manse,
                geomancy_level,
            } => Some(match geomancy_level {
                GeomancyLevel::Standard => DEMENSE_STANDARD,
                GeomancyLevel::Greater => DEMENSE_GREATER,
            }),
            MeritSource::ExaltedHealing { is_exalt } => {
                if *is_exalt {
                    Some(EXALTED_HEALING_EXALT)
                } else {
                    None
                }
            }
            MeritSource::Hearthstone {
                name,
                has_manse,
                geomancy_level,
            } => Some(match (geomancy_level, *has_manse) {
                (GeomancyLevel::Standard, true) => HEARTHSTONE_MANSE_STANDARD,
                (GeomancyLevel::Standard, false) => HEARTHSTONE_STANDARD,
                (GeomancyLevel::Greater, true) => HEARTHSTONE_MANSE_GREATER,
                (GeomancyLevel::Greater, false) => HEARTHSTONE_GREATER,
            }),
            MeritSource::LocalTongues { count } => Some(LANGUAGE_LOCAL_TONGUES),
            MeritSource::MajorLanguage(major_language) => Some(match major_language {
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
            MeritSource::Manse {
                name,
                geomancy_level,
            } => Some(match geomancy_level {
                GeomancyLevel::Standard => MANSE_STANDARD,
                GeomancyLevel::Greater => MANSE_GREATER,
            }),
            MeritSource::MartialArtist { style_name } => None,
            MeritSource::MortalSorcerer => None,
            MeritSource::NonStackable(nonstackable) => nonstackable.dot_description(),
            MeritSource::SorceryArchetype(sorcery_archetype_merit) => None,
            MeritSource::Stackable(stackable) => stackable.dot_description(),
        }
    }
}
