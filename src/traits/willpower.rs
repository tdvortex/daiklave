use eyre::{eyre, Result};

#[derive(Debug)]
pub struct Willpower {
    pub current: u8,
    pub maximum: u8,
}

impl Default for Willpower {
    fn default() -> Self {
        Self {
            current: 5,
            maximum: 5,
        }
    }
}

impl Willpower {
    pub fn recover_all(&mut self) {
        self.current = self.current.max(self.maximum);
    }

    pub fn recover_one(&mut self) {
        self.current = self.maximum.min(self.current + 1);
    }

    pub fn gain_one(&mut self) {
        self.current += 1;
    }

    pub fn spend_one(&mut self) -> Result<()> {
        if self.current == 0 {
            Err(eyre!("Cannot spend willpower while at zero"))
        } else {
            self.current -= 1;
            Ok(())
        }
    }
}
