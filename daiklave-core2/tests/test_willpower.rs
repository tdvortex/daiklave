use daiklave_core2::{Character, SolarTraits};

#[test]
fn test_willpower_character() {
    // Check default (mortal)
    let mut character = Character::default();
    assert_eq!(character.willpower().rating(), 3);
    assert_eq!(character.willpower().current(), 3);

    // Check default (exalt)
    let solar_traits = SolarTraits::builder().build();
    character.set_solar(&solar_traits).unwrap();
    assert_eq!(character.willpower().rating(), 5);
    assert_eq!(character.willpower().current(), 5);

    // Check modifying current willpower
    assert!(character.check_set_current_willpower(3).is_ok());
    assert!(character.set_current_willpower(3).is_ok());
    assert_eq!(character.willpower().rating(), 5);
    assert_eq!(character.willpower().current(), 3);

    // Check modifying willpower rating
    assert!(character.check_set_willpower_rating(7).is_ok());
    assert!(character.set_willpower_rating(7).is_ok());
    assert_eq!(character.willpower().rating(), 7);
    assert_eq!(character.willpower().current(), 7);
}