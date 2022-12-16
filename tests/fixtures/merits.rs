use exalted_3e_gui::{
    abilities::AbilityNameNoSubskill,
    attributes::AttributeName,
    character::CharacterBuilder,
    id::Id,
    merits::{Merit, MeritTemplate, MeritType},
    prerequisite::PrerequisiteSet,
};

const MARTIAL_ARTIST_DESCRIPTION: &str = "\
    The character has undergone systematic training in at least one formal \
combat art. Perhaps she was raised as an Immaculate monk, or studied in \
a dojo during her journeys. This Merit allows the character to purchase \
the Martial Arts Ability.";

const DANGER_SENSE_DESCRIPTION: &str = "\
    An indefinable “sixth sense” warns the character when \
    she is in danger—a definite asset in the Time of Tumult! \
    She enjoys a bonus die on rolls to detect danger.";

const LANGUAGE_DESCRIPTION: &str = "\
Each purchase grants the character fluency in one \
    language in addition to his native tongue. If the character possesses \
    Linguistics 1+, then they are also literate in any written forms of \
    the language.\n\
    The `civilized` inhabitants of each of the outer Directions \
    speak a different language (actually a number of closely-related \
    regional dialects), while the Scavenger Lands has its own particular \
    language and the mighty Realm has two official languages. There are \
    also countless local languages that are only spoken by tribal or \
    ethnic groups spread around Creation's rim.";

pub fn create_initial_merits(builder: CharacterBuilder) -> CharacterBuilder {
    let character_id_placeholder = builder.id();
    let martial_artist_template =
        MeritTemplate::from_book(Id::Placeholder(0), "Core Rulebook".to_owned(), 163)
            .with_name("Martial Artist".to_owned())
            .with_merit_type(MeritType::Purchased)
            .with_description(MARTIAL_ARTIST_DESCRIPTION.to_owned())
            .requiring_detail()
            .with_prerequisite_set(
                PrerequisiteSet::create()
                    .requiring_ability(AbilityNameNoSubskill::Brawl, 1)
                    .build(),
            )
            .build()
            .unwrap();

    let danger_sense_template =
        MeritTemplate::from_book(Id::Placeholder(1), "Core Rulebook".to_owned(), 160)
            .with_name("Danger Sense".to_owned())
            .with_merit_type(MeritType::Innate)
            .with_description(DANGER_SENSE_DESCRIPTION.to_owned())
            .not_requiring_detail()
            .with_prerequisite_set(
                PrerequisiteSet::create()
                    .requiring_attribute(AttributeName::Perception, 3)
                    .build(),
            )
            .with_prerequisite_set(
                PrerequisiteSet::create()
                    .requiring_ability(AbilityNameNoSubskill::Awareness, 3)
                    .build(),
            )
            .build()
            .unwrap();

    let language_template =
        MeritTemplate::from_book(Id::Placeholder(2), "Core Rulebook".to_owned(), 162)
            .with_name("Language".to_owned())
            .with_merit_type(MeritType::Purchased)
            .with_description(LANGUAGE_DESCRIPTION.to_owned())
            .requiring_detail()
            .build()
            .unwrap();

    let custom_template = MeritTemplate::custom(Id::Placeholder(3), character_id_placeholder)
        .with_name("Test Custom Merit Template".to_owned())
        .with_merit_type(MeritType::Supernatural)
        .with_description("Test Custom Merit Template Description".to_owned())
        .not_requiring_detail()
        .build()
        .unwrap();

    builder
        .with_merit(
            martial_artist_template,
            4,
            Some("Single Point Shining Into Void Style".to_owned()),
            Id::Placeholder(0),
        )
        .unwrap()
        .with_merit(danger_sense_template, 3, None, Id::Placeholder(1))
        .unwrap()
        .with_merit(
            language_template,
            2,
            Some("Low Realm(Native), Flametongue, Riverspeak".to_owned()),
            Id::Placeholder(2),
        )
        .unwrap()
        .with_merit(custom_template, 1, None, Id::Placeholder(3))
        .unwrap()
}

pub fn validate_initial_merits(merits: &Vec<Merit>, should_have_id: bool) {
    [
        (
            "Martial Artist",
            MeritType::Purchased,
            Some("Single Point Shining Into Void Style"),
            MARTIAL_ARTIST_DESCRIPTION,
            true,
            &vec![PrerequisiteSet::create()
                .requiring_ability(AbilityNameNoSubskill::Brawl, 1)
                .build()],
            4,
        ),
        (
            "Danger Sense",
            MeritType::Innate,
            None,
            DANGER_SENSE_DESCRIPTION,
            false,
            &vec![
                PrerequisiteSet::create()
                    .requiring_attribute(AttributeName::Perception, 3)
                    .build(),
                PrerequisiteSet::create()
                    .requiring_ability(AbilityNameNoSubskill::Awareness, 3)
                    .build(),
            ],
            3,
        ),
        (
            "Language",
            MeritType::Purchased,
            Some("Low Realm(Native), Flametongue, Riverspeak"),
            LANGUAGE_DESCRIPTION,
            true,
            &vec![],
            2,
        ),
        (
            "Test Custom Merit Template",
            MeritType::Supernatural,
            None,
            "Test Custom Merit Template Description",
            false,
            &vec![],
            1,
        ),
    ]
    .into_iter()
    .zip(merits.iter())
    .for_each(|(expected, actual)| {
        assert_eq!(!actual.instance_id().is_placeholder(), should_have_id);
        assert_eq!(!actual.template_id().is_placeholder(), should_have_id);
        assert_eq!(expected.0, actual.name());
        assert_eq!(expected.1, actual.merit_type());
        assert_eq!(expected.2, actual.detail());
        assert_eq!(expected.3, actual.description());
        assert_eq!(expected.4, actual.requires_detail());
        assert_eq!(expected.5.len(), actual.prerequisites().len());
        for (expected_set, actual_set) in expected.5.iter().zip(actual.prerequisites().iter()) {
            assert_eq!(!actual_set.id().is_placeholder(), should_have_id);
            assert_eq!(expected_set.len(), actual_set.len());
            for (expected_prerequisite, actual_prerequisite) in
                expected_set.iter().zip(actual_set.iter())
            {
                assert_eq!(
                    expected_prerequisite.prerequisite_type(),
                    actual_prerequisite.prerequisite_type()
                );
            }
        }
        assert_eq!(expected.6, actual.dots());
    });
}

pub fn modify_merits(merits: &mut Vec<Merit>) {
    // Add merit
    let artifact_template =
        MeritTemplate::from_book(Id::Placeholder(4), "Core Rulebook".to_owned(), 159)
            .with_name("Artifact".to_owned())
            .requiring_detail()
            .with_description(
                "The character owns a magical item—see Chapter Nine for more details.".to_owned(),
            )
            .with_merit_type(MeritType::Story)
            .build()
            .unwrap();

    let screamer_merit = Merit::from_template(
        artifact_template,
        3,
        Some("Screamer (Red Jade Reaper Daiklave)".to_owned()),
        Id::Placeholder(4),
    )
    .unwrap();

    merits.push(screamer_merit);

    // Remove merit
    merits.remove(1);
}

pub fn validate_modified_merits(merits: &Vec<Merit>) {
    [
        (
            "Martial Artist",
            MeritType::Purchased,
            Some("Single Point Shining Into Void Style"),
            MARTIAL_ARTIST_DESCRIPTION,
            true,
            &vec![PrerequisiteSet::create()
                .requiring_ability(AbilityNameNoSubskill::Brawl, 1)
                .build()],
            4,
        ),
        (
            "Language",
            MeritType::Purchased,
            Some("Low Realm(Native), Flametongue, Riverspeak"),
            LANGUAGE_DESCRIPTION,
            true,
            &vec![],
            2,
        ),
        (
            "Test Custom Merit Template",
            MeritType::Supernatural,
            None,
            "Test Custom Merit Template Description",
            false,
            &vec![],
            1,
        ),
        (
            "Artifact",
            MeritType::Story,
            Some("Screamer (Red Jade Reaper Daiklave)"),
            "The character owns a magical item—see Chapter Nine for more details.",
            true,
            &vec![],
            3,
        ),
    ]
    .into_iter()
    .zip(merits.iter())
    .for_each(|(expected, actual)| {
        assert_eq!(expected.0, actual.name());
        assert_eq!(expected.1, actual.merit_type());
        assert_eq!(expected.2, actual.detail());
        assert_eq!(expected.3, actual.description());
        assert_eq!(expected.4, actual.requires_detail());
        assert_eq!(expected.5.len(), actual.prerequisites().len());
        for (expected_set, actual_set) in expected.5.iter().zip(actual.prerequisites().iter()) {
            assert_eq!(expected_set.len(), actual_set.len());
            for (expected_prerequisite, actual_prerequisite) in
                expected_set.iter().zip(actual_set.iter())
            {
                assert_eq!(
                    expected_prerequisite.prerequisite_type(),
                    actual_prerequisite.prerequisite_type()
                );
            }
        }
        assert_eq!(expected.6, actual.dots());
    });
}
