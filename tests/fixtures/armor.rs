use exalted_3e_gui::{
    armor::{Armor, ArmorItem, ArmorTag},
    character::CharacterBuilder,
    data_source::{BookReference, DataSource},
};

pub fn create_initial_armor(builder: CharacterBuilder) -> CharacterBuilder {
    builder
        .with_armor(
            ArmorItem::from_book("Core Rulebook".to_owned(), 600)
                .with_name("Silken Armor".to_owned())
                .as_light()
                .as_artifact()
                .with_tag(ArmorTag::Silent)
                .with_tag(ArmorTag::Special)
                .build()
                .unwrap(),
            false,
        )
        .with_armor(
            ArmorItem::custom(None)
                .with_name("Straw Hat".to_owned())
                .as_light()
                .build()
                .unwrap(),
            true,
        )
}

pub fn validate_initial_armor_items(armor: &Armor, should_have_id: bool) {
    let mut count = 0;

    for (key, item, worn) in armor.iter() {
        count += 1;
        match item.name() {
            "Straw Hat" => {
                assert!(worn);
                assert!(armor.get_by_index(key).unwrap().1.id().is_some() == should_have_id);
                assert_eq!(
                    armor.get_by_index(key).unwrap().1.tags(),
                    [ArmorTag::Light].into()
                );
                if should_have_id {
                    assert!(match armor.get_by_index(key).unwrap().1.data_source() {
                        DataSource::Book(_) => panic!("should be custom"),
                        DataSource::Custom(None) => panic!("should have custom creator id"),
                        DataSource::Custom(Some(_)) => true,
                    });
                } else {
                    assert_eq!(
                        armor.get_by_index(key).unwrap().1.data_source(),
                        &DataSource::Custom(None)
                    );
                }
            }
            "Silken Armor" => {
                assert!(!worn);
                assert!(armor.get_by_index(key).unwrap().1.id().is_some() == should_have_id);
                assert_eq!(
                    armor.get_by_index(key).unwrap().1.tags(),
                    [
                        ArmorTag::Light,
                        ArmorTag::Artifact,
                        ArmorTag::Silent,
                        ArmorTag::Special
                    ]
                    .into()
                );
                assert_eq!(
                    armor.get_by_index(key).unwrap().1.data_source(),
                    &DataSource::Book(BookReference {
                        book_title: "Core Rulebook".to_owned(),
                        page_number: 600
                    })
                );
            }
            wrong => panic!("Unknown armor name: {}", wrong),
        }
    }
    assert!(count == 2);
}

pub fn modify_armor(armor: &mut Armor) {
    // Unequip an item
    armor.unequip_armor_item();

    // Remove an item
    let straw_hat_key = armor
        .iter()
        .find(|(_, item, _)| item.name() == "Straw Hat")
        .unwrap()
        .0;
    armor.remove_armor_item(straw_hat_key).unwrap();

    // Equip an item
    let silken_armor_key = armor
        .iter()
        .find(|(_, item, _)| item.name() == "Silken Armor")
        .unwrap()
        .0;
    armor.equip_armor_item(silken_armor_key).unwrap();

    // Add an item
    armor.add_armor_item(
        ArmorItem::custom(None)
            .as_medium()
            .with_name("Stolen Guard's Breastplate".to_owned())
            .build()
            .unwrap(),
    );
}

pub fn validate_modified_armor_items(armor: &Armor) {
    let mut count = 0;
    for (key, item, worn) in armor.iter() {
        count += 1;
        match item.name() {
            "Stolen Guard's Breastplate" => {
                assert!(!worn);
                assert_eq!(
                    armor.get_by_index(key).unwrap().1.tags(),
                    [ArmorTag::Medium].into()
                );
            }
            "Silken Armor" => {
                assert!(worn);
                assert_eq!(
                    armor.get_by_index(key).unwrap().1.tags(),
                    [
                        ArmorTag::Light,
                        ArmorTag::Artifact,
                        ArmorTag::Silent,
                        ArmorTag::Special
                    ]
                    .into()
                );
                assert_eq!(
                    armor.get_by_index(key).unwrap().1.data_source(),
                    &DataSource::Book(BookReference {
                        book_title: "Core Rulebook".to_owned(),
                        page_number: 600
                    })
                );
            }
            wrong => panic!("Unknown armor name: {}", wrong),
        }
    }
    assert!(count == 2);
}
