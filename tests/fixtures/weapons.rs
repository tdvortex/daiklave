use exalted_3e_gui::{
    character::CharacterBuilder,
    data_source::{BookReference, DataSource},
    weapons::{EquipHand, RangeBand, Weapon, WeaponTag, Weapons},
};

pub fn create_initial_weapons(builder: CharacterBuilder) -> CharacterBuilder {
    builder
        .with_weapon(
            Weapon::create_from_book("Core Rulebook".to_owned(), 581)
                .with_name("Knife".to_owned())
                .as_light()
                .as_one_handed()
                .as_melee()
                .with_thrown_range(RangeBand::Short)
                .dealing_lethal()
                .build()
                .unwrap(),
            None,
        )
        .unwrap()
        .with_weapon(
            Weapon::create_custom(None)
                .with_name("Screamer (Red Jade Reaper Daiklave)".to_owned())
                .as_artifact()
                .as_medium()
                .as_one_handed()
                .as_melee()
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
    for (key, maybe_hand, weapon_ref) in weapons.iter() {
        count += 1;
        match weapon_ref.name() {
            "Knife" => {
                assert!(maybe_hand.is_none());
                assert!(weapons.get(key).unwrap().id().is_some() == should_have_id);
                assert_eq!(
                    weapons.get(key).unwrap().tags(),
                    [
                        WeaponTag::Lethal,
                        WeaponTag::Melee,
                        WeaponTag::OneHanded,
                        WeaponTag::Thrown(RangeBand::Short),
                        WeaponTag::Light
                    ]
                    .into()
                );
                assert_eq!(
                    weapons.get(key).unwrap().data_source(),
                    &DataSource::Book(BookReference {
                        book_title: "Core Rulebook".to_owned(),
                        page_number: 581,
                    })
                );
            }
            "Screamer (Red Jade Reaper Daiklave)" => {
                assert!(maybe_hand == Some(EquipHand::Main));
                assert!(weapons.get(key).unwrap().id().is_some() == should_have_id);
                assert_eq!(
                    weapons.get(key).unwrap().tags(),
                    [
                        WeaponTag::Lethal,
                        WeaponTag::Melee,
                        WeaponTag::OneHanded,
                        WeaponTag::Medium,
                        WeaponTag::Balanced,
                        WeaponTag::Artifact,
                        WeaponTag::MartialArts("Single Point Shining Into Void Style".to_owned())
                    ]
                    .into()
                );
                if should_have_id {
                    assert!(match weapons.get(key).unwrap().data_source() {
                        DataSource::Book(_) => panic!("should be custom"),
                        DataSource::Custom(None) => panic!("should have custom creator id"),
                        DataSource::Custom(Some(_)) => true,
                    });
                } else {
                    assert_eq!(
                        weapons.get(key).unwrap().data_source(),
                        &DataSource::Custom(None)
                    );
                }
            }
            wrong => panic!("Unknown armor name: {}", wrong),
        }
    }
    assert_eq!(count, 2);
}

pub fn modify_weapons(weapons: &mut Weapons) {
    // Add weapon
    let unarmed_key = weapons.add_weapon(
        Weapon::create_from_book("Core Rulebook".to_owned(), 582)
            .with_name("Unarmed".to_owned())
            .as_brawl()
            .as_light()
            .as_one_handed()
            .dealing_bashing()
            .with_tag(WeaponTag::Grappling)
            .with_tag(WeaponTag::Natural)
            .build()
            .unwrap(),
    );

    // Remove weapon
    let knife_id = weapons
        .iter()
        .find(|(_, _, weapon)| weapon.name() == "Knife")
        .unwrap()
        .0;
    weapons.remove_weapon(knife_id);

    // Unequip weapon
    weapons.unequip(EquipHand::Main);

    // Equip weapon
    weapons.equip(unarmed_key, EquipHand::Both).unwrap();
}

pub fn validate_modified_weapons(weapons: &Weapons) {
    let mut count = 0;
    for (key, maybe_hand, weapon_ref) in weapons.iter() {
        count += 1;
        match weapon_ref.name() {
            "Unarmed" => {
                assert!(maybe_hand == Some(EquipHand::Both));
                assert_eq!(
                    weapons.get(key).unwrap().tags(),
                    [
                        WeaponTag::Bashing,
                        WeaponTag::Brawl,
                        WeaponTag::OneHanded,
                        WeaponTag::Light,
                        WeaponTag::Grappling,
                        WeaponTag::Natural,
                    ]
                    .into()
                );
                assert_eq!(
                    weapons.get(key).unwrap().data_source(),
                    &DataSource::Book(BookReference {
                        book_title: "Core Rulebook".to_owned(),
                        page_number: 582,
                    })
                );
            }
            "Screamer (Red Jade Reaper Daiklave)" => {
                assert!(maybe_hand == None);
                assert_eq!(
                    weapons.get(key).unwrap().tags(),
                    [
                        WeaponTag::Lethal,
                        WeaponTag::Melee,
                        WeaponTag::OneHanded,
                        WeaponTag::Medium,
                        WeaponTag::Balanced,
                        WeaponTag::Artifact,
                        WeaponTag::MartialArts("Single Point Shining Into Void Style".to_owned())
                    ]
                    .into()
                );
            }
            wrong => panic!("Unknown armor name: {}", wrong),
        }
    }
    assert_eq!(count, 2);
}
