use daiklave_core2::{
    abilities::{AbilityName, AbilityNameQualified, AbilityNameVanilla},
    armor::armor_item::ArmorWeightClass,
    attributes::AttributeName,
    book_reference::{Book, BookReference},
    exaltation::exalt::exalt_type::solar::caste::EclipseAbility,
    martial_arts::style::AddMartialArtsStyle,
    mutations::{SetConcept, SetName, SetSolar},
    sorcery::{AddSorcery, SorceryCircle},
    CharacterEvent, CharacterEventSource, CharacterMutationError,
};
use std::num::NonZeroU8;

#[test]
fn lib_doctest() {
    lib_doctest_inner().unwrap()
}

fn lib_doctest_inner() -> Result<(), CharacterMutationError> {
    // First, create an event source
    let mut event_source = CharacterEventSource::default();

    // Give the character a name
    let character = SetName("Horizon Dancer".into()).apply_event(&mut event_source)?;
    assert_eq!(character.name(), "Horizon Dancer");

    // Give the character a short concept
    assert_eq!(character.concept(), None);
    let character =
        SetConcept("A demonstration character".into()).apply_event(&mut event_source)?;
    assert_eq!(character.concept(), Some("A demonstration character"));

    // Pick an Exalt type and caste
    SetSolar::eclipse()
        .caste_ability(EclipseAbility::Linguistics)
        .caste_ability(EclipseAbility::Occult)
        .caste_ability(EclipseAbility::Presence)
        .caste_ability(EclipseAbility::Sail)
        .caste_ability(EclipseAbility::Socialize)
        .supernal_ability(EclipseAbility::Occult)
        .favored_ability(AbilityName::Awareness)
        .favored_ability(AbilityName::Brawl)
        .favored_ability(AbilityName::Lore)
        .favored_ability(AbilityName::Medicine)
        .favored_ability(AbilityName::Performance)
        .limit_trigger("Betraying someone's trust")
        .build()?
        .apply_event(&mut event_source)?;

    // Set attributes
    let attributes = [
        (AttributeName::Appearance, 4),
        (AttributeName::Charisma, 4),
        (AttributeName::Manipulation, 3),
        (AttributeName::Dexterity, 4),
        (AttributeName::Strength, 2),
        (AttributeName::Stamina, 3),
        (AttributeName::Intelligence, 2),
        (AttributeName::Wits, 2),
        (AttributeName::Perception, 3),
    ];

    for (name, dots) in attributes {
        name.set_dots(dots)?.apply_event(&mut event_source)?;
    }
    let character = event_source.as_character()?;
    assert_eq!(
        character.attributes().get(AttributeName::Appearance).dots(),
        4
    );

    // Set abilities, except for MartialArts
    let abilities = [
        (AbilityNameVanilla::Athletics, 1),
        (AbilityNameVanilla::Awareness, 2),
        (AbilityNameVanilla::Brawl, 1),
        (AbilityNameVanilla::Dodge, 2),
        (AbilityNameVanilla::Integrity, 1),
        (AbilityNameVanilla::Investigation, 1),
        (AbilityNameVanilla::Linguistics, 2),
        (AbilityNameVanilla::Lore, 2),
        (AbilityNameVanilla::Medicine, 1),
        (AbilityNameVanilla::Occult, 3),
        (AbilityNameVanilla::Performance, 2),
        (AbilityNameVanilla::Presence, 2),
        (AbilityNameVanilla::Sail, 2),
        (AbilityNameVanilla::Socialize, 3),
    ];

    for (ability_name, dots) in abilities {
        AbilityNameQualified::from(ability_name)
            .set_dots(dots)?
            .apply_event(&mut event_source)?;
    }
    let character = event_source.as_character()?;
    assert_eq!(
        character
            .abilities()
            .get(AbilityNameQualified::from(AbilityNameVanilla::Linguistics))
            .unwrap()
            .dots(),
        2
    );

    // Add a martial arts style and give it some dots
    AddMartialArtsStyle::name("Single Point Shining Into the Void Style")
        .book_reference(BookReference::new(Book::CoreRulebook, 434))
        .description(
            "Single Point Shining Into the Void is a sword style that \
    emphasizes blinding speed and deadly-perfect finishing \
    moves.",
        )
        .weapon("Slashing Sword")
        .weapon("Reaper Daiklave")
        .max_armor_weight(ArmorWeightClass::Medium)
        .apply_event(&mut event_source)?;
    AbilityNameQualified::MartialArts("Single Point Shining Into the Void Style")
        .set_dots(3)?
        .apply_event(&mut event_source)?;
    let character = event_source.as_character()?;
    assert_eq!(
        character
            .merits()
            .iter()
            .find(|merit| {
                merit.name() == "Martial Artist"
                    && matches!(
                        merit.detail(),
                        Some("Single Point Shining Into the Void Style")
                    )
            })
            .unwrap()
            .dots(),
        4
    );
    assert_eq!(
        character
            .abilities()
            .get(AbilityNameQualified::MartialArts(
                "Single Point Shining Into the Void Style"
            ))
            .unwrap()
            .dots(),
        3
    );

    // Add Sorcery
    AddSorcery::terrestrial_circle()
        .archetype_name("Pact with an Ifrit Lord")
        .book_reference(BookReference::new(Book::CoreRulebook, 467))
        .description("You have stood in the court of one of the ifrits or another elemental lord of fire[...]")
        .shaping_ritual_summary("Gain motes by extinguishing flames")
        .description("Whenever the sorcerer takes a shape sorcery action, she \
            may draw an additional (Essence) sorcerous motes from \
            any fire within medium range, coaxing its power into her \
            spell[...]")
        .control_spell_name("Cirrus Skiff")
        .book_reference(BookReference::new(Book::CoreRulebook, 471))
        .sorcerous_motes(NonZeroU8::new(15).unwrap())
        .willpower(NonZeroU8::new(1).unwrap())
        .duration("Until ended")
        .summary("Summon a cloud to ride on")
        .description("The sorcerer calls down a Cirrus Skiff to bear her skyward, \
            a small white puffy cloud just large enough for her and \
            one other passenger to ride upon[...]")
        .control_spell_description("A character who knows Cirrus Skiff as her control spell \
            may cast it with an Indefinite duration[...]")
        .distortion(NonZeroU8::new(7).unwrap(), "Distorting a Cirrus Skiff \
        weighs it down, turning the cloud into a heavy, dense fog \
        for a scene[...]")
        .apply_event(&mut event_source)?;

    let character = event_source.as_character()?;
    assert!(character
        .sorcery()
        .unwrap()
        .archetype("Pact with an Ifrit Lord")
        .is_some());
    assert!(character
        .sorcery()
        .unwrap()
        .shaping_ritual(SorceryCircle::Terrestrial)
        .is_some());
    assert_eq!(
        character
            .sorcery()
            .unwrap()
            .control_spell(SorceryCircle::Terrestrial)
            .unwrap()
            .name(),
        "Cirrus Skiff"
    );

    // Set native language
    // Add a second language
    // Add an artifact
    // Add a hearthstone and manse
    // Add a stackable merit
    // Add a nonstackable merit
    // Add a sorcery archetype merit
    // Add a Solar Charm
    // Add an Eclipse Charm
    // Add a Martial Arts Charm
    // Add a Spell
    // Add an Evocation
    // Raise Willpower rating
    Ok(())
}
