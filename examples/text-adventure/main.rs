//! A small example text adventure, the logic of which is implemented as a Free Monad based eDSL.
//! 
//! The goal of this game is to buy a sausage roll.

mod dsl;
mod logic;
mod side_effects;

fn main() {
    let game = logic::game();
}