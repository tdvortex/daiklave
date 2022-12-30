use daiklave_core2::{
    guided::{begin_guided_builder, ExaltationChoice, GuidedMutation, GuidedStage},
    id::{CharacterId, Id},
    AbilityName, AttributeName, CharacterMutation,
};

#[test]
fn test_guided_solar() {
    let mut guided_builder = begin_guided_builder(CharacterId(Id::Placeholder(1)));

    // Choose character name
    let mutation =
        GuidedMutation::CharacterMutation(CharacterMutation::SetName("Test Solar".to_owned()));
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
    let mutation = GuidedMutation::SetExaltation(ExaltationChoice::Night);
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
        15
    );

    // Set attributes without bonus points
    [
        CharacterMutation::SetAttribute(AttributeName::Strength, 2),
        CharacterMutation::SetAttribute(AttributeName::Dexterity, 5),
        CharacterMutation::SetAttribute(AttributeName::Stamina, 2),
        CharacterMutation::SetAttribute(AttributeName::Charisma, 2),
        CharacterMutation::SetAttribute(AttributeName::Manipulation, 3),
        CharacterMutation::SetAttribute(AttributeName::Appearance, 2),
        CharacterMutation::SetAttribute(AttributeName::Perception, 4),
        CharacterMutation::SetAttribute(AttributeName::Intelligence, 4),
        CharacterMutation::SetAttribute(AttributeName::Wits, 3),
    ]
    .into_iter()
    .map(|cm| GuidedMutation::CharacterMutation(cm))
    .for_each(|gcm| assert!(guided_builder.apply_mutation(gcm).is_ok()));

    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        15
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
        11
    );

    guided_builder
        .apply_mutation(GuidedMutation::CharacterMutation(
            CharacterMutation::SetAttribute(AttributeName::Wits, 4),
        ))
        .unwrap();
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        7
    );

    guided_builder
        .apply_mutation(GuidedMutation::CharacterMutation(
            CharacterMutation::SetAttribute(AttributeName::Appearance, 3),
        ))
        .unwrap();
    assert_eq!(
        guided_builder
            .as_guided_view()
            .unwrap()
            .bonus_points_remaining(),
        4
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
        15
    );

    // Move on to next stage
    let mutation = GuidedMutation::SetStage(GuidedStage::ChooseSolarCasteAbilities);
    guided_builder.check_mutation(&mutation).unwrap();
    assert!(guided_builder.apply_mutation(mutation).is_ok());

    // Check cannot add an invalid caste ability
    assert!(guided_builder
        .check_mutation(&GuidedMutation::AddSolarCasteAbility(AbilityName::Archery))
        .is_err());

    // Check can add 5 valid caste abilities
    assert!(guided_builder
        .check_mutation(&GuidedMutation::AddSolarCasteAbility(
            AbilityName::Athletics
        ))
        .is_ok());
    assert!(guided_builder
        .apply_mutation(GuidedMutation::AddSolarCasteAbility(AbilityName::Athletics))
        .is_ok());
    assert!(guided_builder
        .apply_mutation(GuidedMutation::AddSolarCasteAbility(AbilityName::Awareness))
        .is_ok());
    assert!(guided_builder
        .apply_mutation(GuidedMutation::AddSolarCasteAbility(AbilityName::Dodge))
        .is_ok());
    assert!(guided_builder
        .apply_mutation(GuidedMutation::AddSolarCasteAbility(
            AbilityName::Investigation
        ))
        .is_ok());
    assert!(guided_builder
        .apply_mutation(GuidedMutation::AddSolarCasteAbility(AbilityName::Larceny))
        .is_ok());

    // Check cannot add a 6th caste ability
    assert!(guided_builder
        .check_mutation(&GuidedMutation::AddSolarCasteAbility(AbilityName::Ride))
        .is_err());

    // Check can remove a caste ability
    assert!(guided_builder
        .check_mutation(&GuidedMutation::RemoveSolarCasteAbility(
            AbilityName::Investigation
        ))
        .is_ok());
    assert!(guided_builder
        .apply_mutation(GuidedMutation::RemoveSolarCasteAbility(
            AbilityName::Investigation
        ))
        .is_ok());

    // Check cannot remove a missing caste ability
    assert!(guided_builder
        .check_mutation(&GuidedMutation::RemoveSolarCasteAbility(AbilityName::Ride))
        .is_err());

    // Check cannot proceed without 5 caste abilities
    assert!(guided_builder
        .check_mutation(&GuidedMutation::SetStage(
            GuidedStage::ChooseSolarSupernalAbility
        ))
        .is_err());

    // Move on to next stage
    assert!(guided_builder
        .apply_mutation(GuidedMutation::AddSolarCasteAbility(AbilityName::Stealth))
        .is_ok());
    let mutation = GuidedMutation::SetStage(GuidedStage::ChooseSolarSupernalAbility);
    assert!(guided_builder.check_mutation(&mutation).is_ok());
    assert!(guided_builder.apply_mutation(mutation).is_ok());

    // Check you cannot choose a supernal ability that is not an owned caste ability
    assert!(guided_builder
        .check_mutation(&GuidedMutation::SetSolarSupernalAbility(
            AbilityName::Socialize
        ))
        .is_err());

    // Move on to next stage
    assert!(guided_builder
        .check_mutation(&GuidedMutation::SetSolarSupernalAbility(AbilityName::Dodge))
        .is_ok());
    assert!(guided_builder
        .apply_mutation(GuidedMutation::SetSolarSupernalAbility(AbilityName::Dodge))
        .is_ok());
    let mutation = GuidedMutation::SetStage(GuidedStage::ChooseSolarFavoredAbilities);
    assert!(guided_builder.check_mutation(&mutation).is_ok());
    assert!(guided_builder.apply_mutation(mutation).is_ok());

    // Check you cannot add a favored ability that is already a caste ability
    assert!(guided_builder
        .check_mutation(&GuidedMutation::AddSolarFavoredAbility(
            AbilityName::Athletics
        ))
        .is_err());

    // Check you cannot add MartialArts as a favored ability
    assert!(guided_builder
        .check_mutation(&GuidedMutation::AddSolarFavoredAbility(
            AbilityName::MartialArts
        ))
        .is_err());

    // Check can add 5 valid favored abilities
    guided_builder
        .check_mutation(&GuidedMutation::AddSolarFavoredAbility(
            AbilityName::Linguistics,
        ))
        .unwrap();
    assert!(guided_builder
        .apply_mutation(GuidedMutation::AddSolarFavoredAbility(
            AbilityName::Linguistics
        ))
        .is_ok());
    assert!(guided_builder
        .apply_mutation(GuidedMutation::AddSolarFavoredAbility(AbilityName::Lore))
        .is_ok());
    assert!(guided_builder
        .apply_mutation(GuidedMutation::AddSolarFavoredAbility(AbilityName::Occult))
        .is_ok());
    assert!(guided_builder
        .apply_mutation(GuidedMutation::AddSolarFavoredAbility(
            AbilityName::Socialize
        ))
        .is_ok());
    assert!(guided_builder
        .apply_mutation(GuidedMutation::AddSolarFavoredAbility(AbilityName::Thrown))
        .is_ok());

    // Check cannot add a 6th favored ability
    assert!(guided_builder
        .check_mutation(&GuidedMutation::AddSolarFavoredAbility(
            AbilityName::Survival
        ))
        .is_err());

    // Check can remove a favored ability
    assert!(guided_builder
        .check_mutation(&GuidedMutation::RemoveSolarFavoredAbility(
            AbilityName::Lore
        ))
        .is_ok());
    assert!(guided_builder
        .apply_mutation(GuidedMutation::RemoveSolarFavoredAbility(AbilityName::Lore))
        .is_ok());

    // Check cannot remove a missing favored ability
    assert!(guided_builder
        .check_mutation(&GuidedMutation::RemoveSolarCasteAbility(AbilityName::Lore))
        .is_err());

    // Check cannot proceed without 5 favored abilities
    assert!(guided_builder
        .check_mutation(&GuidedMutation::SetStage(
            GuidedStage::ChooseMartialArtsStyles
        ))
        .is_err());

    // Finish building solar
    guided_builder
        .apply_mutation(GuidedMutation::AddSolarFavoredAbility(
            AbilityName::Survival,
        ))
        .unwrap();
    let solar = guided_builder
        .as_guided_view()
        .unwrap()
        .solar_traits()
        .unwrap();
    guided_builder
        .apply_mutation(GuidedMutation::CharacterMutation(
            CharacterMutation::SetSolar(solar),
        ))
        .unwrap();

    // After finalizing caste/supernal/favored, should be a valid Solar with
    // the correct abilities.
    let guided_view = guided_builder.as_guided_view().unwrap();
    let character_view = guided_view.as_character_view();
    assert!(character_view.is_solar());
    let solar_traits = character_view.solar_traits().unwrap();

    assert!(solar_traits.has_caste_ability(AbilityName::Athletics));
    assert!(solar_traits.has_caste_ability(AbilityName::Awareness));
    assert!(solar_traits.has_caste_ability(AbilityName::Dodge));
    assert!(solar_traits.has_caste_ability(AbilityName::Larceny));
    assert!(solar_traits.has_caste_ability(AbilityName::Stealth));
    assert_eq!(solar_traits.supernal_ability(), AbilityName::Dodge);
    assert!(solar_traits.has_favored_ability(AbilityName::Linguistics));
    assert!(solar_traits.has_favored_ability(AbilityName::Occult));
    assert!(solar_traits.has_favored_ability(AbilityName::Socialize));
    assert!(solar_traits.has_favored_ability(AbilityName::Survival));
    assert!(solar_traits.has_favored_ability(AbilityName::Thrown));

    // Move on to next stage
    let mutation = GuidedMutation::SetStage(GuidedStage::ChooseMartialArtsStyles);
    guided_builder.check_mutation(&mutation).unwrap();
    assert!(guided_builder.apply_mutation(mutation).is_ok());
}
