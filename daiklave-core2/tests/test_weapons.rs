use daiklave_core2::{
    book_reference::{Book, BookReference},
    unique_id::UniqueId,
    weapons::{WeaponId, Equipped, BaseWeaponId, ArtifactWeaponId, WeaponWeightClass},
    CharacterEventSource, CharacterMutation,
};

#[test]
fn test_weapons_event_source() {
    let mut event_source = CharacterEventSource::default();
    let character_view = event_source.as_character_view().unwrap();
    // Default characters have the Unarmed weapon
    let unarmed = character_view.weapons().get(WeaponId::Unarmed).unwrap();
    assert_eq!(unarmed.id(), WeaponId::Unarmed);
    assert_eq!(unarmed.name(), "Unarmed");
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
    assert_eq!(unarmed.damage(AttackRange::Ranged(RangeBand::Close)), 7);
    assert_eq!(unarmed.parry_mod(), Some(0));
    assert_eq!(unarmed.overwhelming(), 1);

    // Natural weapons are always equipped
    assert_eq!(unarmed.is_equipped(), Some(Equipped::Natural))l
    assert!(character_view.unequip_weapon(WeaponId::Unarmed).is_err());
    assert!(character_view
        .equip_weapon(WeaponId::Unarmed, None)
        .is_err());

    // Cannot equip or unequip missing weapons
    assert!(character_view
        .unequip_weapon(WeaponId::Mundane(UniqueId::Placeholder(1)))
        .is_err());
    assert!(character_view
        .equip_weapon(WeaponId::Mundane(UniqueId::Placeholder(1)), None)
        .is_err());

    // Add some additional mundane weapons
    [
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(1)),
            MundaneWeapon::builder("Tiger Claws")
                .book_reference(BookReference::new(Book::CoreRulebook, 581))
                .weight_class(WeaponWeightClass::Light)
                .worn()
                .lethal()
                .brawl()
                .build()
                .as_memo(),
        ),
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(2)),
            MundaneWeapon::builder("Axe")
                .weight_class(WeaponWeightClass::Medium)
                .one_handed()
                .book_reference(BookReference::new(Book::CoreRulebook, 581))
                .lethal()
                .melee()
                .thrown_range(RangeBand::Short)
                .tag(OtherWeaponTag::Chopping)
                .build()
                .as_memo(),
        ),
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(3)),
            MundaneWeapon::builder("Shield")
                .weight_class(WeaponWeightClass::Medium)
                .one_handed()
                .bashing()
                .book_reference(BookReference::new(Book::CoreRulebook, 583))
                .melee()
                .tag(OtherWeaponTag::Shield)
                .build()
                .as_memo(),
        ),
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(4)),
            MundaneWeapon::builder("Hook Sword")
                .weight_class(WeaponWeightClass::Medium)
                .one_handed()
                .lethal()
                .martial_arts()
                .book_reference(BookReference::new(Book::CoreRulebook, 583))
                .tag(OtherWeaponTag::Disarming)
                .build()
                .as_memo(),
        ),
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(5)),
            MundaneWeapon::builder("Great Sword")
                .weight_class(WeaponWeightClass::Heavy)
                .two_handed()
                .lethal()
                .melee()
                .tag(OtherWeaponTag::Balanced)
                .book_reference(BookReference::new(Book::CoreRulebook, 584))
                .tag(OtherWeaponTag::Reaching)
                .build()
                .as_memo()
        ),
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(6)),
            MundaneWeapon::builder("Chakram")
                .weight_class(WeaponWeightClass::Light)
                .one_handed()
                .lethal()
                .thrown()
                .thrown_range(RangeBand::Medium)
                .tag(OtherWeaponTag::Cutting)
                .tag(OtherWeaponTag::Mounted)
                .book_reference(BookReference::new(Book::CoreRulebook, 587))
                .build()
                .as_memo()
        ),
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(7)),
            MundaneWeapon::builder("Crossbow")
                .weight_class(WeaponWeightClass::Light)
                .two_handed()
                .lethal()
                .archery()
                .archery_range(RangeBand::Long)
                .tag(OtherWeaponTag::Crossbow)
                .tag(OtherWeaponTag::Piercing)
                .tag(OtherWeaponTag::Powerful)
                .tag(OtherWeaponTag::Slow)
                .build()
                .as_memo()
        ),
    ].into_iter().fold(&mut event_source, |source, mutation| source.apply_mutation(mutation).unwrap());

    // Worn weapons can be equipped and unequipped without needing hands
    let mutation = CharacterMutation::EquipWeapon(WeaponId::Mundane(BaseWeaponId(UniqueId::Placeholder(1))), None);
    event_source.apply_mutation(mutation).unwrap();
    let mutation = CharacterMutation::UnequipWeapon(WeaponId::Mundane(UniqueId::Placeholder(1)), None);
    event_source.apply_mutation(mutation).unwrap();

    // Can wield one handed weapons as main only, two different, off hand only, or paired
    [
        CharacterMutation::EquipWeapon(WeaponId::Mundane(BaseWeaponId(UniqueId::Placeholder(4))), Some(EquipHand::OffHand)),
        CharacterMutation::EquipWeapon(WeaponId::Mundane(BaseWeaponId(UniqueId::Placeholder(4))), Some(EquipHand::MainHand)),
        CharacterMutation::EquipWeapon(WeaponId::Mundane(BaseWeaponId(UniqueId::Placeholder(3))), Some(EquipHand::OffHand)),
    ].into_iter().fold(&mut event_source, |source, mutation| source.apply_mutation(mutation).unwrap());

    // Equipping a two handed weapon unequips all one-handed weapons
    let mutation = CharacterMutation::EquipWeapon(WeaponId::Mundane(UniqueId::Placeholder(5)), None);
    event_source.apply_mutation(mutation).unwrap();

    // Create and add a unique artifact weapon
    let mutation = CharacterMutation::AddArtifact(
        Artifact::Weapon(
            ArtifactWeaponId(UniqueId::Placeholder(1)),
            ArtifactWeapon::builder("Volcano Cutter")
            .book_reference(BookReference::new(Book::CoreRulebook, 597))
            .merit_dots(5)
            .material(MagicMaterial::RedJade)
            .base_artifact(
                BaseWeaponId(UniqueId::Placeholder(8)),
                BaseArtifactWeapon::builder("Grand Daiklave")
                .weight_class(WeaponWeightClass::Heavy)
                .two_handed()
                .lethal()
                .melee()
                .tag(OtherWeaponTag::Balanced)
                .tag(OtherWeaponTag::Reaching)
                .build()
            )
            .hearthstone_slots(2)
            .lore("Long lore description".to_owned())
            .powers("A Solar who attunes to Volcano Cutter awakens Grand \
                Eruption at no experience cost. By paying an extra three motes \
                when attuning the grand daiklave, the Solar gains an \
                additional point of Initiative on any successful withering \
                attack made with Volcano Cutter which rolls no 1s.".to_owned())
            .book_reference(BookReference::new(Book::CoreRulebook, 627))
            .build()
        )
    );
}
