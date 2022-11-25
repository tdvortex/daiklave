mod character;
mod range_bands;

#[cfg(test)]
mod tests {
    #[test]
    fn default_character() {
        use crate::character::Character;
        let character = Character::default();
        dbg!(character);
    }
}
