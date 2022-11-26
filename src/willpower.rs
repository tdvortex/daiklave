use eyre::{eyre, Result};

pub trait HasWillpower {
    fn recover_one_willpower(&mut self);
    fn recover_all_willpower(&mut self);
    fn gain_one_willpower(&mut self);
    fn spend_one_willpower(&mut self) -> Result<()>;
}

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
