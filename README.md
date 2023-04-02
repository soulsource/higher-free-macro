# Disclaimer
This is a proof-of-concept. Please do not use this for production code, unless you do a careful review first.
There probably are tons of bugs. Pull requests are more than welcome.

# Description
This is a simple port of Haskell's [Control.Monad.Free](https://hackage.haskell.org/package/free/docs/Control-Monad-Free.html) package to Rust, using the traits from [higher](https://crates.io/crates/higher).
This crate uses macros to generate a unique Free Monad type for each user-supplied Functor.

# Usage
The usage is rather straightforward. First, you create your Functor type, then call the `free!` macro to create the actual Free Monad type based on it. For example:

```Rust
use higher_free_macro::free;
use higher::*;

#[derive(Clone,Functor)]
enum TestFunctor<A>{
    Variant(A)
}

free!(FreeTestFunctor<A>, TestFunctor<FreeTestFunctor<A>>);
```

In order to get an actual `Monad`, the `Functor` needs to implement `Clone`. It is needed for the `Apply` implementation, which in a Free Monad needs to call `bind()` on `self` for each `Pure` node in the other Free Monad, but `bind()`, as written in the `higher` crate, consumes `self`. The other traits (`Functor`, `Bind`, `Pure`) do not need cloneability, but without `Apply` the type is not `Applicative`, and without `Applicative` it is not `Monad`.

If the lifetime of the result of the Functor's `fmap(f)` depends on the lifetime of the function `f` passed into it, the macro needs to know which lifetime parameter is affected. For instance, in the more involved example below, the macro is called as `free!(<'a>, FreeSaturday<'a, A>, Saturday<'a,FreeSaturday<'a, A>>);`, since `Saturday`'s `fmap(f : F)` has `F : Fn(A)->B + 'a`.

# Why macros though?
 This is mainly because a fully generic implementation seems impossible without [Non-lifetime Binders](https://github.com/rust-lang/rust/issues/108185), which would be needed to express the bound that the type of `Functor<A>::Result<T>` should not depend on `A`. Also, the [Never Type](https://github.com/rust-lang/rust/issues/35121) would be nice to have for a generic implementation.

 I've actually done a [proof-of-concept implementation](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021&gist=1af5b0970cfe400a9245483f84080b8f) of a fully generic `Free` type using Nightly, but didn't follow this further, because it doesn't work in the stable toolchain yet.

 # A more involved example

The trivial example above doesn't really show much. A more involved example would for instance be building an embedded Domain Specific Language.
The code below plans a night of drinking at different bars, with varying beer quality. However, while planning we don't actually know the quality of the beer at each bar. That's a piece of information we only get while interpreting the plan (as in: that's a _side effect_ of actually going there, and tasting the beer):

```Rust
use std::rc::Rc;
use higher_free_macro::free;
use higher::*;

#[derive(Debug, Clone, PartialEq)]
enum BeerQuality{
    Lukewarm,
    Refreshing
}

#[derive(Clone)]
enum Saturday<'a, Next>{
    GoToBar{
        name_of_bar : &'a str,
        next : Next
    },
    DrinkABeer (Rc<dyn Fn(BeerQuality)->Next + 'a>) //Rc, because it's cloneable, dyn to keep it out of the type signature.
}

//Saturday is too complex to derive Functor for it (at the moment at least).
//Note that the lifetime of the continuation function for DrinkABeer depends on the lifetime of f : Fn(Next) -> B + 'a.
impl<'a, Next : 'a> Functor<'a, Next> for Saturday<'a, Next>{
    type Target<T> = Saturday<'a, T>;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(Next) -> B + 'a {
        match self {
            Saturday::GoToBar { name_of_bar, next } => Saturday::GoToBar { name_of_bar, next: f(next) },
            Saturday::DrinkABeer(continuation) => {
                Saturday::DrinkABeer(Rc::new(move |x| f(continuation(x))))
            },
        }
    }
}

//Here we create the Free Monad FreeSaturday over the Functor Saturday
//The result of fmap(f) depends on the lifetime of f, 'a. That's why we pass this to the macro as first parameter.
free!(<'a>, FreeSaturday<'a, A>, Saturday<'a,FreeSaturday<'a, A>>);

//Helpers, so we don't have to write FreeSaturday::lift_f() all the time
fn go_to_bar(s : &str) -> FreeSaturday<'_, ()>{ FreeSaturday::lift_f(Saturday::GoToBar { name_of_bar: s, next: () }) }
fn drink_a_beer<'a>() -> FreeSaturday<'a, BeerQuality>{ FreeSaturday::lift_f(Saturday::DrinkABeer(Rc::new(std::convert::identity))) }

//The plan for a nice evening. If someone serves lukewarm beer, we go home. Assumes that beer quality is constant at each bar.
fn a_nice_evening() -> FreeSaturday<'static,()>{
    run! { //yes, higher has do-notation :-D
        drink_a_beer(); //at home. Don't care if lukewarm.
        go_to_bar("Sunken Norwegian");
        x <= drink_a_beer();
        if x != BeerQuality::Lukewarm { run! {
            drink_a_beer(); //alredy know if the beer here was lukewarm or not.
            go_to_bar("Le Rieur Sanglier");
            x <= drink_a_beer();
            if x != BeerQuality::Lukewarm { run! {
                drink_a_beer();
                go_to_bar("Coyote Ugly");
                x <= drink_a_beer();
                if x != BeerQuality::Lukewarm { run! {
                    drink_a_beer(); 
                    yield ()
                }} else{ run! { yield () } } 
            }} else{ run! { yield () } } 
        } } else{ run! { yield () } }
    }
}
//This wouldn't compile if a_nice_evening weren't a Monad. Sooo, it obviously is.
fn _test_if_a_nice_evening_is_monad() -> impl Monad<'static, ()>{
    a_nice_evening()

}

//The interpreter that counts how many beers were consumed at which bar is a bit more convoluted than it would need to be, because:
//it can't match directly for box contents here, because https://github.com/rust-lang/rust/issues/87121 isn't implemented yet :-(
//and also cannot match for Saturday::GoToBar using pattern guards, because if-let pattern-guards aren't stable either: https://github.com/rust-lang/rust/issues/51114
fn count_beers_consumed_per_bar(evening : FreeSaturday<()>) -> Vec<(&str, u32)>{
    //for this illustration let's just assume that get_beer_quality_of_location() is causing side effects.
    fn get_beer_quality_of_location(l : &str) -> BeerQuality { if l == "Le Rieur Sanglier" {BeerQuality::Lukewarm} else {BeerQuality::Refreshing}}
    fn interpret_evening_step<'a, 'b : 'a>(l : &'b str, mut v : Vec<(&'a str, u32)>, saturday : FreeSaturday<'b,()>) -> Vec<(&'a str, u32)>{
        match (l,&*v,saturday){
            (_,_,FreeSaturday::Pure(_)) => v,
            (l, [.., (lo,co)], FreeSaturday::Free(f)) if *lo == l=> {
                match *f {
                    Saturday::GoToBar { name_of_bar, next } => interpret_evening_step(name_of_bar, v, next),
                    Saturday::DrinkABeer(next) => {
                        v.last_mut().unwrap().1 = co+1;
                        interpret_evening_step(l,v,next(get_beer_quality_of_location(l)))
                    }
                }
            }
            (l, _, FreeSaturday::Free(f)) => {
                match *f {
                    Saturday::GoToBar { name_of_bar, next } => interpret_evening_step(name_of_bar, v, next),
                    Saturday::DrinkABeer(next) => {
                        v.push((l,1));
                        interpret_evening_step(l,v,next(get_beer_quality_of_location(l)))
                    }
                }
            }
        }
    }
    interpret_evening_step("Home", Vec::new(), evening)
}

//Just to show this works. Prints [("Home", 1), ("Sunken Norwegian", 2), ("Le Rieur Sanglier", 1)].
fn main() {
    let the_free_monad = a_nice_evening();
    let beers_per_bar = count_beers_consumed_per_bar(the_free_monad);
    println!("{:?}", beers_per_bar);
}
```

## An even more involved example
The "examples/text-adventure" folder in the source repo contains a (very) short text-adventure to illustrate the usage of a Free Monads as an embedded Domain Specific Language. It also shows some potential show-stoppers one should be aware of when creating a Free Monad based eDSL.

# A note about the origin of this code
The work on this project started at stillalive studios. The original goal was to learn about potential applications of Free Monads in game development, but this project has meanwhile outgrown that original plan, and has become a full proof-of-concept implementation for Free Monads in Rust.