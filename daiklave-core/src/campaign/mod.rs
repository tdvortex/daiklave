use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Campaign {
    _id: i32,
    name: String,
    description: Option<String>,
    bot_channel: i64,
}

impl Campaign {
    pub fn new(id: i32, name: String, bot_channel: i64, description: Option<String>) -> Self {
        Self {
            _id: id,
            name,
            description,
            bot_channel,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn bot_channel(&self) -> i64 {
        self.bot_channel
    }
}