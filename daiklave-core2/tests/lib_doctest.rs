use daiklave_core2::{
    abilities::{AbilityName, AbilityNameQualified, AbilityNameVanilla},
    armor::armor_item::ArmorWeightClass,
    attributes::AttributeName,
    exaltation::exalt::exalt_type::solar::caste::EclipseAbility,
    martial_arts::style::AddMartialArtsStyle,
    mutations::{SetConcept, SetName, SetSolar},
    CharacterEvent, CharacterEventSource, CharacterMutationError,
};

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

    // Add a martial arts style
    AddMartialArtsStyle::name("Single Point Shining Into the Void Style")
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

    Ok(())
}
