use daiklave_core2::{
    armor::armor_item::{ArmorItem, ArmorName, ArmorNameMutation, ArmorTag, ArmorWeightClass},
    artifact::{AddArtifact, ArtifactNameMutation, MagicMaterial},
    book_reference::{Book, BookReference},
    CharacterEventSource, CharacterMutation,
};

#[test]
fn test_armor() {
    let mut event_source = CharacterEventSource::default();
    let character = event_source.as_character().unwrap();

    // Characters have no armor by default
    assert!(character.armor().iter().next().is_none());
    assert!(character.armor().worn().is_none());

    // Add some armor
    let chain_shirt = ArmorItem::base_builder("Chain Shirt".to_owned())
        .book_reference(BookReference::new(Book::CoreRulebook, 592))
        .weight_class(ArmorWeightClass::Light)
        .tag(ArmorTag::Concealable)
        .build_mundane();

    let mutation = CharacterMutation::AddMundaneArmor(chain_shirt);
    let character = event_source.apply_mutation(mutation).unwrap();

    // Armor should initially be unequipped
    assert!(character.armor().worn().is_none());
    assert_eq!(
        character
            .armor()
            .get(ArmorName::Mundane("Chain Shirt"))
            .unwrap()
            .name(),
        ArmorName::Mundane("Chain Shirt")
    );
    assert_eq!(
        character.armor().iter().next().unwrap(),
        ArmorName::Mundane("Chain Shirt")
    );

    // Equip the armor
    let mutation =
        CharacterMutation::EquipArmor(ArmorNameMutation::Mundane("Chain Shirt".to_owned()));
    let character = event_source.apply_mutation(mutation).unwrap();

    // Check the properties of the armor
    assert_eq!(
        character.armor().iter().next().unwrap(),
        (ArmorName::Mundane("Chain Shirt"))
    );
    let chain_shirt = character.armor().worn().unwrap();
    assert_eq!(chain_shirt.name(), ArmorName::Mundane("Chain Shirt"));
    assert_eq!(
        chain_shirt.book_reference().unwrap(),
        BookReference::new(Book::CoreRulebook, 592)
    );
    assert_eq!(chain_shirt.soak_bonus(), 3);
    assert_eq!(chain_shirt.mobility_penalty(), 0);
    assert_eq!(chain_shirt.hardness(), 0);
    assert_eq!(chain_shirt.attunement_cost(), None);
    assert_eq!(
        chain_shirt.tags().collect::<Vec<ArmorTag>>(),
        vec![ArmorTag::Concealable]
    );
    assert_eq!(chain_shirt.hearthstone_slots(), 0);
    assert!(chain_shirt.is_equipped());

    // Unequip the armor
    let mutation = CharacterMutation::UnequipArmor;
    event_source.apply_mutation(mutation).unwrap();

    // Add some artifact armor
    let mutation = CharacterMutation::AddArtifact(AddArtifact::Armor(
        ArmorItem::artifact_builder("Brilliant Sentinel".to_owned())
            .base_artifact(
                ArmorItem::base_builder("Articulated Plate (Artifact)".to_owned())
                    .book_reference(BookReference::new(Book::CoreRulebook, 600))
                    .weight_class(ArmorWeightClass::Heavy)
                    .build_artifact(),
            )
            .material(MagicMaterial::Orichalcum)
            .merit_dots(3)
            .hearthstone_slots(2)
            .lore("There was once an enlightened city[...]")
            .powers(
                "When attuned and worn, the armor imposes a -1 penalty \
            to the Join Battle rolls of all enemies who have harmed \
            or wish to harm the object of one of the wearer's Major \
            or Defining Intimacies. This penalty becomes -1 success \
            to all unExalted creatures of darkness.",
            )
            .book_reference(BookReference::new(Book::CoreRulebook, 616))
            .build(),
    ));
    event_source.apply_mutation(mutation).unwrap();

    // Equip the artifact armor
    let mutation =
        CharacterMutation::EquipArmor(ArmorNameMutation::Artifact("Brilliant Sentinel".to_owned()));
    let character = event_source.apply_mutation(mutation).unwrap();

    assert_eq!(
        character.armor().worn().unwrap().name(),
        ArmorName::Artifact("Brilliant Sentinel")
    );

    // Equipping another piece of armor should swap the two
    let mutation =
        CharacterMutation::EquipArmor(ArmorNameMutation::Mundane("Chain Shirt".to_owned()));
    let character = event_source.apply_mutation(mutation).unwrap();
    assert_eq!(
        character.armor().worn().unwrap().name(),
        ArmorName::Mundane("Chain Shirt")
    );

    // Remove the artifact armor
    let mutation = CharacterMutation::RemoveArtifact(ArtifactNameMutation::Armor(
        "Brilliant Sentinel".to_owned(),
    ));
    event_source.apply_mutation(mutation).unwrap();

    // Check you can't remove equipped armor
    let mutation = CharacterMutation::RemoveMundaneArmor("Chain Shirt".to_owned());
    assert!(event_source.apply_mutation(mutation).is_err());

    // Check we can remove it if we unequip first
    let mutation = CharacterMutation::UnequipArmor;
    event_source.apply_mutation(mutation).unwrap();

    let mutation = CharacterMutation::RemoveMundaneArmor("Chain Shirt".to_owned());
    let character = event_source.apply_mutation(mutation).unwrap();

    assert!(character.armor().iter().next().is_none())
}
