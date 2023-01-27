use daiklave_core2::{
    armor::armor_item::{
        artifact::ArtifactArmorId, ArmorId, ArmorItem, ArmorTag, ArmorWeightClass, BaseArmorId,
    },
    artifact::{Artifact, ArtifactName, MagicMaterial},
    book_reference::{Book, BookReference},
    unique_id::UniqueId,
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
    let chain_shirt = ArmorItem::base("Chain Shirt (Mundane)")
        .book_reference(BookReference::new(Book::CoreRulebook, 592))
        .weight_class(ArmorWeightClass::Light)
        .tag(ArmorTag::Concealable)
        .build_mundane();

    let mutation =
        CharacterMutation::AddMundaneArmor(BaseArmorId(UniqueId::Placeholder(1)), chain_shirt);
    let character = event_source.apply_mutation(mutation).unwrap();

    // Armor should initially be unequipped
    assert!(character.armor().worn().is_none());
    assert_eq!(
        character
            .armor()
            .get(ArmorId::Mundane(BaseArmorId(UniqueId::Placeholder(1))))
            .unwrap()
            .id(),
        ArmorId::Mundane(BaseArmorId(UniqueId::Placeholder(1)))
    );
    assert_eq!(
        character.armor().iter().next().unwrap(),
        ArmorId::Mundane(BaseArmorId(UniqueId::Placeholder(1)))
    );

    // Equip the armor
    let mutation =
        CharacterMutation::EquipArmor(ArmorId::Mundane(BaseArmorId(UniqueId::Placeholder(1))));
    let character = event_source.apply_mutation(mutation).unwrap();

    // Check the properties of the armor
    assert_eq!(
        character.armor().iter().next().unwrap(),
        (ArmorId::Mundane(BaseArmorId(UniqueId::Placeholder(1))))
    );
    let chain_shirt = character.armor().worn().unwrap();
    assert_eq!(
        chain_shirt.id(),
        ArmorId::Mundane(BaseArmorId(UniqueId::Placeholder(1)))
    );
    assert_eq!(chain_shirt.name(), "Chain Shirt (Mundane)");
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
    let mutation = CharacterMutation::AddArtifact(Artifact::Armor(
        ArtifactArmorId(UniqueId::Placeholder(1)),
        ArmorItem::artifact("Brilliant Sentinel")
            .base_artifact(
                BaseArmorId(UniqueId::Placeholder(2)),
                ArmorItem::base("Articulated Plate (Artifact)")
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
        CharacterMutation::EquipArmor(ArmorId::Artifact(ArtifactArmorId(UniqueId::Placeholder(1))));
    let character = event_source.apply_mutation(mutation).unwrap();

    assert_eq!(
        character.armor().worn().unwrap().id(),
        ArmorId::Artifact(ArtifactArmorId(UniqueId::Placeholder(1)))
    );

    // Equipping another piece of armor should swap the two
    let mutation =
        CharacterMutation::EquipArmor(ArmorId::Mundane(BaseArmorId(UniqueId::Placeholder(1))));
    let character = event_source.apply_mutation(mutation).unwrap();
    assert_eq!(
        character.armor().worn().unwrap().id(),
        ArmorId::Mundane(BaseArmorId(UniqueId::Placeholder(1)))
    );

    // Remove the artifact armor
    let mutation = CharacterMutation::RemoveArtifact(ArtifactName::Armor(ArtifactArmorId(
        UniqueId::Placeholder(1),
    )));
    event_source.apply_mutation(mutation).unwrap();

    // Check you can't remove equipped armor
    let mutation = CharacterMutation::RemoveMundaneArmor(BaseArmorId(UniqueId::Placeholder(1)));
    assert!(event_source.apply_mutation(mutation).is_err());

    // Check we can remove it if we unequip first
    let mutation = CharacterMutation::UnequipArmor;
    event_source.apply_mutation(mutation).unwrap();

    let mutation = CharacterMutation::RemoveMundaneArmor(BaseArmorId(UniqueId::Placeholder(1)));
    let character = event_source.apply_mutation(mutation).unwrap();

    assert!(character.armor().iter().next().is_none())
}
