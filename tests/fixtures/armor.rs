use exalted_3e_gui::{
    armor::{Armor, ArmorItem, ArmorTag},
    character::CharacterBuilder,
    data_source::{BookReference, DataSource},
};

pub fn create_initial_armor(builder: CharacterBuilder) -> CharacterBuilder {
    builder
        .with_armor(
            ArmorItem::create_from_book("Core Rulebook".to_owned(), 600)
                .with_name("Silken Armor".to_owned())
                .as_light()
                .as_artifact()
                .with_tag(ArmorTag::Silent)
                .with_tag(ArmorTag::Special)
                .build()
                .unwrap(),
            false,
        )
        .unwrap()
        .with_armor(
            ArmorItem::create_custom(None)
                .with_name("Straw Hat".to_owned())
                .as_light()
                .build()
                .unwrap(),
            true,
        )
        .unwrap()
}

pub fn validate_initial_armor_items(armor: &Armor, should_have_id: bool) {
    for (key, worn, item) in armor.iter() {
        match item.name() {
            "Straw Hat" => {
                assert!(worn);
                assert!(armor.get(key).unwrap().id().is_some() == should_have_id);
                assert_eq!(armor.get(key).unwrap().tags(), [ArmorTag::Light].into());
                if should_have_id {
                    assert!(match armor.get(key).unwrap().data_source() {
                        DataSource::Book(_) => panic!("should be custom"),
                        DataSource::Custom(None) => panic!("should have custom creator id"),
                        DataSource::Custom(Some(_)) => true,
                    });
                } else {
                    assert_eq!(
                        armor.get(key).unwrap().data_source(),
                        &DataSource::Custom(None)
                    );
                }
            }
            "Silken Armor" => {
                assert!(!worn);
                assert!(armor.get(key).unwrap().id().is_some() == should_have_id);
                assert_eq!(
                    armor.get(key).unwrap().tags(),
                    [
                        ArmorTag::Light,
                        ArmorTag::Artifact,
                        ArmorTag::Silent,
                        ArmorTag::Special
                    ]
                    .into()
                );
                assert_eq!(
                    armor.get(key).unwrap().data_source(),
                    &DataSource::Book(BookReference {
                        book_title: "Core Rulebook".to_owned(),
                        page_number: 600
                    })
                );
            }
            wrong => panic!("Unknown armor name: {}", wrong),
        }
    }
}
