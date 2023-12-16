#[derive(Clone, Copy, Debug)]
pub struct Attack {
    attack: i32,
    element: Element,
}

impl Attack {
    pub fn new(attack: i32, element: Element) -> Self {
        Self { attack, element }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Element {
    Normal,
    Fire,
    Fighting,
    Water,
    Flying,
    Grass,
    Poison,
    Electric,
    Ground,
    Psychic,
    Rock,
    Ice,
    Bug,
    Dragon,
    Ghost,
    Dark,
    Steel,
    Fairy,
    Stellar,
}
