use std::collections::HashSet;

use daiklave_core2::{
    abilities::AbilityNameVanilla,
    attributes::AttributeName,
    book_reference::{Book, BookReference},
    charms::{CharmCost, CharmCostType, CharmKeyword},
    guided::{ExaltationChoice, GuidedEventSource, GuidedMutation},
    martial_arts::{MartialArtsStyle, MartialArtsStyleId},
    sorcery::{
        ShapingRitual, ShapingRitualId, SorceryArchetype, SorceryArchetypeId, SorceryCircle, Spell,
        SpellId, TerrestrialSpell,
    },
    unique_id::UniqueId,
    weapons::BaseWeaponId,
    CharacterMutation,
};

#[test]
fn test_guided_mortal() {
    let mut guided_builder = GuidedEventSource::default();

    // Choose character name
    let mutation =
        GuidedMutation::CharacterMutation(CharacterMutation::SetName("Test Mortal".to_owned()));
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    // Choose character concept
    let mutation =
        GuidedMutation::CharacterMutation(CharacterMutation::SetConcept("Test Concept".to_owned()));
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    // Move on to next stage
    let mutation = GuidedMutation::AdvanceStage;
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    // Move back to previous stage and undo adding concept
    assert!(guided_builder.can_undo());
    assert!(guided_builder.undo());
    assert!(guided_builder.can_undo());
    assert!(guided_builder.undo());

    // Move on to next stage (again)
    let mutation = GuidedMutation::AdvanceStage;
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    // Bonus points are not alloted until after choosing exaltation
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        0
    );

    // Choose to be mortal and progress to attributes
    let mutation = GuidedMutation::SetExaltation(ExaltationChoice::Mortal);
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    let mutation = GuidedMutation::AdvanceStage;
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        21
    );

    // Set attributes without bonus points
    [
        CharacterMutation::SetAttribute(AttributeName::Strength, 4),
        CharacterMutation::SetAttribute(AttributeName::Dexterity, 3),
        CharacterMutation::SetAttribute(AttributeName::Stamina, 2),
        CharacterMutation::SetAttribute(AttributeName::Charisma, 3),
        CharacterMutation::SetAttribute(AttributeName::Manipulation, 2),
        CharacterMutation::SetAttribute(AttributeName::Appearance, 2),
        CharacterMutation::SetAttribute(AttributeName::Perception, 3),
        CharacterMutation::SetAttribute(AttributeName::Intelligence, 2),
        CharacterMutation::SetAttribute(AttributeName::Wits, 1),
    ]
    .into_iter()
    .map(|cm| GuidedMutation::CharacterMutation(cm))
    .for_each(|gcm| {
        guided_builder.apply_mutation(gcm).unwrap();
    });

    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        21
    );

    // Check attribute bonus points costs
    guided_builder
        .apply_mutation(GuidedMutation::CharacterMutation(
            CharacterMutation::SetAttribute(AttributeName::Stamina, 3),
        ))
        .unwrap();
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        17
    );

    guided_builder
        .apply_mutation(GuidedMutation::CharacterMutation(
            CharacterMutation::SetAttribute(AttributeName::Manipulation, 3),
        ))
        .unwrap();
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        13
    );

    guided_builder
        .apply_mutation(GuidedMutation::CharacterMutation(
            CharacterMutation::SetAttribute(AttributeName::Wits, 2),
        ))
        .unwrap();
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        10
    );

    // Revert attribute bonus point expenditures
    guided_builder.undo();
    guided_builder.undo();
    guided_builder.undo();
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        21
    );

    // Move on to the next stage
    let mutation = GuidedMutation::AdvanceStage;
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    // Add a martial arts style
    let crane_style = MartialArtsStyle::new(
        Some(BookReference::new(Book::CoreRulebook, 443)),
        "Crane Style".to_owned(),
        "Crane style is a defensive style, emulating the grace of the \
        crane in avoiding the blows of an enemy. Its students learn \
        not just to fight with physical blows, but to empathize \
        with her enemy, speaking or debating with him in an \
        attempt to bring the fight to an end without violence. \
        However, those who mistake the Crane master's restraint \
        for weakness find themselves quickly meeting the ground. \
        When she must, a student of this style can unleash \
        devastating counterattacks, flowing with the force of an \
        enemy's blow so she can strike back in turn. \n\
        Crane Weapons: Crane style practitioners typically dual \
        wield a war fan and hook sword, using the fan for defense \
        while disarming enemies with the sword. Unarmed attacks \
        usually consist of graceful kicks, but a Crane stylist lacking \
        his usual weapons might use one hand to deliver rapid \
        chops while holding back the other for powerful lunges \
        and sweeping blows. \n \
        Armor: Crane style is incompatible with armor. \n \
        Complementary Abilities: Many Crane stylists use \
        Presence, Performance, or Socialize in combat to sway \
        their opponents into peaceful resolution or compromise, \
        and later Charms of this style empower such efforts."
            .to_owned(),
        HashSet::from([
            BaseWeaponId(UniqueId::Placeholder(1)),
            BaseWeaponId(UniqueId::Placeholder(2)),
            BaseWeaponId(UniqueId::Placeholder(3)),
        ]),
        None,
    );

    let mutation = GuidedMutation::AddMartialArtsStyle(
        MartialArtsStyleId(UniqueId::Placeholder(1)),
        crane_style.clone(),
    );
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    // Check can't add martial arts style with the same Id
    let mutation = GuidedMutation::AddMartialArtsStyle(
        MartialArtsStyleId(UniqueId::Placeholder(1)),
        crane_style,
    );
    assert!(guided_builder.check_mutation(&mutation).is_err());

    // Remove a martial arts style
    let mutation =
        GuidedMutation::RemoveMartialArtsStyle(MartialArtsStyleId(UniqueId::Placeholder(1)));
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    // Check can't remove absent martial arts style
    let mutation =
        GuidedMutation::RemoveMartialArtsStyle(MartialArtsStyleId(UniqueId::Placeholder(1)));
    assert!(guided_builder.check_mutation(&mutation).is_err());

    // Undo removal
    assert!(guided_builder.can_undo());
    assert!(guided_builder.undo());

    // Check martial arts counts against merits budget
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        21
    );

    let dummy_style = MartialArtsStyle::new(
        None,
        "Dummy style".to_owned(),
        "Dummy description".to_owned(),
        HashSet::from([BaseWeaponId(UniqueId::Placeholder(1))]),
        None,
    );
    let mutation = GuidedMutation::AddMartialArtsStyle(
        MartialArtsStyleId(UniqueId::Placeholder(2)),
        dummy_style,
    );
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        20
    );

    // Undo dummy style
    assert!(guided_builder.can_undo());
    assert!(guided_builder.undo());

    // Move on to the next stage
    let mutation = GuidedMutation::AdvanceStage;
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    // Check we can skip ahead directly to abilities
    let mutation = GuidedMutation::AdvanceStage;
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    // Undo, go back to sorcery
    guided_builder.undo();

    // Add sorcery archetype
    let archetype = SorceryArchetype::new(
        "Bargain with Mara".to_owned(),
        Some(BookReference::new(Book::CoreRulebook, 466)),
        "You have met the demon Mara, the deer-footed creature \
        of shadows who pursues lovers marked by dark destinies \
        and feeds them stolen souls. You may have met her as she \
        sojourned through Creation, or seen her lovely, wicked \
        face in dreams—perhaps you even summoned her yourself \
        to make your bargain. Through cunning, charisma, or \
        pleasing offerings, you have made a pact with her, taking \
        the power of the Shadow Lover for your own."
            .to_owned(),
    );
    let archetype_id = SorceryArchetypeId(UniqueId::Placeholder(1));

    let mutation = GuidedMutation::SetSorceryArchetype(archetype_id, archetype);
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    // Add shaping ritual
    let shaping_ritual_description = "Like Mara herself, the sorcerer draws power from those who \
        love her. The Essence of their adoration is clay in \
        her hands, taking form in her sorcery. Whenever she \
        takes a shape sorcery action while within medium range \
        of a character with an unrequited Tie of love (or a similar \
        emotion) towards her, she may reap additional sorcerous \
        motes equal to that Intimacy's value (for example, four motes \
        from a Defining Tie). Intimacies can only be tapped for \
        motes in this fashion once per day, and the sorcerer \
        cannot harvest power from love she reciprocates. Normally \
        she may only drain one Intimacy to fuel the casting of a spell, \
        but she may draw power from any available Intimacies when \
        casting her control spell. She cannot draw more than ten \
        sorcerous motes per scene with this ritual.";
    let shaping_ritual = ShapingRitual::new(
        archetype_id,
        Some(BookReference::new(Book::CoreRulebook, 467)),
        shaping_ritual_description.to_owned(),
    );
    let shaping_ritual_id = ShapingRitualId(UniqueId::Placeholder(1));

    let mutation = GuidedMutation::SetShapingRitual(shaping_ritual_id, shaping_ritual);
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    // Add control spell
    let control_spell = TerrestrialSpell::from_spell(Spell::new(
        "Corrupted Words".to_owned(),
        Some(BookReference::new(Book::CoreRulebook, 472)),
        vec![
            CharmCost::new(CharmCostType::SorcerousMotes, 15),
            CharmCost::new(CharmCostType::Willpower, 1),
        ],
        vec![CharmKeyword::Psyche],
        "Indefinite".to_owned(),
        "Really long spell description".to_owned(),
    ));
    let control_spell_id = SpellId(UniqueId::Placeholder(1));

    let mutation = GuidedMutation::SetControlSpell(control_spell_id, control_spell);
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    // Sorcery should be a 5-dot merit for mortals
    assert_eq!(guided_builder.as_guided_view().unwrap().merit_dots(), 9);

    // Move on to the next stage
    let mutation = GuidedMutation::AdvanceStage;
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    // Brawl must be 1 to allow MartialArtist
    // Occult must be 3 to allow MortalSorcerer merit
    let guided_view = guided_builder.as_guided_view().unwrap();
    let character_view = guided_view.as_character_view();
    assert_eq!(
        character_view
            .abilities()
            .get(AbilityNameVanilla::Brawl)
            .dots(),
        1
    );
    assert_eq!(
        character_view
            .abilities()
            .get(AbilityNameVanilla::Occult)
            .dots(),
        3
    );

    // Check you can't reduce Brawl to 0 or Occult below 3 if you have Martial
    // Arts styles or Sorcery
    assert!(guided_builder
        .check_mutation(&GuidedMutation::CharacterMutation(
            CharacterMutation::SetAbilityDots(AbilityNameVanilla::Brawl, 0)
        ))
        .is_err());
    assert!(guided_builder
        .check_mutation(&GuidedMutation::CharacterMutation(
            CharacterMutation::SetAbilityDots(AbilityNameVanilla::Occult, 2)
        ))
        .is_err());

    // Check you can't add dots to an unowned Martial Arts style
    assert!(guided_builder
        .check_mutation(&GuidedMutation::CharacterMutation(
            CharacterMutation::SetMartialArtsDots(
                MartialArtsStyleId(UniqueId::Placeholder(666)),
                1
            )
        ))
        .is_err());

    // Check purchasing dots above 3 costs bonus points even if free dots remain
    let mutation = GuidedMutation::CharacterMutation(CharacterMutation::SetAbilityDots(
        AbilityNameVanilla::Awareness,
        4,
    ));
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        17
    );

    let mutation = GuidedMutation::CharacterMutation(CharacterMutation::SetMartialArtsDots(
        MartialArtsStyleId(UniqueId::Placeholder(1)),
        4,
    ));
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .as_character_view()
            .martial_arts()
            .style(MartialArtsStyleId(UniqueId::Placeholder(1)))
            .unwrap()
            .ability()
            .dots(),
        4
    );
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        15
    );

    let mutation = GuidedMutation::CharacterMutation(CharacterMutation::SetCraftDots(
        "Armoring".to_owned(),
        4,
    ));
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        13
    );

    // Check purchasing 28 dots of abilities, all 3 or below, costs no bonus points
    let mutation = GuidedMutation::CharacterMutation(CharacterMutation::SetMartialArtsDots(
        MartialArtsStyleId(UniqueId::Placeholder(1)),
        3,
    ));
    guided_builder.apply_mutation(mutation).unwrap();
    let mutation = GuidedMutation::CharacterMutation(CharacterMutation::SetCraftDots(
        "Armoring".to_owned(),
        3,
    ));
    guided_builder.apply_mutation(mutation).unwrap();
    [
        (AbilityNameVanilla::Archery, 3),
        (AbilityNameVanilla::Athletics, 3),
        (AbilityNameVanilla::Awareness, 3),
        (AbilityNameVanilla::Brawl, 3),
        (AbilityNameVanilla::Bureaucracy, 3),
        (AbilityNameVanilla::Dodge, 3),
        (AbilityNameVanilla::Occult, 3),
        (AbilityNameVanilla::Integrity, 1),
    ]
    .into_iter()
    .map(|(ability_name, dots)| {
        GuidedMutation::CharacterMutation(CharacterMutation::SetAbilityDots(ability_name, dots))
    })
    .fold(&mut guided_builder, |builder, mutation| {
        builder.apply_mutation(mutation).unwrap()
    });
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        19
    );

    // Check purchasing more than 28 dots of abilities costs bonus points even at 3 or below
    let mutation = GuidedMutation::CharacterMutation(CharacterMutation::SetAbilityDots(
        AbilityNameVanilla::Integrity,
        2,
    ));
    guided_builder.apply_mutation(mutation).unwrap();
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        17
    );

    // Check can't advance with less than 28 free dots (even with 28 total dots)
    let mutation = GuidedMutation::CharacterMutation(CharacterMutation::SetAbilityDots(
        AbilityNameVanilla::Archery,
        1,
    ));
    guided_builder.apply_mutation(mutation).unwrap();
    assert!(guided_builder
        .check_mutation(&GuidedMutation::AdvanceStage)
        .is_err());

    guided_builder.undo();
    guided_builder.undo();

    // Move on to the next stage
    let mutation = GuidedMutation::AdvanceStage;
    guided_builder.check_mutation(&mutation).unwrap();
    guided_builder.apply_mutation(mutation).unwrap();

    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        19
    );

    // After advancing, should have actual Martial Arts and Sorcery in character view
    let guided_view = guided_builder.as_guided_view().unwrap();
    let character_view = guided_view.as_character_view();
    assert_eq!(
        character_view
            .martial_arts()
            .style(MartialArtsStyleId(UniqueId::Placeholder(1)))
            .unwrap()
            .name(),
        "Crane Style"
    );
    assert_eq!(
        character_view
            .martial_arts()
            .style(MartialArtsStyleId(UniqueId::Placeholder(1)))
            .unwrap()
            .ability()
            .dots(),
        3
    );
    assert_eq!(
        character_view
            .sorcery()
            .unwrap()
            .archetype(SorceryArchetypeId(UniqueId::Placeholder(1)))
            .unwrap()
            .name(),
        "Bargain with Mara"
    );
    assert_eq!(
        character_view
            .sorcery()
            .unwrap()
            .shaping_ritual(SorceryCircle::Terrestrial)
            .unwrap()
            .1
            .description(),
        shaping_ritual_description
    );
    assert_eq!(
        character_view
            .sorcery()
            .unwrap()
            .control_spell(SorceryCircle::Terrestrial)
            .unwrap()
            .1
            .name(),
        "Corrupted Words"
    );
}
