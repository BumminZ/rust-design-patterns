mod pokemon;
mod trainer;

use crate::component::{Attack, Element};

// Builder to build pokemon;
pub trait Builder {
    type OutputType;
    fn set_primary_type(&mut self, element: Element);
    fn set_secondary_type(&mut self, element: Element);
    fn set_attack(&mut self, attack: Attack);
    fn set_defence(&mut self, defence: i32);
    fn set_learn(&mut self, element: Element);
    fn build(self) -> Self::OutputType;
}

pub use pokemon::PokemonBuilder;
