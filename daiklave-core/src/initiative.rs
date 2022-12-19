use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Default)]
pub struct Initiative(pub Option<i32>);

impl Initiative {
    pub fn is_in_combat(&self) -> bool {
        self.0.is_some()
    }

    pub fn current(&self) -> Option<i32> {
        self.0
    }

    pub fn is_crashed(&self) -> Option<bool> {
        self.0.map(|i| i <= 0)
    }

    pub fn join_battle(&mut self, roll_successes: i32) {
        self.0 = Some(roll_successes + 3);
    }

    pub fn leave_battle(&mut self) {
        self.0 = None;
    }

    pub fn set_initiative(&mut self, new_initiative: i32) -> Option<i32> {
        if self.0.is_some() {
            self.0 = Some(new_initiative);
        } 
        self.0
    }
}