#![deny(clippy::pedantic)]
#![deny(clippy::all)]
//! A small example text adventure, the logic of which is implemented as a Free Monad based eDSL.
//!
//! The goal of this game is to buy a sausage roll. With pickle.
//!
//! The code of this example contains a few peculiarities, to highlight features of and issues with the current
//! Free Monad code.
//! For instance, it intentionally does not have `Copy` implemented on the player's inventory, to illustrate how
//! one can work around a limitation in the current run!{} macro version.
//! Another thing that is not really that useful in practice is that all strings that are hardcoded are references
//! instead of owned copies. This is just to illustrate that lifetimes are supported too.
//!
//! In a real project, I'd just make all game state (here: inventory) `Copy`, and use owned values wherever possible to make the code
//! more concise. If `Copy` is not an option, I'd probably make a custom version of `run!{}` that allows to clone the
//! game state in a convenient way (see [higher issue 6](https://github.com/bodil/higher/issues/6)).
//!
//! But on to the explanation what is going on:
//! This project has 4 modules:
//! - `data` contains the data. Stuff like item types, item descriptions, rooms, etc.
//! - `dsl` contains the embedded domain specific language. In other words, a Functor and the corresponding Free Monad type (and some helpers)
//! - `logic` describes the game's main logic using the language defined in "dsl"
//! - `side_effects` actually runs the logic.
//!
//! The important part here is that all the stuff that isn't in `side_effects` is independent of the concrete implementation of `side_effects`.
//! The current `side_effects` runs a text-adventure, but it could just as well render as a visual-novel, without the need to touch any of the other modules.

mod data;
mod dsl;
mod logic;
mod side_effects;

fn main() -> std::io::Result<()> {
    //Let's build the game logic. As a data structure.
    let game = logic::game();

    //And now let's do something with it.
    side_effects::run(game)
}
