/// For one-handed weapons, the position of that weapon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EquipHand {
    /// Wielded in the main hand
    MainHand,
    /// Wielded in the off hand
    OffHand,
}

impl Default for EquipHand {
    fn default() -> Self {
        Self::MainHand
    }
}
