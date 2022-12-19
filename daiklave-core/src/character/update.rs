use crate::Character;

#[derive(Debug, Default)]
pub struct CharacterBaseDiff(pub Option<(String, Option<String>, i16, i16, i16, i16)>);

impl Character {
    pub fn compare_newer_base(&self, newer: &Character) -> CharacterBaseDiff {
        let mut diff = CharacterBaseDiff::default();

        let eq_condition = (self.name.as_str() == newer.name.as_str())
            && (self.concept.as_deref() == newer.concept.as_deref())
            && (self.willpower.current == newer.willpower.current)
            && (self.willpower.maximum == newer.willpower.maximum)
            && (self.experience.current.min(i16::MAX as u16)
                != newer.experience.current.max(i16::MAX as u16))
            && (self.experience.total.min(i16::MAX as u16)
                != newer.experience.total.max(i16::MAX as u16));

        if !eq_condition {
            diff = CharacterBaseDiff(Some((
                newer.name.clone(),
                newer.concept.clone(),
                newer.willpower.current as i16,
                newer.willpower.maximum as i16,
                newer
                    .experience
                    .current
                    .min(i16::MAX as u16)
                    .try_into()
                    .unwrap(),
                newer
                    .experience
                    .total
                    .min(i16::MAX as u16)
                    .try_into()
                    .unwrap(),
            )));
        }

        diff
    }
}
