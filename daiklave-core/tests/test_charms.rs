#[test]
fn test_solar_charms() {
    // Mortals cannot add Solar Charms, even if they meet the ability prerequisites
    // Solars must meet ability requirements
    // Solars must meet Essence requirements
    // ...unless they have the ability as a Supernal ability
    // Solars must meet Charm tree requirements
}

#[test]
fn test_evocations() {
    // Mortals cannot add Evocations, even if they have the artifact/hearthstone
    // Exalts must meet Essence requirements
    // Exalts must have the right artifact or hearthstone
    // Exalts must meet tree requirements
    // Upgrade-type Evocations require the upgraded Charm
}

#[test]
fn test_spells() {
    // Non-sorcerers cannot add Spells
    // Mortal sorcerers can add Terrestrial Spells
    // Solar Terrestrial sorcerers can add Terrestrial Spells
    // ...but not Celestial Spells or Solar circle spells
    // Solar Celestial sorcerer can add Terrestrial or Celestial spells
    // ...but not Solar circle spells
    // Solar Solar sorcerers can add any spell
}

#[test]
fn test_martial_arts_charms() {
    // Mortals cannot add MA charms, even if they have the right style
    // Exalts must have the right MA style
    // Exalts must meet the MA ability requirements of charms
    // Exalts must meet the Essence requirements of charms
    // ...unless they are Dawn Solars with Martial Arts Supernal
    // Exalts must satisfy the Charm tree prerequisites of their Styles
}

#[test]
fn test_eclipse_charms() {
    // Mortals cannot add Eclipse charms
    // Non-Eclipse Solars cannot add Eclipse charms
    // Eclipse Solars can add Eclipse Charms
    // Eclipse Solars must meet the Essence requirement of Eclipse Charms
}
