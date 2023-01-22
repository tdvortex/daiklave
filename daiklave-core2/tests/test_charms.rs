use std::num::NonZeroU8;

use daiklave_core2::{
    abilities::{AbilityName, AbilityNameVanilla},
    book_reference::{Book, BookReference},
    charms::{
        charm::{Charm, CharmId, CharmMutation},
        CharmActionType, CharmCostType,
    },
    exaltation::exalt::exalt_type::solar::{
        caste::TwilightAbility,
        charm::{SolarCharm, SolarCharmAbility, SolarCharmId, SolarCharmKeyword},
        Solar,
    },
    unique_id::UniqueId,
    CharacterEventSource, CharacterMutation,
};

#[test]
fn test_solar_charms() {
    let mut event_source = CharacterEventSource::default();
    let character = event_source.as_character().unwrap();
    assert!(character.charms().iter().next().is_none());

    // Mortals cannot add Solar Charms, even if they meet the ability prerequisites
    event_source
        .apply_mutation(CharacterMutation::SetAbilityDots(
            AbilityNameVanilla::Archery,
            5,
        ))
        .unwrap();
    let wise_arrow = SolarCharm::builder("Wise Arrow".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 255))
        .cost(CharmCostType::Motes, NonZeroU8::new(1).unwrap())
        .essence_required(NonZeroU8::new(1).unwrap())
        .ability_required(SolarCharmAbility::Archery, 2)
        .action_type(CharmActionType::Supplemental)
        .keyword(SolarCharmKeyword::Uniform)
        .duration("Instant".to_owned())
        .description("With skill and effor, the Exalt guides her arrow to its mark[...]".to_owned())
        .summary("Shoot around cover".to_owned())
        .build();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(
            SolarCharmId(UniqueId::Placeholder(1)),
            wise_arrow.clone()
        )))
        .is_err());

    // Solars must meet ability requirements
    [
        AbilityNameVanilla::Archery,
        AbilityNameVanilla::Dodge,
        AbilityNameVanilla::Thrown,
        AbilityNameVanilla::War,
        AbilityNameVanilla::Stealth,
    ]
    .into_iter()
    .for_each(|ability_name| {
        event_source
            .apply_mutation(CharacterMutation::SetAbilityDots(ability_name, 1))
            .unwrap();
    });

    let new_solar = Solar::builder()
        .twilight()
        .caste_ability(TwilightAbility::Bureaucracy)
        .caste_ability(TwilightAbility::Integrity)
        .caste_ability(TwilightAbility::Craft)
        .caste_ability(TwilightAbility::Investigation)
        .supernal_ability(TwilightAbility::Lore)
        .favored_ability(AbilityName::Archery)
        .favored_ability(AbilityName::Dodge)
        .favored_ability(AbilityName::War)
        .favored_ability(AbilityName::Thrown)
        .favored_ability(AbilityName::Stealth)
        .limit_trigger("A limit trigger".to_owned())
        .build()
        .unwrap();

    event_source
        .apply_mutation(CharacterMutation::SetSolar(new_solar))
        .unwrap();

    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(
            SolarCharmId(UniqueId::Placeholder(1)),
            wise_arrow.clone()
        )))
        .is_err());

    event_source
        .apply_mutation(CharacterMutation::SetAbilityDots(
            AbilityNameVanilla::Archery,
            3,
        ))
        .unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(
            SolarCharmId(UniqueId::Placeholder(1)),
            wise_arrow.clone(),
        )))
        .unwrap();
    let Charm::Solar(owned_wise_arrow) = character.charms().get(CharmId::Solar(SolarCharmId(UniqueId::Placeholder(1)))).unwrap() else {panic!("Wrong charm type");};
    assert_eq!(owned_wise_arrow, &wise_arrow);

    let character = event_source
        .apply_mutation(CharacterMutation::SetAbilityDots(
            AbilityNameVanilla::Archery,
            1,
        ))
        .unwrap();
    assert!(character
        .charms()
        .get(CharmId::Solar(SolarCharmId(UniqueId::Placeholder(1))))
        .is_none());

    event_source.undo().unwrap();

    // Solars must meet Essence requirements
    let some_expensive_charm = SolarCharm::builder("Some expensive charm".to_owned())
        .essence_required(NonZeroU8::new(2).unwrap())
        .ability_required(SolarCharmAbility::War, 2)
        .action_type(CharmActionType::Permanent)
        .duration("Indefinite".to_owned())
        .description("Some description".to_owned())
        .build();
    assert!(event_source
        .apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(
            SolarCharmId(UniqueId::Placeholder(2)),
            some_expensive_charm
        )))
        .is_err());

    // ...unless they have the ability as a Supernal ability
    event_source
        .apply_mutation(CharacterMutation::SetAbilityDots(
            AbilityNameVanilla::Lore,
            5,
        ))
        .unwrap();

    let order_affirming_blow = SolarCharm::builder("Order-Affirming Blow".to_owned())
    .book_reference(BookReference::new(Book::CoreRulebook, 334))
    .cost(CharmCostType::Motes, NonZeroU8::new(15).unwrap())
    .cost(CharmCostType::Willpower, NonZeroU8::new(1).unwrap())
    .essence_required(NonZeroU8::new(3).unwrap())
    .ability_required(SolarCharmAbility::Lore, 5)
    .action_type(CharmActionType::Simple)
    .duration("Instant".to_owned())
    .description("The ravages of the Wyld and alterations to the Loom of Fate can be reversed by the Solar Exalted[...]".to_owned())
    .build();

    event_source
        .apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(
            SolarCharmId(UniqueId::Placeholder(3)),
            order_affirming_blow,
        )))
        .unwrap();

    // Solars must meet Charm tree requirements
    event_source
        .apply_mutation(CharacterMutation::RemoveCharm(CharmId::Solar(
            SolarCharmId(UniqueId::Placeholder(1)),
        )))
        .unwrap();

    let sight_without_eyes = SolarCharm::builder("Sight Without Eyes".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 255))
        .charm_prerequisite(SolarCharmId(UniqueId::Placeholder(1)))
        .essence_required(NonZeroU8::new(1).unwrap())
        .ability_required(SolarCharmAbility::Archery, 3)
        .cost(CharmCostType::Motes, NonZeroU8::new(1).unwrap())
        .action_type(CharmActionType::Reflexive)
        .duration("One tick".to_owned())
        .description("The Exalt opens her eyes not to the visual world[...]".to_owned())
        .summary("Ignore vision penalties".to_owned())
        .build();

    assert!(event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(SolarCharmId(UniqueId::Placeholder(4)), sight_without_eyes.clone()))).is_err());

    event_source.undo().unwrap();
    event_source.apply_mutation(CharacterMutation::AddCharm(CharmMutation::Solar(SolarCharmId(UniqueId::Placeholder(4)), sight_without_eyes))).unwrap();
    let character = event_source
        .apply_mutation(CharacterMutation::RemoveCharm(CharmId::Solar(
            SolarCharmId(UniqueId::Placeholder(1)),
        )))
        .unwrap();

    assert!(character.charms().get(CharmId::Solar(SolarCharmId(UniqueId::Placeholder(4)))).is_none());
}

#[test]
fn test_evocations() {
    // Mortals cannot add Evocations, even if they have the artifact/hearthstone
    // Exalts must meet Essence requirements
    // Exalts must have the right artifact or hearthstone
    // Exalts must meet tree requirements
}

#[test]
fn test_spells() {
    // Non-sorcerers cannot add Spells
    // Mortal sorcerers can add Terrestrial Spells
    // Solar Terrestrial sorcerers can add Terrestrial Spells
    // ...but not Celestial Spells or Solar circle spells
    // Solar Celestial sorcerer can add Terrestrial or Celestial spells
    // ...but not Solar circle spells
    // Solar Solar sorcerers can add any spell
}

#[test]
fn test_martial_arts_charms() {
    // Mortals cannot add MA charms, even if they have the right style
    // Exalts must have the right MA style
    // Exalts must meet the MA ability requirements of charms
    // Exalts must meet the Essence requirements of charms
    // ...unless they are Dawn Solars with Martial Arts Supernal
}

#[test]
fn test_eclipse_charms() {
    // Mortals cannot add Eclipse charms
    // Non-Eclipse Solars cannot add Eclipse charms
    // Eclipse Solars must meet the Essence requirement of Eclipse Charms
}
