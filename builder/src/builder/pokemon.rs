use crate::{
    component::{Attack, Element},
    product::Pokemon,
};

use super::Builder;

#[derive(Default)]
pub struct PokemonBuilder {
    primary_type: Option<Element>,
    secondary_type: Option<Element>,
    attack: Option<Attack>,
    defence: Option<i32>,
    learn: Option<Element>,
}

impl Builder for PokemonBuilder {
    type OutputType = Pokemon;

    fn set_primary_type(&mut self, element: Element) {
        self.primary_type = Some(element)
    }

    fn set_secondary_type(&mut self, element: Element) {
        self.secondary_type = Some(element)
    }

    fn set_attack(&mut self, attack: Attack) {
        self.attack = Some(attack)
    }

    fn set_defence(&mut self, defence: i32) {
        self.defence = Some(defence)
    }

    fn set_learn(&mut self, element: Element) {
        self.learn = Some(element)
    }

    fn build(self) -> Pokemon {
        Pokemon::new(
            self.primary_type.expect("Please set an element."),
            self.secondary_type.expect("Please set an element."),
            self.attack
                .expect("Please set an element and damage amount."),
            self.defence.expect("Please set a defence amount."),
            self.learn.expect("Please set an element."),
        )
    }
}
