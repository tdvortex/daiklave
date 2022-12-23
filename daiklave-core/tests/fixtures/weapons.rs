use daiklave_core::{
    character::CharacterBuilder,
    data_source::{BookReference, DataSource},
    id::{CharacterId, Id},
    weapons::{EquipHand, RangeBand, Weapon, WeaponTag, Weapons},
};

pub fn create_initial_weapons(builder: CharacterBuilder) -> CharacterBuilder {
    let character_placeholder_id = builder.id();
    builder
        .with_weapon(
            Weapon::from_book(Id::Placeholder(0), "Core Rulebook".to_owned(), 581)
                .with_name("Knife".to_owned())
                .into_light()
                .into_one_handed()
                .into_melee()
                .with_thrown_range(RangeBand::Short)
                .dealing_lethal()
                .build()
                .unwrap(),
            None,
        )
        .unwrap()
        .with_weapon(
            Weapon::custom(Id::Placeholder(1), character_placeholder_id)
                .with_name("Screamer (Red Jade Reaper Daiklave)".to_owned())
                .into_artifact()
                .into_medium()
                .into_one_handed()
                .into_melee()
                .with_tag(WeaponTag::Balanced)
                .with_martial_arts("Single Point Shining Into Void Style".to_owned())
                .dealing_lethal()
                .build()
                .unwrap(),
            Some(EquipHand::Main),
        )
        .unwrap()
}

pub fn validate_initial_weapons(weapons: &Weapons, should_have_id: bool) {
    let mut count = 0;
    for (key, weapon_ref, maybe_hand) in weapons.iter() {
        count += 1;
        match weapon_ref.name() {
            "Knife" => {
                assert!(maybe_hand.is_none());
                assert!(
                    weapons.get_by_index(key).unwrap().1.id().is_placeholder() != should_have_id
                );
                assert_eq!(
                    weapons.get_by_index(key).unwrap().1.tags(),
                    vec![
                        WeaponTag::Lethal,
                        WeaponTag::Light,
                        WeaponTag::Melee,
                        WeaponTag::OneHanded,
                        WeaponTag::Thrown(RangeBand::Short),
                    ]
                );
                assert_eq!(
                    weapons.get_by_index(key).unwrap().1.data_source(),
                    &DataSource::Book(BookReference {
                        book_title: "Core Rulebook".to_owned(),
                        page_number: 581,
                    })
                );
            }
            "Screamer (Red Jade Reaper Daiklave)" => {
                assert!(maybe_hand == Some(EquipHand::Main));
                assert!(
                    weapons.get_by_index(key).unwrap().1.id().is_placeholder() != should_have_id
                );
                assert_eq!(
                    weapons.get_by_index(key).unwrap().1.tags(),
                    vec![
                        WeaponTag::Artifact,
                        WeaponTag::Balanced,
                        WeaponTag::Lethal,
                        WeaponTag::MartialArts("Single Point Shining Into Void Style".to_owned()),
                        WeaponTag::Medium,
                        WeaponTag::Melee,
                        WeaponTag::OneHanded,
                    ]
                );
                if should_have_id {
                    assert!(match weapons.get_by_index(key).unwrap().1.data_source() {
                        DataSource::Book(_) => panic!("should be custom"),
                        DataSource::Custom(CharacterId(Id::Placeholder(_))) =>
                            panic!("should have creator id in database"),
                        DataSource::Custom(CharacterId(Id::Database(_))) => true,
                    });
                } else {
                    assert!(weapons
                        .get_by_index(key)
                        .unwrap()
                        .1
                        .data_source()
                        .is_custom());
                }
            }
            wrong => panic!("Unknown armor name: {}", wrong),
        }
    }
    assert_eq!(count, 2);
}

pub fn modify_weapons(weapons: &mut Weapons) {
    // Remove weapon
    let knife_id = weapons
        .iter()
        .find(|(_, weapon, _)| weapon.name() == "Knife")
        .unwrap()
        .0;
    weapons.remove_weapon(knife_id).unwrap();

    // Add weapon
    let unarmed_key = weapons.add_weapon(
        Weapon::from_book(Id::Placeholder(2), "Core Rulebook".to_owned(), 582)
            .with_name("Unarmed".to_owned())
            .into_brawl()
            .into_light()
            .into_one_handed()
            .dealing_bashing()
            .with_tag(WeaponTag::Grappling)
            .with_tag(WeaponTag::Natural)
            .build()
            .unwrap(),
    );

    // Unequip weapon
    weapons.unequip(EquipHand::Main);

    // Equip weapon
    weapons.equip(unarmed_key, EquipHand::Both).unwrap();
}

pub fn validate_modified_weapons(weapons: &Weapons) {
    let mut count = 0;
    for (key, weapon_ref, maybe_hand) in weapons.iter() {
        count += 1;
        match weapon_ref.name() {
            "Unarmed" => {
                assert!(maybe_hand == Some(EquipHand::Both));
                assert_eq!(
                    weapons.get_by_index(key).unwrap().1.tags(),
                    vec![
                        WeaponTag::Bashing,
                        WeaponTag::Brawl,
                        WeaponTag::Grappling,
                        WeaponTag::Light,
                        WeaponTag::Natural,
                        WeaponTag::OneHanded,
                    ]
                );
                assert_eq!(
                    weapons.get_by_index(key).unwrap().1.data_source(),
                    &DataSource::Book(BookReference {
                        book_title: "Core Rulebook".to_owned(),
                        page_number: 582,
                    })
                );
            }
            "Screamer (Red Jade Reaper Daiklave)" => {
                assert!(maybe_hand == None);
                assert_eq!(
                    weapons.get_by_index(key).unwrap().1.tags(),
                    vec![
                        WeaponTag::Artifact,
                        WeaponTag::Balanced,
                        WeaponTag::Lethal,
                        WeaponTag::MartialArts("Single Point Shining Into Void Style".to_owned()),
                        WeaponTag::Medium,
                        WeaponTag::Melee,
                        WeaponTag::OneHanded,
                    ]
                );
            }
            wrong => panic!("Unknown armor name: {}", wrong),
        }
    }
    assert_eq!(count, 2);
}
