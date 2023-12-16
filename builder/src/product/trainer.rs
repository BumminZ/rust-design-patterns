use crate::component::{Attack, Element};

pub struct Trainer {
    pokemon_type: Element,
    command_attack: Attack,
    command_defend: i32,
}

impl Trainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Train {:?} pokemon", self.pokemon_type)?;
        writeln!(f, "Attack: {:?}", self.command_attack)?;
        writeln!(f, "Defend: {:?}", self.command_defend)?;

        Ok(())
    }
}
