#![deny(clippy::pedantic)]
#![deny(clippy::all)]
//! Tests if a trivial functor, which's lifetime depends on the mapping function, works.
use std::rc::Rc;
use higher::{Functor, Bind};
use higher_free_macro::free;

#[derive(Clone)]
struct TrivWithLifetime<'a,A,B>{
    next : Rc<dyn Fn(B)->A + 'a>,
}

impl<'a,A : 'a,B : 'a> Functor<'a,A> for TrivWithLifetime<'a,A,B> {
    type Target<T> = TrivWithLifetime<'a,T,B>;

    fn fmap<C, F>(self, f: F) -> Self::Target<C>
    where
        F: Fn(A) -> C + 'a {
        TrivWithLifetime{ next : Rc::new(move |x| f((self.next)(x)))}
    }
}

free!(<'a>, FreeTriv<'a,A,B>, TrivWithLifetime<'a,FreeTriv<'a,A,B>,B>);

#[test]
fn test_trivial_with_lifetime(){
    let f = FreeTriv::lift_f(TrivWithLifetime{next : Rc::new(i32::unsigned_abs)});
    let f = f.bind(FreeTriv::Pure);
    match f {
        FreeTriv::Free(f) => {
            let n = (f.next)(-4);
            match n {
                FreeTriv::Pure(v) => assert_eq!(v,4u32),
                FreeTriv::Free(_) => unreachable!(),
            }
        },
        FreeTriv::Pure(_) => unreachable!()
    }
}