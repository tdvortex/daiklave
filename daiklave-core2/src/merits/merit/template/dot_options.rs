#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MeritTemplateDotOptions {
    Fixed(u8),
    Variable([Option<String>; 6]),
}

impl MeritTemplateDotOptions {
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        let mut to_iter = [None; 6];
        match self {
            MeritTemplateDotOptions::Fixed(fixed) => {
                if *fixed <= 5 {
                    to_iter[*fixed as usize] = Some(*fixed);
                }
            }
            MeritTemplateDotOptions::Variable(maybe_detail) => {
                maybe_detail
                    .iter()
                    .enumerate()
                    .for_each(|(i, maybe_description)| {
                        if maybe_description.is_some() {
                            to_iter[i] = Some(i as u8);
                        }
                    });
            }
        }

        to_iter.into_iter().flatten()
    }
}
