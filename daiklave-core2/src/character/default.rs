use crate::Character;

impl<'source> Default for Character<'source> {
    fn default() -> Self {
        Self {
            name: "New Character",
            ..Default::default()
        }
    }
}
