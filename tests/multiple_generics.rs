#![deny(clippy::pedantic)]
#![deny(clippy::all)]
//! Tests if multiple generic parameters work, for the case that lifetimes are independent of mapping functions.
//! For simplicity, it just creates a `FreeResult` based on `Result`.

use higher_free_macro::free;
use higher::{Functor, Bind, Apply};

free!(FreeResult<O,E>, Result<FreeResult<O,E>,E>);


//just to appease clippy without disabling the lint....
macro_rules! assert_nearly_equal {
    ($a:expr, $b:expr, $c:expr) => {
        assert!((($a)-($b)).abs() < $c)
    };
}

#[test]
fn test_multiple_generics(){
    let m : FreeResult<_, String> = FreeResult::lift_f(Ok(37u32));
    let m = m.fmap(|x| x*2);
    let m = m.bind(|x| FreeResult::Free(Box::new(Ok(FreeResult::Pure(x)))));
    let f = FreeResult::Pure((|x| x*3).into());
    let m = m.apply(f);
    match m {
        FreeResult::Free(b) => {
            match *b {
                Ok(FreeResult::Free(b)) => {
                    match *b {
                        Ok(FreeResult::Pure(x)) => assert_eq!(x, 37*6),
                        _ => unreachable!()
                    }
                }
                _ => unreachable!()
            }
        }
        FreeResult::Pure(_) => unreachable!()
    }
}

#[test]
fn test_multiple_generics2(){
    let m : FreeResult<_, String> = FreeResult::lift_f(Ok(37u32));
    let m = m.bind(|_| FreeResult::<u32, _>::lift_f(Err("An early out.".to_owned())));
    match m{
        FreeResult::Free(m) => {
            match &*m {
                Ok(FreeResult::Free(m)) => {
                    match &**m {
                        Err(e) => assert_eq!(e, "An early out."),
                        _ => unreachable!()
                    }
                },
                _ => unreachable!()
            }
        },
        FreeResult::Pure(_) => unreachable!()
    }
}

#[test]
fn test_multiple_generics3(){
    let m : FreeResult<_, String> = FreeResult::lift_f(Ok(37u32));
    let f : FreeResult<_, String> = FreeResult::Pure(|x : u32| -> f64 {f64::from(x)*0.5f64}).fmap(Into::into);
    let m = m.apply(f);
    match m{
        FreeResult::Free(m) => {
            match &*m{
                Ok(k) => {
                    match k {
                        FreeResult::Pure(k) => assert_nearly_equal!(18.5f64, *k, f64::EPSILON),
                        FreeResult::Free(_) => unreachable!(),
                    }
                }
                Err(_) => unreachable!(),
            }
        },
        FreeResult::Pure(_) => unreachable!()
    }
}