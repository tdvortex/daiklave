use exalted_3e_gui::{character::CharacterBuilder, abilities::AbilityNameNoSubskill};

pub fn create_abilities(builder: CharacterBuilder) -> CharacterBuilder {
    vec![
        (AbilityNameNoSubskill::Awareness, 4),
        (AbilityNameNoSubskill::War, 3),
        (AbilityNameNoSubskill::Resistance, 3),
        (AbilityNameNoSubskill::Dodge, 3),
        (AbilityNameNoSubskill::Integrity, 2),
        (AbilityNameNoSubskill::Presence, 2),
        (AbilityNameNoSubskill::Socialize, 2),
        (AbilityNameNoSubskill::Athletics, 2),
        (AbilityNameNoSubskill::Linguistics, 1),
        (AbilityNameNoSubskill::Brawl, 1),
    ]
    .into_iter()
    .fold(builder, |ic, (ability_name_no_subskill, dots)| {
        ic.with_ability(ability_name_no_subskill, dots).unwrap()
    })
    .with_craft("Weapon Forging", 1)
    .with_martial_arts("Single Point Shining Into Void Style", 4)
    .with_specialty(AbilityNameNoSubskill::War, "While Outnumbered".to_owned())
    .unwrap()
    .with_specialty(AbilityNameNoSubskill::Socialize, "Tavern Gossip".to_owned())
    .unwrap()
    .with_craft_specialty("Weapon Forging", "Sharpening Blades".to_owned())
    .unwrap()
    .with_martial_arts_specialty(
        "Single Point Shining Into Void Style",
        "Join Battle".to_owned(),
    )
    .unwrap()
}