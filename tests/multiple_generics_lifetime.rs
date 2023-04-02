#![deny(clippy::pedantic)]
#![deny(clippy::all)]
//! Tests if multiple generic parameters work, if the return value's lifetime depends on the mapping function lifetime.

use higher::{Apply, Bind, Functor};
use higher_free_macro::free;
use std::rc::Rc;

#[derive(Clone)]
struct TestFunctor<'a, 'b, A, B> {
    data: &'b B,
    next: Rc<dyn Fn(i32) -> A + 'a>,
}

impl<'a, 'b, A: 'a, B> Functor<'a, A> for TestFunctor<'a, 'b, A, B> {
    type Target<T> = TestFunctor<'a, 'b, T, B>;

    fn fmap<C, F>(self, f: F) -> Self::Target<C>
    where
        F: Fn(A) -> C + 'a,
    {
        TestFunctor {
            data: self.data,
            next: Rc::new(move |x| f((self.next)(x))),
        }
    }
}

free!(<'xx>, FreeTest<'xx,'yy,AA,BB>, TestFunctor<'xx, 'yy, FreeTest<'xx, 'yy, AA, BB>, BB>);

#[test]
fn test_lifetime_multiple_generics() {
    let free_monad = FreeTest::lift_f(TestFunctor {
        data: &"Listening to NSP while writing this.",
        next: Rc::new(|x| f64::from(x) * 0.5f64),
    });
    let functions = FreeTest::Pure(|x: f64| -> bool { x > 0.7f64 }).fmap(Into::into);
    let free_monad_after_apply = free_monad.apply(functions);
    match free_monad_after_apply {
        FreeTest::Free(m) => {
            assert_eq!(m.data, &"Listening to NSP while writing this.");
            let x = m.next.clone();
            let y = m.next.clone();
            let m1 = x(1);
            match m1 {
                FreeTest::Pure(v) => assert!(!v),
                FreeTest::Free(_) => unreachable!(),
            }
            let m2 = y(3);
            match m2 {
                FreeTest::Pure(v) => assert!(v),
                FreeTest::Free(_) => unreachable!(),
            }
        }
        FreeTest::Pure(_) => unreachable!(),
    }
}

//just to appease clippy without disabling the lint....
macro_rules! assert_nearly_equal {
    ($a:expr, $b:expr, $c:expr) => {
        assert!((($a) - ($b)).abs() < $c)
    };
}

#[test]
fn test_lifetime_multiple_generics_bind() {
    let m = FreeTest::lift_f(TestFunctor {
        data: &"Listening to Soilwork while writing this.",
        next: Rc::new(|x| f64::from(x) * 0.5f64),
    });
    let m = m.bind(|x: f64| -> FreeTest<_, _> {
        if x < 0.0 {
            FreeTest::Pure(x.abs().floor())
        } else {
            FreeTest::lift_f(TestFunctor {
                data: &"Now it's Little Big.",
                next: Rc::new(move |y| f64::from(y) + x.ceil()),
            })
        }
    });
    match m {
        FreeTest::Free(m) => {
            assert_eq!(m.data, &"Listening to Soilwork while writing this.");
            match (m.next)(-3) {
                FreeTest::Pure(v) => assert_nearly_equal!(v, 1f64, f64::EPSILON),
                FreeTest::Free(_) => unreachable!(),
            }
            match (m.next)(3) {
                FreeTest::Pure(_) => unreachable!(),
                FreeTest::Free(v) => {
                    assert_eq!(v.data, &"Now it's Little Big.");
                    match (v.next)(5) {
                        FreeTest::Pure(v) => assert_nearly_equal!(v, 7f64, f64::EPSILON),
                        FreeTest::Free(_) => unreachable!(),
                    }
                }
            }
        }
        FreeTest::Pure(_) => unreachable!(),
    }
}
