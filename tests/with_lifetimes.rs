#![deny(clippy::pedantic)]
#![deny(clippy::all)]
//! Test for the case that the Functor the Free Monad is based on has lifetime parameters that do not depend on the
//! lifetime of the mapping function in the Functor implementation.

use higher_free_macro::free;
use higher::{Functor, Bind, Apply};

#[derive(Functor, Clone)]
struct WithLifetimes<'a,'b, A>{
    s1 : &'a str,
    s2 : &'b str,
    next : A
}

free!(FreeWithLifetimes<'a,'b,A>, WithLifetimes<'a,'b,FreeWithLifetimes<'a,'b,A>>);

fn lifetime_helper<'a,'b>(s1 : &'a str, s2 : &'b str) -> FreeWithLifetimes<'a, 'b, u32>{
    let fv = FreeWithLifetimes::lift_f(WithLifetimes{ s1, s2, next: 15});
    fv.fmap(|x| x+1)
}

#[test]
fn test_with_lifetimes(){
    let s1 = "First";
    let s2 = "Second";
    let fv = lifetime_helper(s1, s2);
    let s3 = "Third";
    let s4 = "Fourth";
    let fv = fv.bind(|x| FreeWithLifetimes::lift_f(WithLifetimes{ s1: s3, s2: s4, next : x+2}));
    let s5 = "Fifth";
    let s6 = "Sixth";
    let fa = FreeWithLifetimes::lift_f(WithLifetimes{s1: s5, s2: s6, next : (|x| x+3).into()});
    let fv = fv.apply(fa);
    match fv {
        FreeWithLifetimes::Free(v) => {
            assert_eq!(v.s1, s5);
            assert_eq!(v.s2, s6);
            match v.next {
                FreeWithLifetimes::Free(v) => {
                    assert_eq!(v.s1, s1);
                    assert_eq!(v.s2, s2);
                    match v.next {
                        FreeWithLifetimes::Free(v) => {
                            assert_eq!(v.s1, s3);
                            assert_eq!(v.s2, s4);
                            match v.next {
                                FreeWithLifetimes::Free(_) => unreachable!(),
                                FreeWithLifetimes::Pure(a) => {
                                    assert_eq!(a, 21);
                                },
                            }
                        },
                        FreeWithLifetimes::Pure(_) => unreachable!(),
                    }
                },
                FreeWithLifetimes::Pure(_) => unreachable!(),
            }
        },
        FreeWithLifetimes::Pure(_) => unreachable!()
    }
}