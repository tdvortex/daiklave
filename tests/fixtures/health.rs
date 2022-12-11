use exalted_3e_gui::{
    character::CharacterBuilder,
    health::{DamageLevel, Health, WoundPenalty},
};

pub fn create_initial_health(builder: CharacterBuilder) -> CharacterBuilder {
    builder
        .with_wound_penalties(vec![
            WoundPenalty::Incapacitated,
            WoundPenalty::MinusFour,
            WoundPenalty::MinusTwo,
            WoundPenalty::MinusTwo,
            WoundPenalty::MinusTwo,
            WoundPenalty::MinusTwo,
            WoundPenalty::MinusOne,
            WoundPenalty::MinusOne,
            WoundPenalty::MinusOne,
            WoundPenalty::Zero,
        ])
        .with_damage(2, 3, 1)
}

pub fn validate_initial_health(health: &Health) {
    assert_eq!(
        health
            .health_boxes()
            .iter()
            .map(|hbox| { (hbox.wound_penalty(), hbox.damage()) })
            .collect::<Vec<_>>(),
        vec![
            (WoundPenalty::Zero, DamageLevel::Aggravated),
            (WoundPenalty::MinusOne, DamageLevel::Lethal),
            (WoundPenalty::MinusOne, DamageLevel::Lethal),
            (WoundPenalty::MinusOne, DamageLevel::Lethal),
            (WoundPenalty::MinusTwo, DamageLevel::Bashing),
            (WoundPenalty::MinusTwo, DamageLevel::Bashing),
            (WoundPenalty::MinusTwo, DamageLevel::None),
            (WoundPenalty::MinusTwo, DamageLevel::None),
            (WoundPenalty::MinusFour, DamageLevel::None),
            (WoundPenalty::Incapacitated, DamageLevel::None)
        ]
    );
}

pub fn modify_health(health: &mut Health) {
    // Add health box
    health.add_health_box(WoundPenalty::Zero);
    // Remove health box
    health.remove_health_box(WoundPenalty::MinusTwo).unwrap();
    // Heal damage
    health.heal_all_damage();
}

pub fn validate_modified_health(health: &Health) {
    assert_eq!(
        health
            .health_boxes()
            .iter()
            .map(|hbox| { (hbox.wound_penalty(), hbox.damage()) })
            .collect::<Vec<_>>(),
        vec![
            (WoundPenalty::Zero, DamageLevel::None),
            (WoundPenalty::Zero, DamageLevel::None),
            (WoundPenalty::MinusOne, DamageLevel::None),
            (WoundPenalty::MinusOne, DamageLevel::None),
            (WoundPenalty::MinusOne, DamageLevel::None),
            (WoundPenalty::MinusTwo, DamageLevel::None),
            (WoundPenalty::MinusTwo, DamageLevel::None),
            (WoundPenalty::MinusTwo, DamageLevel::None),
            (WoundPenalty::MinusFour, DamageLevel::None),
            (WoundPenalty::Incapacitated, DamageLevel::None)
        ]
    );
}