#![deny(clippy::pedantic)]
#![deny(clippy::all)]
//! Tests if creating a Free Monad for a Vec works. Not sure if this is useful in any way.
//! It is a nice illustration that Free Monads are tree-like though.

use higher_free_macro::free;
use higher::{Functor, Bind, Apply};

free!(FreeVec<A>, Vec<FreeVec<A>>);


//just to appease clippy without disabling the lint....
macro_rules! assert_nearly_equal {
    ($a:expr, $b:expr, $c:expr) => {
        assert!((($a)-($b)).abs() < $c)
    };
}

#[test]
fn test_vector(){
    let free_monad = FreeVec::lift_f(vec![2,3,4]);
    let free_monad_after_fmap = free_monad.fmap(|x| x*2);
    let free_monad_after_bind = free_monad_after_fmap.bind(|x| if x%3 == 0 {FreeVec::Pure(x)} else {FreeVec::lift_f(vec![x,x+1])});
    let functions = FreeVec::lift_f(vec![(|x| f64::from(x) / 3.0) as fn(u32)->f64, (|x| f64::from(x+2)) as fn(u32)->f64]);
    let free_monad_after_apply = free_monad_after_bind.apply(functions.fmap(Into::into));
    match free_monad_after_apply {
        FreeVec::Free(v) => {
            match &**v{
                [FreeVec::Free(left), FreeVec::Free(right)] => {
                    match &***left {
                        [FreeVec::Free(left), FreeVec::Pure(middle), FreeVec::Free(right)] => {
                            match &***left {
                                [FreeVec::Pure(left), FreeVec::Pure(right)] => {
                                    assert_nearly_equal!(4.0f64/3.0f64, *left, f64::EPSILON);
                                    assert_nearly_equal!(5.0f64/3.0f64, *right, f64::EPSILON);
                                },
                                _ => unreachable!()
                            }
                            assert_nearly_equal!(2.0f64, *middle, f64::EPSILON);
                            match &***right {
                                [FreeVec::Pure(left),FreeVec::Pure(right)] => {
                                    assert_nearly_equal!(8.0f64/3.0f64, *left, f64::EPSILON);
                                    assert_nearly_equal!(3.0f64, *right, f64::EPSILON);
                                },
                                _ => unreachable!()
                            }
                        },
                        _ => unreachable!()
                    }
                    match &***right {
                        [FreeVec::Free(left),FreeVec::Pure(middle),FreeVec::Free(right)] => {
                            match &***left {
                                [FreeVec::Pure(left),FreeVec::Pure(right)] => {
                                    assert_nearly_equal!(6.0f64, *left, f64::EPSILON);
                                    assert_nearly_equal!(7.0f64, *right, f64::EPSILON);
                                },
                                _ => unreachable!()
                            }
                            assert_nearly_equal!(8.0f64, *middle, f64::EPSILON);
                            match &***right {
                                [FreeVec::Pure(left),FreeVec::Pure(right)] => {
                                    assert_nearly_equal!(10.0f64, *left, f64::EPSILON);
                                    assert_nearly_equal!(11.0f64, *right, f64::EPSILON);
                                },
                                _ => unreachable!()
                            }
                        },
                        _ => unreachable!()
                    }
                },
                _ => unreachable!()
            }
        }
        FreeVec::Pure(_) => unreachable!()
    }
}