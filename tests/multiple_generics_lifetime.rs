//! Tests if multiple generic parameters work, if the return value's lifetime depends on the mapping function lifetime.

use std::rc::Rc;
use higher_free_macro::free;
use higher::{Functor, Bind, Apply};

#[derive(Clone)]
struct TestFunctor<'a, 'b, A, B>{
    data : &'b B,
    next : Rc<dyn Fn(i32)->A + 'a>,
}

impl<'a,'b,A : 'a,B> Functor<'a,A> for TestFunctor<'a, 'b, A, B>{
    type Target<T> = TestFunctor<'a, 'b, T, B>;

    fn fmap<C, F>(self, f: F) -> Self::Target<C>
    where
        F: Fn(A) -> C + 'a {
        TestFunctor{ data : self.data, next : Rc::new(move |x| f((self.next)(x)))}
    }
}

free!(<'xx>, FreeTest<'xx,'yy,AA,BB>, TestFunctor<'xx, 'yy, FreeTest<'xx, 'yy, AA, BB>, BB>);

#[test]
fn test_lifetime_multiple_generics(){
    let m = FreeTest::lift_f(TestFunctor{ data : &"Listening to NSP while writing this.", next : Rc::new(|x| (x as f32)*0.5f32)});
    let f = FreeTest::Pure(|x : f32| -> bool {x > 0.7f32} ).fmap(Into::into);
    let m = m.apply(f);
    match m {
        FreeTest::Free(m) => {
            assert_eq!(m.data, &"Listening to NSP while writing this.");
            let x = m.next.clone();
            let y = m.next.clone();
            let m1 = x(1);
            match m1{
                FreeTest::Pure(v) => assert!(!v),
                FreeTest::Free(_) => unreachable!(),
            }
            let m2 = y(3);
            match m2{
                FreeTest::Pure(v) => assert!(v),
                FreeTest::Free(_) => unreachable!(),
            }
        },
        _ => unreachable!()
    }
}

#[test]
fn test_lifetime_multiple_generics_bind(){
    let m = FreeTest::lift_f(TestFunctor{ data : &"Listening to Soilwork while writing this.", next : Rc::new(|x| (x as f32)*0.5f32)});
    let m = m.bind(|x : f32| -> FreeTest<_,_> {
        if x < 0.0 {
            FreeTest::Pure(x.abs().floor() as u32)
        } else {
            FreeTest::lift_f(TestFunctor{data : &"Now it's Little Big.", next : Rc::new(move |y| (y as u32) + (x.ceil() as u32))})
        }});
    match m{
        FreeTest::Free(m) => {
            assert_eq!(m.data, &"Listening to Soilwork while writing this.");
            match (m.next)(-3){
                FreeTest::Pure(v) => assert_eq!(v, 1),
                FreeTest::Free(_) => unreachable!(),
            }
            match (m.next)(3){
                FreeTest::Pure(_) => unreachable!(),
                FreeTest::Free(v) => {
                    assert_eq!(v.data, &"Now it's Little Big.");
                    match (v.next)(5) {
                        FreeTest::Pure(v) => {
                            assert_eq!(v, 7)
                        },
                        FreeTest::Free(_) => unreachable!(),
                    }
                },
            }
        },
        _ => unreachable!()
    }
}