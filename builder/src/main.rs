#![allow(unused)]
use builder::{Builder, PokemonBuilder};
use component::Element;
use director::Director;

mod builder;
mod component;
mod director;
mod product;

fn main() {
    let mut pokemon_builder = PokemonBuilder::default();

    Director::construct_pokemon(&mut pokemon_builder, Element::Fire);

    let pokemon = pokemon_builder.build();

    println!("Pokemon built: {:?}", pokemon)
}
