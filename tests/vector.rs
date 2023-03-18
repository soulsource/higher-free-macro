//! Tests if creating a Free Monad for a Vec works. Not sure if this is useful in any way.
//! It is a nice illustration that Free Monads are tree-like though.

use higher_free_macro::free;
use higher::{Functor, Bind, Apply};

free!(FreeVec<A>, Vec<FreeVec<A>>);

#[test]
fn test_vector(){
    let fv = FreeVec::lift_f(vec![2,3,4]);
    let fv = fv.fmap(|x| x*2);
    let fv = fv.bind(|x| if x%3 == 0 {FreeVec::Pure(x)} else {FreeVec::lift_f(vec![x,x+1])});
    let f = FreeVec::lift_f(vec![(|x| (x as f32) / 3.0) as fn(u32)->f32, (|x| (x+2) as f32) as fn(u32)->f32]);
    let r = fv.apply(f.fmap(Into::into));
    match r {
        FreeVec::Free(v) => {
            match &**v{
                [a,b] => {
                    match a {
                        FreeVec::Free(v) => {
                            match &***v {
                                [a,b,c] => {
                                    match a {
                                        FreeVec::Free(v) => {
                                            match &***v {
                                                [a,b] => {
                                                    match a{
                                                        FreeVec::Free(_) => unreachable!(),
                                                        FreeVec::Pure(v) => {assert_eq!(4.0f32/3.0f32, *v)}
                                                    }
                                                    match b{
                                                        FreeVec::Free(_) => unreachable!(),
                                                        FreeVec::Pure(v) => {assert_eq!(5.0f32/3.0f32, *v)}
                                                    }
                                                },
                                                _ => unreachable!()
                                            }
                                        }
                                        FreeVec::Pure(_) => unreachable!(),
                                    }
                                    match b {
                                        FreeVec::Free(_) => unreachable!(),
                                        FreeVec::Pure(v) => assert_eq!(2.0f32, *v),
                                    }
                                    match c {
                                        FreeVec::Free(v) => {
                                            match &***v {
                                                [a,b] => {
                                                    match a{
                                                        FreeVec::Free(_) => unreachable!(),
                                                        FreeVec::Pure(v) => {assert_eq!(8.0f32/3.0f32, *v)}
                                                    }
                                                    match b{
                                                        FreeVec::Free(_) => unreachable!(),
                                                        FreeVec::Pure(v) => {assert_eq!(3.0f32, *v)}
                                                    }
                                                },
                                                _ => unreachable!()
                                            }
                                        }
                                        FreeVec::Pure(_) => unreachable!(),
                                    }
                                },
                                _ => unreachable!()
                            }
                        }
                        FreeVec::Pure(_) => unreachable!()
                    }

                    match b {
                        FreeVec::Free(v) => {
                            match &***v {
                                [a,b,c] => {
                                    match a {
                                        FreeVec::Free(v) => {
                                            match &***v {
                                                [a,b] => {
                                                    match a{
                                                        FreeVec::Free(_) => unreachable!(),
                                                        FreeVec::Pure(v) => {assert_eq!(6.0f32, *v)}
                                                    }
                                                    match b{
                                                        FreeVec::Free(_) => unreachable!(),
                                                        FreeVec::Pure(v) => {assert_eq!(7.0f32, *v)}
                                                    }
                                                },
                                                _ => unreachable!()
                                            }
                                        }
                                        FreeVec::Pure(_) => unreachable!(),
                                    }
                                    match b {
                                        FreeVec::Free(_) => unreachable!(),
                                        FreeVec::Pure(v) => assert_eq!(8.0f32, *v),
                                    }
                                    match c {
                                        FreeVec::Free(v) => {
                                            match &***v {
                                                [a,b] => {
                                                    match a{
                                                        FreeVec::Free(_) => unreachable!(),
                                                        FreeVec::Pure(v) => {assert_eq!(10.0f32, *v)}
                                                    }
                                                    match b{
                                                        FreeVec::Free(_) => unreachable!(),
                                                        FreeVec::Pure(v) => {assert_eq!(11.0f32, *v)}
                                                    }
                                                },
                                                _ => unreachable!()
                                            }
                                        }
                                        FreeVec::Pure(_) => unreachable!(),
                                    }
                                },
                                _ => unreachable!()
                            }
                        }
                        FreeVec::Pure(_) => unreachable!()
                    }
                },
                _ => unreachable!()
            }
        }
        FreeVec::Pure(_) => unreachable!()
    }
}