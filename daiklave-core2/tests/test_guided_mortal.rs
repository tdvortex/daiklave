use daiklave_core2::{
    guided::{begin_guided_builder, ExaltationChoice, GuidedMutation, GuidedStage},
    id::{CharacterId, Id},
    AttributeName, CharacterMutation,
};

#[test]
fn test_guided_mortal() {
    let mut guided_builder = begin_guided_builder(CharacterId(Id::Placeholder(1)));

    // Choose character name
    let mutation =
        GuidedMutation::CharacterMutation(CharacterMutation::SetName("Test Mortal".to_owned()));
    assert!(guided_builder.check_mutation(&mutation).is_ok());
    assert!(guided_builder.apply_mutation(mutation).is_ok());

    // Choose character concept
    let mutation =
        GuidedMutation::CharacterMutation(CharacterMutation::SetConcept("Test Concept".to_owned()));
    assert!(guided_builder.check_mutation(&mutation).is_ok());
    assert!(guided_builder.apply_mutation(mutation).is_ok());

    // Move on to next stage
    let mutation = GuidedMutation::SetStage(GuidedStage::ChooseExaltation);
    assert!(guided_builder.check_mutation(&mutation).is_ok());
    assert!(guided_builder.apply_mutation(mutation).is_ok());

    // Move back to previous stage and undo adding concept
    assert!(guided_builder.can_undo());
    assert!(guided_builder.undo());
    assert!(guided_builder.can_undo());
    assert!(guided_builder.undo());

    // Move on to next stage (again)
    let mutation = GuidedMutation::SetStage(GuidedStage::ChooseExaltation);
    assert!(guided_builder.check_mutation(&mutation).is_ok());
    assert!(guided_builder.apply_mutation(mutation).is_ok());

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
    assert!(guided_builder.check_mutation(&mutation).is_ok());
    assert!(guided_builder.apply_mutation(mutation).is_ok());

    let mutation = GuidedMutation::SetStage(GuidedStage::ChooseAttributes);
    assert!(guided_builder.check_mutation(&mutation).is_ok());
    assert!(guided_builder.apply_mutation(mutation).is_ok());

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
    .for_each(|gcm| assert!(guided_builder.apply_mutation(gcm).is_ok()));

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
    let mutation = GuidedMutation::SetStage(GuidedStage::ChooseMartialArtsStyles);
    assert!(guided_builder.check_mutation(&mutation).is_ok());
    assert!(guided_builder.apply_mutation(mutation).is_ok());
}
