#![deny(clippy::pedantic)]
#![deny(clippy::all)]
//! A trivial test functor. Not holding any data, so this is basically just a linked list of free-nodes.
use higher::{Apply, Bind, Functor};
use higher_free_macro::free;

#[derive(Functor, Clone)]
struct TrivialFunctor<A>(A);

free!(TrivialFreeMonad<A>, TrivialFunctor<TrivialFreeMonad<A>>);

#[test]
fn test_trivial_functor() {
    let m = TrivialFreeMonad::lift_f(TrivialFunctor(37u32));
    let m = m.fmap(|x| x * 2);
    let m = m.bind(|x| TrivialFreeMonad::Free(Box::new(TrivialFunctor(TrivialFreeMonad::Pure(x)))));
    let f = TrivialFreeMonad::Pure((|x| x * 3).into());
    let m = m.apply(f);
    match m {
        TrivialFreeMonad::Free(b) => match *b {
            TrivialFunctor(f) => match f {
                TrivialFreeMonad::Free(b) => match *b {
                    TrivialFunctor(f) => match f {
                        TrivialFreeMonad::Pure(x) => assert_eq!(x, 37 * 6),
                        TrivialFreeMonad::Free(_) => unreachable!(),
                    },
                },
                TrivialFreeMonad::Pure(_) => unreachable!(),
            },
        },
        TrivialFreeMonad::Pure(_) => unreachable!(),
    }
}
