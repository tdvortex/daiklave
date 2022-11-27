#[derive(Debug)]
pub struct Health {
    health_boxes: Vec<WoundLevel>,
    damage_boxes: Vec<Option<DamageLevel>>
}

impl Default for Health {
    fn default() -> Self {
        Self {
            health_boxes: vec![WoundLevel::Zero, 
            WoundLevel::MinusOne, 
            WoundLevel::MinusOne, 
            WoundLevel::MinusTwo, 
            WoundLevel::MinusTwo, 
            WoundLevel::MinusFour, 
            WoundLevel::Incapacitated],
            damage_boxes: vec![None; 7]
        }
    }
}

impl Health {
    pub fn health_boxes(&self) -> &Vec<WoundLevel> {
        &self.health_boxes
    }

    pub fn current_damage(&self) -> &Vec<Option<DamageLevel>> {
        &self.damage_boxes
    }

    pub fn take_damage(&mut self, damage_type: DamageLevel, amount: usize) {
        self.damage_boxes.append(&mut vec![Some(damage_type); amount]);
        self.damage_boxes.sort_by(|a, b| b.cmp(a));
        (0..amount).for_each(|_| {self.damage_boxes.pop();});
    }

    pub fn heal_damage(&mut self, damage_type: DamageLevel, amount: usize) {
        let mut remaining = amount;

        for damage_box in &mut self.damage_boxes {
            if let Some(damage) = damage_box {
                if *damage == damage_type {
                    *damage_box = None;
                    remaining -= 1;
                    if remaining == 0 {
                        break;
                    }
                }
            }
        }

        self.damage_boxes.sort_by(|a, b| b.cmp(a));
    }

    pub fn add_health_boxes(&mut self, mut additional_boxes: Vec<WoundLevel>) {
        let amount = additional_boxes.len();
        self.health_boxes.append(&mut additional_boxes);
        self.health_boxes.sort();
        self.damage_boxes.append(&mut vec![None; amount]);
        self.damage_boxes.sort_by(|a, b| b.cmp(a));
    }

    pub fn delete_health_boxes(&mut self, wound_level: WoundLevel, amount: usize) {
        let mut new_health_boxes = vec![];
        let mut deleted = 0;
        for health_box in self.health_boxes.iter() {
            if *health_box == wound_level && deleted < amount {
                deleted += 1;
            } else {
                new_health_boxes.push(*health_box);
            }
        }
        self.health_boxes = new_health_boxes;
        (0..deleted).for_each(|_| {self.damage_boxes.pop();});
    }

    pub fn current_wound_level(&self) -> Option<WoundLevel> {
        let damage = self.damage_boxes.iter().filter(|x| x.is_some()).count();
        if damage == 0 {
            None
        } else if damage >= self.health_boxes.len() {
            Some(WoundLevel::Incapacitated)
        } else {
            Some(self.health_boxes[damage])
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)] 
pub enum WoundLevel {
    Zero,
    MinusOne,
    MinusTwo,
    MinusFour,
    Incapacitated,
 }

 #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
 pub enum DamageLevel {
    Bashing,
    Lethal,
    Aggravated,
 }