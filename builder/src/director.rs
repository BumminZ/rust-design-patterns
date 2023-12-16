use crate::{
    builder::Builder,
    component::{Attack, Element},
};

pub const DEFAULT_ATTACK_POWER: i32 = 30;
pub const DEFAULT_DEFENCE_POWER: i32 = 50;
pub struct Director;

impl Director {
    pub fn construct_pokemon(builder: &mut impl Builder, element: Element) {
        builder.set_primary_type(element);
        builder.set_secondary_type(element);
        builder.set_attack(Attack::new(DEFAULT_ATTACK_POWER, element));
        builder.set_defence(DEFAULT_DEFENCE_POWER);
        builder.set_learn(element)
    }
}
