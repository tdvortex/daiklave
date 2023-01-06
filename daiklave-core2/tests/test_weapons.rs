use daiklave_core2::{
    book_reference::{Book, BookReference},
    unique_id::UniqueId,
    weapons::{WeaponId, Equipped, BaseWeaponId, ArtifactWeaponId, WeaponWeightClass, WeaponTag, AttackRange, RangeBand, EquipHand, Weapon, OptionalWeaponTag, ArtifactId},
    CharacterEventSource, CharacterMutation, artifact::{MagicMaterial, ArtifactMemo},
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
    assert_eq!(unarmed.damage(AttackRange::Ranged(RangeBand::Close)), Some(7));
    assert_eq!(unarmed.damage(AttackRange::Ranged(RangeBand::Short)), None);
    assert_eq!(unarmed.parry_mod(), Some(0));
    assert_eq!(unarmed.overwhelming(), 1);

    // Natural weapons are always equipped
    assert_eq!(unarmed.is_equipped(), Some(Equipped::Natural));
    assert!(character_view.unequip_weapon(WeaponId::Unarmed, Some(EquipHand::MainHand)).is_err());
    assert!(character_view
        .equip_weapon(WeaponId::Unarmed, None)
        .is_err());

    // Cannot equip or unequip missing weapons
    assert!(character_view
        .unequip_weapon(WeaponId::Mundane(BaseWeaponId(UniqueId::Placeholder(1))), Some(EquipHand::MainHand))
        .is_err());
    assert!(character_view
        .equip_weapon(WeaponId::Mundane(BaseWeaponId(UniqueId::Placeholder(1))), None)
        .is_err());

    // Add some additional mundane weapons
    [
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(1)),
            Weapon::base("Tiger Claws")
                .book_reference(BookReference::new(Book::CoreRulebook, 581))
                .weight_class(WeaponWeightClass::Light)
                .worn()
                .lethal()
                .brawl()
                .build_mundane()
        ),
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(2)),
            Weapon::base("Axe")
                .weight_class(WeaponWeightClass::Medium)
                .one_handed()
                .book_reference(BookReference::new(Book::CoreRulebook, 581))
                .lethal()
                .melee()
                .thrown_range(RangeBand::Short)
                .tag(OptionalWeaponTag::Chopping)
                .build_mundane()
        ),
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(3)),
            Weapon::base("Shield")
                .weight_class(WeaponWeightClass::Medium)
                .one_handed()
                .bashing()
                .book_reference(BookReference::new(Book::CoreRulebook, 583))
                .melee()
                .tag(OptionalWeaponTag::Shield)
                .build_mundane()
        ),
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(4)),
            Weapon::base("Hook Sword")
                .weight_class(WeaponWeightClass::Medium)
                .one_handed()
                .lethal()
                .martial_arts()
                .book_reference(BookReference::new(Book::CoreRulebook, 583))
                .tag(OptionalWeaponTag::Disarming)
                .build_mundane()
        ),
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(5)),
            Weapon::base("Great Sword")
                .weight_class(WeaponWeightClass::Heavy)
                .two_handed()
                .lethal()
                .melee()
                .tag(OptionalWeaponTag::Balanced)
                .book_reference(BookReference::new(Book::CoreRulebook, 584))
                .tag(OptionalWeaponTag::Reaching)
                .build_mundane()
        ),
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(6)),
            Weapon::base("Chakram")
                .weight_class(WeaponWeightClass::Light)
                .one_handed()
                .lethal()
                .thrown()
                .thrown_range(RangeBand::Medium)
                .tag(OptionalWeaponTag::Cutting)
                .tag(OptionalWeaponTag::Mounted)
                .book_reference(BookReference::new(Book::CoreRulebook, 587))
                .build_mundane()
        ),
        CharacterMutation::AddMundaneWeapon(
            BaseWeaponId(UniqueId::Placeholder(7)),
            Weapon::base("Crossbow")
                .weight_class(WeaponWeightClass::Light)
                .two_handed()
                .lethal()
                .archery()
                .archery_range(RangeBand::Long)
                .tag(OptionalWeaponTag::Crossbow)
                .tag(OptionalWeaponTag::Piercing)
                .tag(OptionalWeaponTag::Powerful)
                .tag(OptionalWeaponTag::Slow)
                .build_mundane()
        ),
    ].into_iter().fold(&mut event_source, |source, mutation| source.apply_mutation(mutation).unwrap());

    // Worn weapons can be equipped and unequipped without needing hands
    let mutation = CharacterMutation::EquipWeapon(WeaponId::Mundane(BaseWeaponId(UniqueId::Placeholder(1))), None);
    event_source.apply_mutation(mutation).unwrap();
    let mutation = CharacterMutation::UnequipWeapon(WeaponId::Mundane(BaseWeaponId(UniqueId::Placeholder(1))), None);
    event_source.apply_mutation(mutation).unwrap();

    // Can wield one handed weapons as main only, two different, off hand only, or paired
    [
        CharacterMutation::EquipWeapon(WeaponId::Mundane(BaseWeaponId(UniqueId::Placeholder(4))), Some(EquipHand::OffHand)),
        CharacterMutation::EquipWeapon(WeaponId::Mundane(BaseWeaponId(UniqueId::Placeholder(4))), Some(EquipHand::MainHand)),
        CharacterMutation::EquipWeapon(WeaponId::Mundane(BaseWeaponId(UniqueId::Placeholder(3))), Some(EquipHand::OffHand)),
    ].into_iter().fold(&mut event_source, |source, mutation| source.apply_mutation(mutation).unwrap());

    // Equipping a two handed weapon unequips all one-handed weapons
    let mutation = CharacterMutation::EquipWeapon(WeaponId::Mundane(BaseWeaponId(UniqueId::Placeholder(5))), None);
    event_source.apply_mutation(mutation).unwrap();

    // Create and add a unique artifact weapon
    let mutation = CharacterMutation::AddArtifact(
        ArtifactId::Weapon(ArtifactWeaponId(UniqueId::Placeholder(1))),
        ArtifactMemo::Weapon(
            Weapon::artifact("Volcano Cutter")
            .base_artifact(
                BaseWeaponId(UniqueId::Placeholder(8)),
                Weapon::base("Grand Daiklave")
                .book_reference(BookReference::new(Book::CoreRulebook, 597))
                .weight_class(WeaponWeightClass::Heavy)
                .two_handed()
                .lethal()
                .melee()
                .tag(OptionalWeaponTag::Balanced)
                .tag(OptionalWeaponTag::Reaching)
                .build_artifact()
            )
            .material(MagicMaterial::RedJade)
            .merit_dots(5)
            .hearthstone_slots(2)
            .lore("Long lore description")
            .powers("A Solar who attunes to Volcano Cutter awakens Grand \
                Eruption at no experience cost. By paying an extra three motes \
                when attuning the grand daiklave, the Solar gains an \
                additional point of Initiative on any successful withering \
                attack made with Volcano Cutter which rolls no 1s.")
            .book_reference(BookReference::new(Book::CoreRulebook, 627))
            .build()
        )
    );
    event_source.apply_mutation(mutation).unwrap();
}
