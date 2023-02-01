use crate::{exaltation::Exaltation, languages::language::LanguageMutation, Character};

use super::merit_new::{Merit, MeritSource, NonStackableMerit, StackableMerit};

pub struct Merits<'view, 'source>(pub(crate) &'view Character<'source>);

impl<'view, 'source> Merits<'view, 'source> {
    pub fn iter(&self) -> impl Iterator<Item = Merit<'source>> + '_ {
        let armor = self.0.armor();
        let from_armor = armor
            .iter()
            .filter_map(|armor_name| armor.get(armor_name))
            .flat_map(|armor_item| armor_item.merits().into_iter());

        let weapons = self.0.weapons();
        let from_weapons = weapons
            .iter()
            .filter_map(|(name, equipped)| weapons.get(name, equipped))
            .flat_map(|weapon| weapon.merits().into_iter());

        let wonders = self.0.wonders();
        let from_wonders = wonders
            .iter()
            .filter_map(|name| wonders.get(name))
            .flat_map(|wonder| wonder.merits().into_iter());

        let demenses = self.0.demenses_no_manse.iter().map(|(name, level)| {
            Merit(MeritSource::Demense {
                name: *name,
                has_manse: false,
                geomancy_level: *level,
            })
        });

        let hearthstones = self.0.hearthstones();
        let from_hearthstones = hearthstones
            .iter()
            .filter_map(|name| hearthstones.get(name))
            .flat_map(|hearthstone| hearthstone.merits().into_iter());

        let exalted_healing = if let Exaltation::Mortal(mortal) = &self.0.exaltation {
            if mortal.exalted_healing {
                vec![Merit(MeritSource::ExaltedHealing { is_exalt: false })]
            } else {
                vec![]
            }
        } else {
            vec![Merit(MeritSource::ExaltedHealing { is_exalt: true })]
        }
        .into_iter();

        let (local_count, mut language_merits) = self.0.other_languages.iter().fold(
            (0, Vec::new()),
            |(local_count, mut majors), language| match language {
                LanguageMutation::MajorLanguage(major) => {
                    majors.push(Merit(MeritSource::MajorLanguage(*major)));
                    (local_count, majors)
                }
                LanguageMutation::LocalTongue(_) => (local_count + 1, majors),
            },
        );
        if local_count > 0 {
            language_merits.push(Merit(MeritSource::LocalTongues { count: local_count }));
        }
        let languages = language_merits.into_iter();

        let martial_artists = self
            .0
            .martial_arts()
            .iter()
            .map(|style_name| Merit(MeritSource::MartialArtist { style_name }));

        let mortal_sorcerer = if let Exaltation::Mortal(mortal) = &self.0.exaltation {
            if mortal.sorcery.is_some() {
                vec![Merit(MeritSource::MortalSorcerer)]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
        .into_iter();

        let nonstackable = self.0.nonstackable_merits.iter().map(|(name, instance)| {
            Merit(MeritSource::NonStackable(NonStackableMerit {
                name,
                instance,
            }))
        });
        let stackable =
            self.0
                .stackable_merits
                .iter()
                .map(|((template_name, detail), instance)| {
                    Merit(MeritSource::Stackable(StackableMerit {
                        template_name,
                        detail,
                        instance,
                    }))
                });
        let maybe_sorcery = self.0.sorcery();
        let sorcery_merits = maybe_sorcery.iter().flat_map(|sorcery| {
            sorcery
                .archetypes()
                .filter_map(|name| sorcery.archetype(name))
                .flat_map(|with_merits| with_merits.merits().iter().collect::<Vec<Merit>>().into_iter())
        });

        from_armor
            .chain(from_weapons)
            .chain(from_wonders)
            .chain(demenses)
            .chain(from_hearthstones)
            .chain(exalted_healing)
            .chain(languages)
            .chain(martial_artists)
            .chain(mortal_sorcerer)
            .chain(nonstackable)
            .chain(stackable)
            .chain(sorcery_merits)
            .collect::<Vec<Merit>>()
    }
}
