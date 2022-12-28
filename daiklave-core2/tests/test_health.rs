use daiklave_core2::{
    Character, CharacterEventSource, CharacterMutation, CharacterView, DamageLevel, WoundPenalty,
};

#[test]
fn test_health_character() {
    // Check default health
    let mut character = Character::default();
    let mut count = 0;
    let expected = vec![
        (WoundPenalty::Zero, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusFour, None),
        (WoundPenalty::Incapacitated, None),
    ];
    for ((wound_penalty, damage), &(expected_wound_penalty, expected_damage)) in
        character.health().iter().zip(expected.iter())
    {
        count += 1;
        assert_eq!(wound_penalty, expected_wound_penalty);
        assert_eq!(damage, expected_damage);
    }
    assert_eq!(count, 7);
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::Zero
    );

    // Check modifying health boxes
    let new_wound_penalties = vec![
        WoundPenalty::Zero,
        WoundPenalty::Zero,
        WoundPenalty::MinusOne,
        WoundPenalty::MinusOne,
        WoundPenalty::MinusTwo,
        WoundPenalty::MinusTwo,
        WoundPenalty::MinusFour,
        WoundPenalty::Incapacitated,
    ];
    assert!(character
        .check_set_wound_penalties(&new_wound_penalties)
        .is_ok());
    assert!(character.set_wound_penalties(&new_wound_penalties).is_ok());
    let mut count = 0;
    let expected = vec![
        (WoundPenalty::Zero, None),
        (WoundPenalty::Zero, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusFour, None),
        (WoundPenalty::Incapacitated, None),
    ];
    for ((wound_penalty, damage), &(expected_wound_penalty, expected_damage)) in
        character.health().iter().zip(expected.iter())
    {
        count += 1;
        assert_eq!(wound_penalty, expected_wound_penalty);
        assert_eq!(damage, expected_damage);
    }
    assert_eq!(count, 8);
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::Zero
    );

    // Check taking damage
    assert!(character.check_take_damage(DamageLevel::Bashing, 3).is_ok());
    assert!(character.take_damage(DamageLevel::Bashing, 3).is_ok());
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::MinusOne
    );
    assert!(character.check_take_damage(DamageLevel::Lethal, 2).is_ok());
    assert!(character.take_damage(DamageLevel::Lethal, 2).is_ok());
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::MinusTwo
    );
    assert!(character
        .check_take_damage(DamageLevel::Aggravated, 2)
        .is_ok());
    assert!(character.take_damage(DamageLevel::Aggravated, 2).is_ok());
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::MinusFour
    );
    assert!(character.check_take_damage(DamageLevel::Bashing, 1).is_ok());
    assert!(character.take_damage(DamageLevel::Bashing, 1).is_ok());

    let mut count = 0;
    let expected = vec![
        (WoundPenalty::Zero, Some(DamageLevel::Aggravated)),
        (WoundPenalty::Zero, Some(DamageLevel::Aggravated)),
        (WoundPenalty::MinusOne, Some(DamageLevel::Lethal)),
        (WoundPenalty::MinusOne, Some(DamageLevel::Lethal)),
        (WoundPenalty::MinusTwo, Some(DamageLevel::Bashing)),
        (WoundPenalty::MinusTwo, Some(DamageLevel::Bashing)),
        (WoundPenalty::MinusFour, Some(DamageLevel::Bashing)),
        (WoundPenalty::Incapacitated, Some(DamageLevel::Bashing)),
    ];
    for ((wound_penalty, damage), &(expected_wound_penalty, expected_damage)) in
        character.health().iter().zip(expected.iter())
    {
        count += 1;
        assert_eq!(wound_penalty, expected_wound_penalty);
        assert_eq!(damage, expected_damage);
    }
    assert_eq!(count, 8);
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::Incapacitated
    );

    // Check healing
    assert!(character.check_heal_damage(2).is_ok());
    assert!(character.heal_damage(2).is_ok());
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::MinusTwo
    );
    assert!(character.check_heal_damage(3).is_ok());
    assert!(character.heal_damage(3).is_ok());
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::MinusOne
    );
    assert!(character.check_heal_damage(3).is_ok());
    assert!(character.heal_damage(3).is_ok());
    let mut count = 0;
    let expected = vec![
        (WoundPenalty::Zero, None),
        (WoundPenalty::Zero, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusFour, None),
        (WoundPenalty::Incapacitated, None),
    ];
    for ((wound_penalty, damage), &(expected_wound_penalty, expected_damage)) in
        character.health().iter().zip(expected.iter())
    {
        count += 1;
        assert_eq!(wound_penalty, expected_wound_penalty);
        assert_eq!(damage, expected_damage);
    }
    assert_eq!(count, 8);
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::Zero
    );
}

#[test]
fn test_health_character_view() {
    // Check default health
    let mut character_view = CharacterView::default();
    let mut count = 0;
    let expected = vec![
        (WoundPenalty::Zero, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusFour, None),
        (WoundPenalty::Incapacitated, None),
    ];
    for ((wound_penalty, damage), &(expected_wound_penalty, expected_damage)) in
        character_view.health().iter().zip(expected.iter())
    {
        count += 1;
        assert_eq!(wound_penalty, expected_wound_penalty);
        assert_eq!(damage, expected_damage);
    }
    assert_eq!(count, 7);
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::Zero
    );

    // Check modifying health boxes
    let new_wound_penalties = vec![
        WoundPenalty::Zero,
        WoundPenalty::Zero,
        WoundPenalty::MinusOne,
        WoundPenalty::MinusOne,
        WoundPenalty::MinusTwo,
        WoundPenalty::MinusTwo,
        WoundPenalty::MinusFour,
        WoundPenalty::Incapacitated,
    ];
    assert!(character_view
        .check_set_wound_penalties(&new_wound_penalties)
        .is_ok());
    assert!(character_view
        .set_wound_penalties(&new_wound_penalties)
        .is_ok());
    let mut count = 0;
    let expected = vec![
        (WoundPenalty::Zero, None),
        (WoundPenalty::Zero, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusFour, None),
        (WoundPenalty::Incapacitated, None),
    ];
    for ((wound_penalty, damage), &(expected_wound_penalty, expected_damage)) in
        character_view.health().iter().zip(expected.iter())
    {
        count += 1;
        assert_eq!(wound_penalty, expected_wound_penalty);
        assert_eq!(damage, expected_damage);
    }
    assert_eq!(count, 8);
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::Zero
    );

    // Check taking damage
    assert!(character_view
        .check_take_damage(DamageLevel::Bashing, 3)
        .is_ok());
    assert!(character_view.take_damage(DamageLevel::Bashing, 3).is_ok());
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::MinusOne
    );
    assert!(character_view
        .check_take_damage(DamageLevel::Lethal, 2)
        .is_ok());
    assert!(character_view.take_damage(DamageLevel::Lethal, 2).is_ok());
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::MinusTwo
    );
    assert!(character_view
        .check_take_damage(DamageLevel::Aggravated, 2)
        .is_ok());
    assert!(character_view
        .take_damage(DamageLevel::Aggravated, 2)
        .is_ok());
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::MinusFour
    );
    assert!(character_view
        .check_take_damage(DamageLevel::Bashing, 1)
        .is_ok());
    assert!(character_view.take_damage(DamageLevel::Bashing, 1).is_ok());

    let mut count = 0;
    let expected = vec![
        (WoundPenalty::Zero, Some(DamageLevel::Aggravated)),
        (WoundPenalty::Zero, Some(DamageLevel::Aggravated)),
        (WoundPenalty::MinusOne, Some(DamageLevel::Lethal)),
        (WoundPenalty::MinusOne, Some(DamageLevel::Lethal)),
        (WoundPenalty::MinusTwo, Some(DamageLevel::Bashing)),
        (WoundPenalty::MinusTwo, Some(DamageLevel::Bashing)),
        (WoundPenalty::MinusFour, Some(DamageLevel::Bashing)),
        (WoundPenalty::Incapacitated, Some(DamageLevel::Bashing)),
    ];
    for ((wound_penalty, damage), &(expected_wound_penalty, expected_damage)) in
        character_view.health().iter().zip(expected.iter())
    {
        count += 1;
        assert_eq!(wound_penalty, expected_wound_penalty);
        assert_eq!(damage, expected_damage);
    }
    assert_eq!(count, 8);
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::Incapacitated
    );

    // Check healing
    assert!(character_view.check_heal_damage(2).is_ok());
    assert!(character_view.heal_damage(2).is_ok());
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::MinusTwo
    );
    assert!(character_view.check_heal_damage(3).is_ok());
    assert!(character_view.heal_damage(3).is_ok());
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::MinusOne
    );
    assert!(character_view.check_heal_damage(3).is_ok());
    assert!(character_view.heal_damage(3).is_ok());
    let mut count = 0;
    let expected = vec![
        (WoundPenalty::Zero, None),
        (WoundPenalty::Zero, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusFour, None),
        (WoundPenalty::Incapacitated, None),
    ];
    for ((wound_penalty, damage), &(expected_wound_penalty, expected_damage)) in
        character_view.health().iter().zip(expected.iter())
    {
        count += 1;
        assert_eq!(wound_penalty, expected_wound_penalty);
        assert_eq!(damage, expected_damage);
    }
    assert_eq!(count, 8);
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::Zero
    );
}

#[test]
fn test_health_character_event_source() {
    // Check default health
    let mut event_source = CharacterEventSource::default();
    let character_view = event_source.as_character_view().unwrap();
    let mut count = 0;
    let expected = vec![
        (WoundPenalty::Zero, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusFour, None),
        (WoundPenalty::Incapacitated, None),
    ];
    for ((wound_penalty, damage), &(expected_wound_penalty, expected_damage)) in
        character_view.health().iter().zip(expected.iter())
    {
        count += 1;
        assert_eq!(wound_penalty, expected_wound_penalty);
        assert_eq!(damage, expected_damage);
    }
    assert_eq!(count, 7);
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::Zero
    );

    // Check modifying health boxes
    let new_wound_penalties = vec![
        WoundPenalty::Zero,
        WoundPenalty::Zero,
        WoundPenalty::MinusOne,
        WoundPenalty::MinusOne,
        WoundPenalty::MinusTwo,
        WoundPenalty::MinusTwo,
        WoundPenalty::MinusFour,
        WoundPenalty::Incapacitated,
    ];
    let mutation = CharacterMutation::SetWoundPenalties(new_wound_penalties);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();

    let mut count = 0;
    let expected = vec![
        (WoundPenalty::Zero, None),
        (WoundPenalty::Zero, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusFour, None),
        (WoundPenalty::Incapacitated, None),
    ];
    for ((wound_penalty, damage), &(expected_wound_penalty, expected_damage)) in
        character_view.health().iter().zip(expected.iter())
    {
        count += 1;
        assert_eq!(wound_penalty, expected_wound_penalty);
        assert_eq!(damage, expected_damage);
    }
    assert_eq!(count, 8);
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::Zero
    );

    // Check taking damage
    let mutation = CharacterMutation::TakeDamage(DamageLevel::Bashing, 3);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::MinusOne
    );

    let mutation = CharacterMutation::TakeDamage(DamageLevel::Lethal, 2);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::MinusTwo
    );

    let mutation = CharacterMutation::TakeDamage(DamageLevel::Aggravated, 2);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::MinusFour
    );

    let mutation = CharacterMutation::TakeDamage(DamageLevel::Bashing, 1);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    let mut count = 0;
    let expected = vec![
        (WoundPenalty::Zero, Some(DamageLevel::Aggravated)),
        (WoundPenalty::Zero, Some(DamageLevel::Aggravated)),
        (WoundPenalty::MinusOne, Some(DamageLevel::Lethal)),
        (WoundPenalty::MinusOne, Some(DamageLevel::Lethal)),
        (WoundPenalty::MinusTwo, Some(DamageLevel::Bashing)),
        (WoundPenalty::MinusTwo, Some(DamageLevel::Bashing)),
        (WoundPenalty::MinusFour, Some(DamageLevel::Bashing)),
        (WoundPenalty::Incapacitated, Some(DamageLevel::Bashing)),
    ];
    for ((wound_penalty, damage), &(expected_wound_penalty, expected_damage)) in
        character_view.health().iter().zip(expected.iter())
    {
        count += 1;
        assert_eq!(wound_penalty, expected_wound_penalty);
        assert_eq!(damage, expected_damage);
    }
    assert_eq!(count, 8);
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::Incapacitated
    );

    // Check healing
    let mutation = CharacterMutation::HealDamage(2);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::MinusTwo
    );

    let mutation = CharacterMutation::HealDamage(3);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::MinusOne
    );

    let mutation = CharacterMutation::HealDamage(3);
    assert!(character_view.check_mutation(&mutation).is_ok());
    assert!(event_source.apply_mutation(mutation).is_ok());
    let character_view = event_source.as_character_view().unwrap();
    let mut count = 0;
    let expected = vec![
        (WoundPenalty::Zero, None),
        (WoundPenalty::Zero, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusOne, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusTwo, None),
        (WoundPenalty::MinusFour, None),
        (WoundPenalty::Incapacitated, None),
    ];
    for ((wound_penalty, damage), &(expected_wound_penalty, expected_damage)) in
        character_view.health().iter().zip(expected.iter())
    {
        count += 1;
        assert_eq!(wound_penalty, expected_wound_penalty);
        assert_eq!(damage, expected_damage);
    }
    assert_eq!(count, 8);
    assert_eq!(
        character_view.health().current_wound_penalty(),
        WoundPenalty::Zero
    );

    // Check we can undo the full history
    assert!(!event_source.can_redo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(event_source.undo());
    assert!(!event_source.can_undo());

    // Check we can redo the full history
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(event_source.redo());
    assert!(!event_source.can_redo());
}