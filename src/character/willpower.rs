#[derive(Debug)]
pub struct Willpower {
    current: u8,
    maximum: u8,
}

impl Default for Willpower {
    fn default() -> Self {
        Self {
            current: 5,
            maximum: 5,
        }
    }
}
