use crate::component::{Attack, Element};

#[derive(Debug)]
pub struct Pokemon {
    primary_type: Element,
    secondary_type: Element,
    attack: Attack,
    defence: i32,
    learn: Element,
}

impl Pokemon {
    pub fn new(
        primary_type: Element,
        secondary_type: Element,
        attack: Attack,
        defence: i32,
        learn: Element,
    ) -> Self {
        Self {
            primary_type,
            secondary_type,
            attack,
            defence,
            learn,
        }
    }

    pub fn set_primary_type(&self) -> Element {
        self.primary_type
    }
    pub fn set_secondary_type(&self) -> Element {
        self.secondary_type
    }
    pub fn set_attack(&self) -> Attack {
        self.attack
    }
    pub fn set_defence(&self) -> i32 {
        self.defence
    }
}
