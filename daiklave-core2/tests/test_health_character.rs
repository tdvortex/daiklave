use daiklave_core2::{
    health::{DamageLevel, WoundPenalty},
    Character,
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
    character
        .check_set_wound_penalties(&new_wound_penalties)
        .unwrap();
    character.set_wound_penalties(&new_wound_penalties).unwrap();
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
    character
        .check_take_damage(DamageLevel::Bashing, 3)
        .unwrap();
    character.take_damage(DamageLevel::Bashing, 3).unwrap();
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::MinusOne
    );
    character.check_take_damage(DamageLevel::Lethal, 2).unwrap();
    character.take_damage(DamageLevel::Lethal, 2).unwrap();
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::MinusTwo
    );
    character
        .check_take_damage(DamageLevel::Aggravated, 2)
        .unwrap();
    character.take_damage(DamageLevel::Aggravated, 2).unwrap();
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::MinusFour
    );
    character
        .check_take_damage(DamageLevel::Bashing, 1)
        .unwrap();
    character.take_damage(DamageLevel::Bashing, 1).unwrap();

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
    character.check_heal_damage(2).unwrap();
    character.heal_damage(2).unwrap();
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::MinusTwo
    );
    character.check_heal_damage(3).unwrap();
    character.heal_damage(3).unwrap();
    assert_eq!(
        character.health().current_wound_penalty(),
        WoundPenalty::MinusOne
    );
    character.check_heal_damage(3).unwrap();
    character.heal_damage(3).unwrap();
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
