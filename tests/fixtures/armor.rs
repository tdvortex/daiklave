use exalted_3e_gui::{armor::{Armor, ArmorTag}, data_source::{DataSource, BookReference}};

pub fn check_initial_armor_items(armor: &Armor, should_have_id: bool) {
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