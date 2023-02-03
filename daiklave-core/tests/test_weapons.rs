#[test]
fn test_weapons_event_source() {
    // Default characters have the Unarmed weapon
    // Natural weapons are always equipped
    // Cannot equip or unequip missing weapons
    // Add some additional mundane weapons
    // Can have multiple copies of an unequipped mundane weapon
    // Worn weapons can be equipped and unequipped without needing hands
    // Can wield one handed weapons as main only, two different, off hand only, or paired
    // An equipped weapon always shows up as a quantity of 1
    // Can't equip a two-handed melee weapon if Strength is less than 3
    // Equipping a two handed weapon unequips all one-handed weapons
    // Create and add a unique artifact weapon
    // Check that lowering strength below 3 causes a heavy melee two-handed
    // weapon to unequip
    // Check you can remove an unequipped mundane weapon
    // Check you cannot remove a missing mundane weapon
    // Check you cannot remove an equipped mundane weapon without unequipped copies
    // Check you can remove an unequipped artifact weapon
}
