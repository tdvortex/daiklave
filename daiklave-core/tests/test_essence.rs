#[test]
fn test_essence() {
    // Mortals should not have essence
    // Exalts (including Solars) should have essence
    // Exalts should be able to spend from either peripheral or personal
    // Exalts should be able to commit from either peripheral or personal, with
    // overflow splitting across pools
    // Exalts should not be able to spend more motes than they have available
    // Exalts should not be able to commit more motes than they have available
    // Recovering essence should refill peripheral first
    // ...and personal second
    // Uncommitting mote effects should make them spent again
    // Changing or lowering essence rating should end all mote commitments
    // and refill essence to full
}
