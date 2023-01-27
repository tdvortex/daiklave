use daiklave_core2::{
    artifact::{Artifact, ArtifactName, MagicMaterial},
    attributes::AttributeName,
    book_reference::{Book, BookReference},
    weapons::weapon::{
        AttackRange, EquipHand, Equipped, OptionalWeaponTag, RangeBand, Weapon, WeaponName,
        WeaponNameMutation, WeaponTag, WeaponWeightClass,
    },
    CharacterEventSource, CharacterMutation,
};

#[test]
fn test_weapons_event_source() {
    let mut event_source = CharacterEventSource::default();
    let mut character_view = event_source.as_character().unwrap();
    // Default characters have the Unarmed weapon
    let unarmed = character_view
        .weapons()
        .get(WeaponName::Unarmed, Some(Equipped::Natural))
        .unwrap();
    assert_eq!(unarmed.name(), WeaponName::Unarmed);
    assert_eq!(
        unarmed.book_reference(),
        Some(BookReference::new(Book::CoreRulebook, 582))
    );
    assert_eq!(unarmed.weight_class(), WeaponWeightClass::Light);
    assert_eq!(
        unarmed.tags().collect::<Vec<WeaponTag>>(),
        vec![
            WeaponTag::Bashing,
            WeaponTag::Brawl,
            WeaponTag::Grappling,
            WeaponTag::Natural,
        ]
    );
    assert_eq!(unarmed.accuracy(AttackRange::Melee), Some(4));
    assert_eq!(
        unarmed.accuracy(AttackRange::Ranged(RangeBand::Close)),
        None
    );
    assert_eq!(unarmed.damage(AttackRange::Melee), Some(7));
    assert_eq!(unarmed.damage(AttackRange::Ranged(RangeBand::Short)), None);
    assert_eq!(unarmed.parry_mod(), Some(0));
    assert_eq!(unarmed.overwhelming(), 1);

    // Natural weapons are always equipped
    assert_eq!(unarmed.is_equipped(), Some(Equipped::Natural));
    assert!(character_view
        .unequip_weapon(WeaponName::Unarmed, Equipped::Natural)
        .is_err());
    assert!(character_view
        .equip_weapon(WeaponName::Unarmed, None)
        .is_err());

    // Cannot equip or unequip missing weapons
    assert!(character_view
        .unequip_weapon(WeaponName::Mundane("Missing weapon"), Equipped::MainHand)
        .is_err());
    assert!(character_view
        .equip_weapon(WeaponName::Mundane("Missing weapon"), None)
        .is_err());

    // Add some additional mundane weapons
    [
        CharacterMutation::AddMundaneWeapon(
            Weapon::base("Tiger Claws".to_owned())
                .book_reference(BookReference::new(Book::CoreRulebook, 581))
                .weight_class(WeaponWeightClass::Light)
                .worn()
                .lethal()
                .brawl()
                .build_mundane(),
        ),
        CharacterMutation::AddMundaneWeapon(
            Weapon::base("Axe".to_owned())
                .weight_class(WeaponWeightClass::Medium)
                .one_handed()
                .book_reference(BookReference::new(Book::CoreRulebook, 581))
                .lethal()
                .melee()
                .thrown_range(RangeBand::Short)
                .tag(OptionalWeaponTag::Chopping)
                .build_mundane(),
        ),
        CharacterMutation::AddMundaneWeapon(
            Weapon::base("Shield".to_owned())
                .weight_class(WeaponWeightClass::Medium)
                .one_handed()
                .bashing()
                .book_reference(BookReference::new(Book::CoreRulebook, 583))
                .melee()
                .tag(OptionalWeaponTag::Shield)
                .build_mundane(),
        ),
        CharacterMutation::AddMundaneWeapon(
            Weapon::base("Hook Sword".to_owned())
                .weight_class(WeaponWeightClass::Medium)
                .one_handed()
                .lethal()
                .martial_arts()
                .book_reference(BookReference::new(Book::CoreRulebook, 583))
                .tag(OptionalWeaponTag::Disarming)
                .build_mundane(),
        ),
        CharacterMutation::AddMundaneWeapon(
            Weapon::base("Hook Sword".to_owned())
                .weight_class(WeaponWeightClass::Medium)
                .one_handed()
                .lethal()
                .martial_arts()
                .book_reference(BookReference::new(Book::CoreRulebook, 583))
                .tag(OptionalWeaponTag::Disarming)
                .build_mundane(),
        ),
        CharacterMutation::AddMundaneWeapon(
            Weapon::base("Great Sword".to_owned())
                .weight_class(WeaponWeightClass::Heavy)
                .two_handed()
                .lethal()
                .melee()
                .tag(OptionalWeaponTag::Balanced)
                .book_reference(BookReference::new(Book::CoreRulebook, 584))
                .tag(OptionalWeaponTag::Reaching)
                .build_mundane(),
        ),
        CharacterMutation::AddMundaneWeapon(
            Weapon::base("Chakram".to_owned())
                .weight_class(WeaponWeightClass::Light)
                .one_handed()
                .lethal()
                .thrown()
                .thrown_range(RangeBand::Medium)
                .tag(OptionalWeaponTag::Cutting)
                .tag(OptionalWeaponTag::Mounted)
                .book_reference(BookReference::new(Book::CoreRulebook, 587))
                .build_mundane(),
        ),
        CharacterMutation::AddMundaneWeapon(
            Weapon::base("Crossbow".to_owned())
                .weight_class(WeaponWeightClass::Light)
                .two_handed()
                .lethal()
                .archery()
                .archery_range(RangeBand::Long)
                .tag(OptionalWeaponTag::Crossbow)
                .tag(OptionalWeaponTag::Piercing)
                .tag(OptionalWeaponTag::Powerful)
                .tag(OptionalWeaponTag::Slow)
                .build_mundane(),
        ),
    ]
    .into_iter()
    .for_each(|mutation| {
        event_source.apply_mutation(mutation).unwrap();
    });

    // Can have multiple copies of an unequipped mundane weapon
    let character_view = event_source.as_character().unwrap();
    assert_eq!(
        character_view
            .weapons()
            .get(WeaponName::Mundane("Hook Sword"), None)
            .unwrap()
            .quantity(),
        2
    );

    // Worn weapons can be equipped and unequipped without needing hands
    let mutation =
        CharacterMutation::EquipWeapon(WeaponNameMutation::Mundane("Tiger Claws".to_owned()), None);
    event_source.apply_mutation(mutation).unwrap();
    let mutation = CharacterMutation::UnequipWeapon(
        WeaponNameMutation::Mundane("Tiger Claws".to_owned()),
        Equipped::Worn,
    );
    event_source.apply_mutation(mutation).unwrap();

    // Can wield one handed weapons as main only, two different, off hand only, or paired
    [
        CharacterMutation::EquipWeapon(
            WeaponNameMutation::Mundane("Hook Sword".to_owned()),
            Some(EquipHand::OffHand),
        ),
        CharacterMutation::EquipWeapon(
            WeaponNameMutation::Mundane("Hook Sword".to_owned()),
            Some(EquipHand::MainHand),
        ),
        CharacterMutation::EquipWeapon(
            WeaponNameMutation::Mundane("Shield".to_owned()),
            Some(EquipHand::OffHand),
        ),
    ]
    .into_iter()
    .for_each(|mutation| {
        event_source.apply_mutation(mutation).unwrap();
    });

    // An equipped weapon always shows up as a quantity of 1
    let character_view = event_source.as_character().unwrap();
    assert_eq!(
        character_view
            .weapons()
            .get(WeaponName::Mundane("Hook Sword"), Some(Equipped::MainHand))
            .unwrap()
            .quantity(),
        1
    );
    assert_eq!(
        character_view
            .weapons()
            .get(WeaponName::Mundane("Shield"), Some(Equipped::OffHand))
            .unwrap()
            .quantity(),
        1
    );

    // Can't equip a two-handed melee weapon if Strength is less than 3
    let mutation =
        CharacterMutation::EquipWeapon(WeaponNameMutation::Mundane("Great Sword".to_owned()), None);
    assert!(event_source.apply_mutation(mutation).is_err());

    let mutation = CharacterMutation::SetAttribute(AttributeName::Strength, 3);
    event_source.apply_mutation(mutation).unwrap();

    // Equipping a two handed weapon unequips all one-handed weapons
    let mutation =
        CharacterMutation::EquipWeapon(WeaponNameMutation::Mundane("Great Sword".to_owned()), None);
    event_source.apply_mutation(mutation).unwrap();

    // Create and add a unique artifact weapon
    let mutation = CharacterMutation::AddArtifact(Artifact::Weapon(
        Weapon::artifact("Volcano Cutter".to_owned())
            .base_artifact(
                Weapon::base("Grand Daiklave".to_owned())
                    .book_reference(BookReference::new(Book::CoreRulebook, 597))
                    .weight_class(WeaponWeightClass::Heavy)
                    .two_handed()
                    .lethal()
                    .melee()
                    .tag(OptionalWeaponTag::Balanced)
                    .tag(OptionalWeaponTag::Reaching)
                    .build_artifact(),
            )
            .material(MagicMaterial::RedJade)
            .merit_dots(5)
            .hearthstone_slots(2)
            .lore("Long lore description".to_owned())
            .powers(
                "A Solar who attunes to Volcano Cutter awakens Grand \
                Eruption at no experience cost. By paying an extra three motes \
                when attuning the grand daiklave, the Solar gains an \
                additional point of Initiative on any successful withering \
                attack made with Volcano Cutter which rolls no 1s."
                    .to_owned(),
            )
            .book_reference(BookReference::new(Book::CoreRulebook, 627))
            .build(),
    ));
    event_source.apply_mutation(mutation).unwrap();

    // Check that lowering strength below 3 causes a heavy melee two-handed
    // weapon to unequip
    let character = event_source
        .apply_mutation(CharacterMutation::SetAttribute(AttributeName::Strength, 2))
        .unwrap();
    assert!(character
        .weapons()
        .get(WeaponName::Mundane("Great Sword"), None)
        .is_some());
    event_source.undo().unwrap();

    // Check you can remove an unequipped mundane weapon
    let mutation = CharacterMutation::RemoveMundaneWeapon("Crossbow".to_owned());
    event_source.apply_mutation(mutation).unwrap();

    // Check you cannot remove a missing mundane weapon
    let mutation = CharacterMutation::RemoveMundaneWeapon("Crossbow".to_owned());
    assert!(event_source.apply_mutation(mutation).is_err());

    // Check you cannot remove an equipped mundane weapon without unequipped copies
    let mutation = CharacterMutation::RemoveMundaneWeapon("Great Sword".to_owned());
    assert!(event_source.apply_mutation(mutation).is_err());

    // Check you can remove an unequipped artifact weapon
    let mutation =
        CharacterMutation::RemoveArtifact(ArtifactName::Weapon("Volcano Cutter".to_owned()));
    event_source.apply_mutation(mutation).unwrap();
}
