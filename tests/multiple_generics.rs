//! Tests if multiple generic parameters work, for the case that lifetimes are independent of mapping functions.
//! For simplicity, it just creates a FreeResult

use higher_free_macro::free;
use higher::{Functor, Bind, Apply};

free!(FreeResult<O,E>, Result<FreeResult<O,E>,E>);

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
                Ok(f) => {
                    match f {
                        FreeResult::Free(b) => {
                            match *b {
                                Ok(f) => {
                                    match f{
                                        FreeResult::Pure(x) => assert_eq!(x, 37*6),
                                        _ => unreachable!()
                                    }
                                }
                                _ => unreachable!()
                            }
                        },
                        _ => unreachable!()
                    }
                }
                _ => unreachable!()
            }
        }
        _ => unreachable!()
    }
}

#[test]
fn test_multiple_generics2(){
    let m : FreeResult<_, String> = FreeResult::lift_f(Ok(37u32));
    let m = m.bind(|_| FreeResult::<u32, _>::lift_f(Err("An early out.".to_owned())));
    match m{
        FreeResult::Free(m) => {
            match &*m {
                Ok(m) => {
                    match m{
                        FreeResult::Free(m) => {
                            match &**m {
                                Err(e) => assert_eq!(e, "An early out."),
                                _ => unreachable!()
                            }
                        }
                        _ => unreachable!()
                    }
                },
                _ => unreachable!()
            }
        },
        _ => unreachable!()
    }
}

#[test]
fn test_multiple_generics3(){
    let m : FreeResult<_, String> = FreeResult::lift_f(Ok(37u32));
    let f : FreeResult<_, String> = FreeResult::Pure(|x : u32| -> f32 {(x as f32)*0.5f32}).fmap(Into::into);
    let m = m.apply(f);
    match m{
        FreeResult::Free(m) => {
            match &*m{
                Ok(k) => {
                    match k {
                        FreeResult::Pure(k) => assert_eq!(18.5f32, *k),
                        FreeResult::Free(_) => unreachable!(),
                    }
                }
                Err(_) => unreachable!(),
            }
        },
        _ => unreachable!()
    }
}