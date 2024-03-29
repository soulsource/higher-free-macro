#![deny(clippy::pedantic)]
#![deny(clippy::all)]
//! A macro that uses the traits from the [higher] crate and generates a Free [`Monad`][higher::Monad] type for a given [`Functor`][higher::Functor].
//!
//! This is a port of the Control.Monad.Free part of the ["free" Haskell package](https://hackage.haskell.org/package/free) by Edward Kmett.
//!
//! # What is a Free Monad?
//! A Free Monad is the left-adjoint to the Forget-Functor from the category of Monads into the category of Endofunctors.
//!
//! From a programmer's perspective, however, it is a nifty way to create a [`Monad`][higher::Monad], that is "based" on a given [`Functor`][higher::Functor]
//! and does not impose any additional structure beyond the [Monad Laws](https://wiki.haskell.org/Monad_laws).
//!
//! The structure of the Free [`Monad`][higher::Monad] is defined by the underlying [`Functor`][higher::Functor].
//! For instance, if the underlying [`Functor`][higher::Functor] is a [`Vec`], the corresponding Free [`Monad`][higher::Monad] will be a linked tree.
//! If the underlying [`Functor`][higher::Functor] is an [`Option`], the corresponding Free [`Monad`][higher::Monad] is a linked list.
//! And so on, and so forth.
//!
//! There are many use cases for such a data structure, the most well known one is the creation of embedded
//! [Domain Specific Languages](https://en.wikipedia.org/wiki/Domain-specific_language) (eDSLs).
//! Going into detail would go beyond the scope of this documentation, however. Please check out Nikolay Yakimov's
//! [Introduction to Free Monads](https://serokell.io/blog/introduction-to-free-monads) for that.
//!
//! There is also a [blog post about the development of this macro](https://www.grois.info/posts/2023-03/2023-03-11-adventures-with-free-monads-and-higher.xhtml),
//! that presents a simple (but inexact) mental picture
//! (by means of actual [pictures](https://www.grois.info/posts/2023-03/2023-03-11-adventures-with-free-monads-and-higher.xhtml#ugly_drawings))
//! of how the different [`Monad`][higher::Monad] operations (bind, fmap, pure, apply) work on Free Monads.
//!
//! # How to use the macro?
//!
//! For details, please see the documentation of the [free] macro.
//! In short, the syntax is either  
//! `free!(FreeMonadTypeName<'a,A>, FunctorItsBasedOn<FreeMonadTypeName<'a,A>>)`,  
//! or, if the lifetime of the Free Monad depends on the lifetime of the function passed to the Functor's fmap function,  
//! `free!(<'a>, FreeMonadTypeName<'a,A>, FunctorItsBasedOn<'a,FreeMonadTypeName<'a,A>>)`,  
//! where `'a` is the affected lifetime.
//!
//! # Examples
//! The project's repository contains a folder named "examples", which at the moment contains a tiny text adventure that shows how such a game
//! could be implemented with Free Monads. The example highlights both, features and (current) limitations of Free Monads in Rust.
//!
//! In addition, there is the "tests" folder, which contains integration tests, that show more of the syntax of the `free!()` macro in action.
//!
//! # Why a Macro?
//! Until [non-lifetime binders](https://github.com/rust-lang/rust/issues/108185) become stable, this seems to be the easiest way.
//! In generic code, the type signature would be  
//! `enum Free<A,F> where F : Functor<Free<A,F>>`.  
//! If one now wants to implement the [`Functor`][higher::Functor] trait for this, it is not really possible to express the  
//! `Target<T> = Free<A,F::Target<Free<A,F::Target<...>>>>`  
//! generic associated type.
//!
//! See the [blog post about this crate](https://www.grois.info/posts/2023-03/2023-03-11-adventures-with-free-monads-and-higher.xhtml)
//! for a more detailed explanation.
//!
//! # A word of warning:
//! This crate should be considered a proof-of-concept. Its memory complexity is horrendous, and the performance of the Free Monad's [`Apply`][higher::Apply]
//! implementation can only be described as abysmal due to its reliance on deep copies.
//! In addition, the desugaring of do-notation currently (with higher-0.2) only works well with return types that are [`Copy`].
//! If those types are big, that might be a further performance bottleneck.
//! There is work ongoing to [add explicit clone support to higher](https://github.com/bodil/higher/issues/6) though, so this might no longer be an issue with
//! later higher versions.

#[doc(hidden)]
//that this is re-exported is an implementation detail. Users should import directly from higher imho.
pub extern crate higher;

/// The macro that generates a Free [`Monad`][higher::Monad] type for a given [`Functor`][higher::Functor].
///
/// To declare a Free [`Monad`][higher::Monad] over a [`Functor`][higher::Functor] named `Funky<A>`, the syntax would be  
/// `free!(FreeFunky<A>, Funky<FreeFunky<A>>)`.  
/// This declares an enum named `FreeFunky<A>`, and implements all traits needed for it to be a [`Monad`][higher::Monad].
///
/// # Restrictions
/// It is currently not supported to create a Free Monad for a Functor that does not implement [`Clone`]. This is because it is in general not
/// possible to implement [`Apply`][higher::Apply] for a non-cloneable Free Monad, and because of how Rust resolves trait bounds in recursive types.
///
/// In addition, for the result to actually be a [`Monad`][higher::Monad], the `Pure` type (the type the Free Monad is generic over) needs to support [`Clone`]
/// too. This is again because of the requirement of [`Apply`][higher::Apply], which in turn is a requirement of [`Monad`][higher::Monad]. However,
/// it is typically not necessary to have a fully fledged [`Monad`][higher::Monad]. In most use cases, it's enough to have
/// [`Functor`][higher::Functor] + [`Bind`][higher::Bind] + [`Pure`][higher::Pure].
///
/// The Free Monad type is implemented recursively. It is therefore akin to a linked tree, with all the respective performance implications.
///
/// Furthermore, the implementation of [`Apply`][higher::Apply] creates a potentially high number of deep copies of the `self` parameter.
/// It should therefore be avoided, unless one really needs its
/// [tree-merging behaviour](https://www.grois.info/posts/2023-03/2023-03-11-adventures-with-free-monads-and-higher.xhtml#ugly_apply_drawing).
///
/// # Usage
/// As stated above, the syntax to create a Free Monad is usually to call the macro with the desired Free Monad type as first,
/// and the [`Functor`][higher::Functor] it should be based on as second parameter.
///
/// For example, a Free Monad based on [`Option`] could simply be created like this:
/// ```
/// # #[macro_use] extern crate higher_free_macro;
/// # use higher_free_macro::higher::*;
/// free!(FreeOption<A>, Option<FreeOption<A>>);
/// ```
///
/// The type created by this is indeed a Monad, as long as the wrapped type is [`Clone`]:
/// ```
/// # #[macro_use] extern crate higher_free_macro;
/// # use higher_free_macro::higher::*;
/// free!(FreeOption<A>, Option<FreeOption<A>>);
///
/// fn returns_a_monad<'a, A>(a : A) -> impl Monad<'a,A> where A : Clone + 'a {
///     FreeOption::Pure(a)
/// }
/// ```
/// Since, strictly speaking, [`Apply`][higher::Apply] is not required to express the properties of a Monad (the mathematical structure, not the trait),
/// one might want to skip the requirement of [`Clone`]. The result is still [`Bind`][higher::Bind], [`Functor`][higher::Functor] and [`Pure`][higher::Pure],
/// so in the mathematical sense a Monad:
/// ```
/// # #[macro_use] extern crate higher_free_macro;
/// # use higher_free_macro::higher::*;
/// free!(FreeOption<A>, Option<FreeOption<A>>);
///
/// fn returns_a_bind_pure_functor<'a, A>(a : A) -> impl Bind<'a,A> + Pure<A> + Functor<'a,A>
///     where A : 'a
/// {
///     FreeOption::Pure(a)
/// }
/// ```
///
/// That said, the macro also supports multiple generic parameters. The parameter for which the traits will be implemented is the first generic parameter
/// of the to-be-created Free Monad type. For instance, a Free Monad based on [`Result`] would be:
/// ```
/// # #[macro_use] extern crate higher_free_macro;
/// # use higher_free_macro::higher::*;
/// free!(FreeResult<A,E>, Result<FreeResult<A,E>,E>);
///
/// fn returns_a_monad<'a, A, E>(r : Result<A,E>) -> impl Monad<'a,A>
///     where A : Clone + 'a, E : Clone
/// {
///     FreeResult::lift_f(r)
/// }
/// ```
///
/// Furthermore, the use case that the lifetime of the Free Monad depends on the lifetime of the mapping functions is supported too.
/// This is particularly useful, because it enables the usage of (non-constant) continuation functions, what is a requirement for
/// using the Free Monad for an embedded Domain Specific Language (eDSL).
///
/// Such a [`Functor`][higher::Functor] could for instance look like this:
/// ```
/// # #[macro_use] extern crate higher_free_macro;
/// # use higher_free_macro::higher::*;
/// #[derive(Clone)]
/// struct FunctorWithCont<'a, A>(std::rc::Rc<dyn Fn(i32)->A + 'a>);
/// impl<'a,A : 'a> Functor<'a,A> for FunctorWithCont<'a, A>{
///     type Target<T> = FunctorWithCont<'a, T>;
///     fn fmap<B,F>(self, f :F) -> Self::Target<B> where F : Fn(A)->B + 'a{
///         FunctorWithCont(std::rc::Rc::new(move |x| f((self.0)(x))))
///     }
/// }
/// ```
///
/// Sadly, the macro syntax is a bit more convoluted in this case. The relevant lifetime has to be stated explicitly as the first parameter, like this:
/// ```
/// # #[macro_use] extern crate higher_free_macro;
/// # use higher_free_macro::higher::*;
/// free!(<'a>, FreeFunctorWithCont<'a,A>, FunctorWithCont<'a,FreeFunctorWithCont<'a,A>>);
/// # #[derive(Clone)]
/// # struct FunctorWithCont<'a, A>(std::rc::Rc<dyn Fn(i32)->A + 'a>);
/// # impl<'a,A : 'a> Functor<'a,A> for FunctorWithCont<'a, A>{
/// #     type Target<T> = FunctorWithCont<'a, T>;
/// #     fn fmap<B,F>(self, f :F) -> Self::Target<B> where F : Fn(A)->B + 'a{
/// #         FunctorWithCont(std::rc::Rc::new(move |x| f((self.0)(x))))
/// #     }
/// # }
/// ```
///
/// # Generated Functions
/// In addition to the trait implementations for [`Bind`][higher::Bind], [`Functor`][higher::Functor], [`Apply`][higher::Apply] and [`Pure`][higher::Pure],
/// the macro also generates associated functions for the Free Monad type. These functions are:  
/// `fn lift_f(functor : F) -> Self`  
/// `fn retract(self)-> F where F : Bind + Pure`  
/// where `F` is the [`Functor`][higher::Functor] the Free Monad is based on, specialized for the `Pure` type.
/// A concrete example will make this more clear. Let's take our `FreeOption<A>` example from above. In this case, the signatures are  
/// `fn lift_f(functor : Option<A>) -> FreeOption<A>` and  
/// `fn retract(self : FreeOption<A>) -> Option<A>`
///
/// `lift_f()` converts a base Functor into the corresponding Free Monad, meaning that the Functor gets wrapped in `Free`, and the values it holds get
/// mapped into `Pure`. The (simplified for readability) formula is:  
/// `Self::Free(functor.fmap(|a| Self::Pure(a)))`
///
/// `retract()` is the left-inverse of `lift_f()`. `|x| retract(lift_f(x))` is (ignoring type coercion) equivalent to [`identity`][std::convert::identity]:
/// ```
/// # #[macro_use] extern crate higher_free_macro;
/// # use higher_free_macro::higher::*;
/// free!(FreeOption<A>, Option<FreeOption<A>>);
/// fn main() {
///     let free_monad = FreeOption::lift_f(Some(12345u32));
///     match &free_monad {
///         FreeOption::Free(o) => {
///             match &**o {
///                 Some(p) => {
///                     match p {
///                         FreeOption::Pure(v) => assert_eq!(v, &12345u32),
///                         FreeOption::Free(_) => unreachable!()
///                     }
///                 },
///                 None => unreachable!()
///             }
///         },
///         FreeOption::Pure(_) => unreachable!()
///     }
///     let and_back = free_monad.retract();
///     assert_eq!(and_back, Some(12345u32));
/// }
/// ```
#[macro_export]
macro_rules! free {
    ($v:vis $name:ident<$($other_lifetimes:lifetime,)* $generic:ident $(,$other_generics:ident)*>, $f:ty) => {
        #[derive(Clone)]
        $v enum $name<$($other_lifetimes,)* $generic $(,$other_generics)*> {
            Pure($generic),
            Free(Box<$f>)
        }
        impl<$($other_lifetimes,)* $generic $(,$other_generics)*> $name<$($other_lifetimes,)* $generic $(,$other_generics)*>{
            #[allow(unused)]
            $v fn lift_f(functor : <$f as $crate::higher::Functor<Self>>::Target<$generic>) -> Self{
                use $crate::higher::Functor;
                Self::Free(Box::new(functor.fmap(Self::Pure)))
            }

            #[allow(unused)]
            $v fn retract<'free_macro_reserved_lifetime>(self) -> <$f as $crate::higher::Bind<'free_macro_reserved_lifetime,Self>>::Target<$generic> where $f : $crate::higher::Bind<'free_macro_reserved_lifetime,Self>, <$f as $crate::higher::Bind<'free_macro_reserved_lifetime,Self>>::Target<$generic> : $crate::higher::Pure<$generic> {
                use $crate::higher::{Bind, Pure};
                match self {
                    $name::Pure(a) => {<$f as $crate::higher::Bind<'free_macro_reserved_lifetime,Self>>::Target::<$generic>::pure(a)},
                    $name::Free(m) => {m.bind(|a| a.retract())}
                }
            }
        }

        impl<'free_macro_reserved_lifetime, $($other_lifetimes,)* $generic $(,$other_generics)*> $crate::higher::Functor<'free_macro_reserved_lifetime,$generic> for $name<$($other_lifetimes,)* $generic $(,$other_generics)*> {
            type Target<FreeMacroReservedType> = $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*>;
            fn fmap<FreeMacroReservedType,FreeMacroReservedType2>(self, f: FreeMacroReservedType2) -> Self::Target<FreeMacroReservedType> where FreeMacroReservedType2: Fn($generic) -> FreeMacroReservedType + 'free_macro_reserved_lifetime{
                fn __fmap_impl<'free_macro_reserved_lifetime, $($other_lifetimes,)* $generic $(,$other_generics)*, FreeMacroReservedType, FreeMacroReservedType2>(s : $name<$($other_lifetimes,)* $generic $(,$other_generics)*>, f: &FreeMacroReservedType2) -> $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*> where FreeMacroReservedType2: Fn($generic) -> FreeMacroReservedType + 'free_macro_reserved_lifetime{
                    match s {
                        $name::Pure(a) => {$name::Pure(f(a))},
                        $name::Free(fa) => {$name::Free(Box::new(fa.fmap(|x| __fmap_impl(x, f))))},
                    }
                }
                __fmap_impl(self, &f)
            }
        }

        impl<$($other_lifetimes,)* $generic $(,$other_generics)*> $crate::higher::Pure<$generic> for $name<$($other_lifetimes,)* $generic $(,$other_generics)*> {
            fn pure(value : $generic) -> Self {
                Self::Pure(value)
            }
        }

        impl<'free_macro_reserved_lifetime, $($other_lifetimes,)* $generic $(,$other_generics)*> $crate::higher::Apply<'free_macro_reserved_lifetime, $generic> for $name<$($other_lifetimes,)* $generic $(,$other_generics)*> where $generic: 'free_macro_reserved_lifetime + Clone, Self : Clone {
            type Target<FreeMacroReservedType> = $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*> where FreeMacroReservedType:'free_macro_reserved_lifetime;
            fn apply<FreeMacroReservedType>(
                self,
                f: <Self as $crate::higher::Apply<'free_macro_reserved_lifetime, $generic>>::Target<$crate::higher::apply::ApplyFn<'free_macro_reserved_lifetime, $generic, FreeMacroReservedType>>,
            ) -> <Self as $crate::higher::Apply<'free_macro_reserved_lifetime, $generic>>::Target<FreeMacroReservedType>
            where
                FreeMacroReservedType: 'free_macro_reserved_lifetime,
            {
                $crate::higher::apply::ap(f,self)
            }
        }

        impl<'free_macro_reserved_lifetime, $($other_lifetimes,)* $generic $(,$other_generics)*> $crate::higher::Bind<'free_macro_reserved_lifetime,$generic> for $name<$($other_lifetimes,)* $generic $(,$other_generics)*>{
            type Target<FreeMacroReservedType> = $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*>;
            fn bind<FreeMacroReservedType, FreeMacroReservedType2>(self, f: FreeMacroReservedType2) -> Self::Target<FreeMacroReservedType>
            where
                FreeMacroReservedType2: Fn($generic) -> Self::Target<FreeMacroReservedType>,
            {
                fn __bind_impl<'free_macro_reserved_lifetime, $($other_lifetimes,)* $generic $(,$other_generics)*, FreeMacroReservedType, FreeMacroReservedType2>(s : $name<$($other_lifetimes,)* $generic $(,$other_generics)*>, f: &FreeMacroReservedType2) -> $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*> where FreeMacroReservedType2: Fn($generic) -> $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*> + 'free_macro_reserved_lifetime{
                    use $crate::higher::Functor;
                    match s {
                        $name::Pure(a) => {f(a)},
                        $name::Free(fa) => {$name::Free(Box::new(fa.fmap(|x| __bind_impl(x, f))))},
                    }
                }
                __bind_impl(self, &f)
            }
        }
    };
    (<$a:lifetime>, $v:vis $name:ident<$($other_lifetimes:lifetime,)+ $generic:ident $(,$other_generics:ident)*>, $f:ty) =>{
        #[derive(Clone)]
        $v enum $name<$($other_lifetimes,)* $generic $(,$other_generics)*> {
            Pure($generic),
            Free(Box<$f>)
        }
        impl<$($other_lifetimes : $a,)* $generic $(,$other_generics)*> $name<$($other_lifetimes,)* $generic $(,$other_generics)*> where $generic : $a $(,$other_generics : $a)*{
            #[allow(unused)]
            $v fn lift_f(functor : <$f as $crate::higher::Functor<$a, Self>>::Target<$generic>) -> Self{
                use $crate::higher::Functor;
                Self::Free(Box::new(functor.fmap(Self::Pure)))
            }

            #[allow(unused)]
            $v fn retract(self) -> <$f as $crate::higher::Bind<$a,Self>>::Target<$generic> where $f : $crate::higher::Bind<$a,Self>, <$f as $crate::higher::Bind<$a,Self>>::Target<$generic> : $crate::higher::Pure<$generic> {
                use $crate::higher::{Bind, Pure};
                match self {
                    $name::Pure(a) => {<$f as $crate::higher::Bind<$a,Self>>::Target::<$generic>::pure(a)},
                    $name::Free(m) => {m.bind(Self::retract)}
                }
            }
        }

        impl<$($other_lifetimes : $a,)* $generic $(,$other_generics)*> $crate::higher::Functor<$a,$generic> for $name<$($other_lifetimes,)* $generic $(,$other_generics)*> where $generic : $a $(,$other_generics : $a)* {
            type Target<FreeMacroReservedType> = $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*>;
            fn fmap<FreeMacroReservedType,F>(self, f: F) -> Self::Target<FreeMacroReservedType>
                where F: Fn($generic) -> FreeMacroReservedType + $a
            {
                fn __fmap_impl<$($other_lifetimes : $a,)* $generic $(,$other_generics)*, FreeMacroReservedType, F>(s : $name<$($other_lifetimes,)* $generic $(,$other_generics)*>, f : std::rc::Rc<F>) -> $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*> where $generic : $a $(,$other_generics : $a)*, F: Fn($generic) -> FreeMacroReservedType + $a{
                    match s {
                        $name::Pure(a) => {$name::Pure(f(a))},
                        $name::Free(fa) => {$name::Free(Box::new(fa.fmap(move |x : $name<$($other_lifetimes,)* $generic $(,$other_generics)*>| __fmap_impl(x, f.clone()))))},
                    }
                }

                let r = std::rc::Rc::new(f);
                __fmap_impl(self, r)
            }
        }

        impl<$($other_lifetimes,)* $generic $(,$other_generics)*> $crate::higher::Pure<$generic> for $name<$($other_lifetimes,)* $generic $(,$other_generics)*> {
            fn pure(value : $generic) -> Self {
                Self::Pure(value)
            }
        }

        impl<$($other_lifetimes : $a,)* $generic $(,$other_generics)*> $crate::higher::Apply<$a, $generic> for $name<$($other_lifetimes,)* $generic $(,$other_generics)*> where $generic: $a + Clone $(,$other_generics : $a + Clone)*, Self : Clone{
            type Target<FreeMacroReservedType> = $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*> where FreeMacroReservedType:$a;
            fn apply<FreeMacroReservedType>(
                self,
                f: <Self as $crate::higher::Apply<$a, $generic>>::Target<$crate::higher::apply::ApplyFn<$a, $generic, FreeMacroReservedType>>,
            ) -> <Self as $crate::higher::Apply<$a, $generic>>::Target<FreeMacroReservedType>
            where
            FreeMacroReservedType: $a,
            {
                $crate::higher::apply::ap(f,self)
            }
        }

        impl<$($other_lifetimes : $a,)* $generic $(,$other_generics)*> $crate::higher::Bind<$a,$generic> for $name<$($other_lifetimes,)* $generic $(,$other_generics)*> where $generic : $a $(,$other_generics : $a)*{
            type Target<FreeMacroReservedType> = $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*>;
            fn bind<FreeMacroReservedType, F>(self, f: F) -> Self::Target<FreeMacroReservedType>
            where
                F: Fn($generic) -> Self::Target<FreeMacroReservedType> + $a,
            {
                fn __bind_impl<$($other_lifetimes : $a,)* $generic $(,$other_generics)*, FreeMacroReservedType, F>(s : $name<$($other_lifetimes,)* $generic $(,$other_generics)*>, f : std::rc::Rc<F>) -> $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*> where $generic : $a $(,$other_generics : $a)*, F: Fn($generic) -> $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*> + $a{
                    use $crate::higher::Functor;
                    match s {
                        $name::Pure(a) => {f(a)},
                        $name::Free(fa) => {$name::Free(Box::new(fa.fmap(move |x : $name<$($other_lifetimes,)* $generic $(,$other_generics)*>| __bind_impl(x, f.clone()))))},
                    }
                }
                let r = std::rc::Rc::new(f);
                __bind_impl(self, r)
            }
        }
    };
}

#[cfg(test)]
mod free_monad_tests {
    use higher::{apply::ApplyFn, Apply, Bind, Functor, Pure};

    use super::free;

    free!(FreeVec<A>, Vec<FreeVec<A>>);

    #[test]
    fn test_lift_f_no_lifetime() {
        let free = FreeVec::lift_f(vec![1, 2, 3]);
        match free {
            FreeVec::Free(vector) => match &**vector {
                [FreeVec::Pure(a), FreeVec::Pure(b), FreeVec::Pure(c)] => {
                    assert_eq!(vec![*a, *b, *c], vec![1, 2, 3]);
                }
                _ => unreachable!(),
            },
            FreeVec::Pure(_) => unreachable!(),
        }
    }

    #[test]
    fn test_retract_no_lifetime() {
        let f = FreeVec::lift_f(vec![1, 2, 3]);
        let v = f.retract();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_pure_no_lifetime() {
        let f = FreeVec::pure(3);
        match f {
            FreeVec::Pure(v) => assert_eq!(v, 3),
            FreeVec::Free(_) => unreachable!(),
        }
    }

    #[test]
    fn test_fmap_no_lifetime() {
        let f = FreeVec::lift_f(vec![1, 2, 3]);
        let f = f.fmap(|x| (f64::from(x)) / 2.0);
        match f {
            FreeVec::Free(f) => match &**f {
                [FreeVec::Pure(a), FreeVec::Pure(b), FreeVec::Pure(c)] => {
                    assert_eq!(vec![0.5f64, 1f64, 1.5f64], vec![*a, *b, *c]);
                }
                _ => unreachable!(),
            },
            FreeVec::Pure(_) => unreachable!(),
        }
    }

    //just to appease clippy without disabling the lint....
    macro_rules! assert_nearly_equal {
        ($a:expr, $b:expr, $c:expr) => {
            assert!((($a) - ($b)).abs() < $c)
        };
    }

    #[test]
    fn test_bind_no_lifetime() {
        let f = FreeVec::lift_f(vec![1, 2]);
        let f = f.bind(|x| {
            if x % 2 == 0 {
                FreeVec::lift_f(vec![f64::from(x), f64::from(x) + 1.0f64])
            } else {
                FreeVec::Pure(f64::from(x))
            }
        });
        match f {
            FreeVec::Free(f) => match &**f {
                [FreeVec::Pure(a), FreeVec::Free(b)] => {
                    assert_nearly_equal!(*a, 1.0f64, f64::EPSILON);
                    match &***b {
                        [FreeVec::Pure(a), FreeVec::Pure(b)] => {
                            assert_nearly_equal!(*a, 2.0f64, f64::EPSILON);
                            assert_nearly_equal!(*b, 3.0f64, f64::EPSILON);
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            },
            FreeVec::Pure(_) => unreachable!(),
        }
    }

    #[test]
    fn test_apply_no_lifetime() {
        let functions = FreeVec::Free(Box::new(vec![
            FreeVec::Free(Box::new(vec![
                FreeVec::Pure((|x| i64::from(x) * 2) as fn(u32) -> i64),
                FreeVec::Pure((|x| i64::from(x) + 2) as fn(u32) -> i64),
            ])),
            FreeVec::Pure((|x| i64::from(x) - 5) as fn(u32) -> i64),
        ]));
        let free_monad = FreeVec::Free(Box::new(vec![
            FreeVec::Pure(5u32),
            FreeVec::Free(Box::new(vec![FreeVec::Pure(6u32), FreeVec::Pure(7u32)])),
        ]));
        let free_monad = free_monad.apply(functions.fmap(Into::into));
        //what have I gotten myself into...
        //at least the mapped sub-trees are all identical in shape to m, so, they can be tested by the same test function...
        let check_mlike_structure = |free_monad: &FreeVec<_>, pure1, pure2, pure3| match free_monad
        {
            FreeVec::Free(vector) => match &***vector {
                [FreeVec::Pure(left), FreeVec::Free(right)] => {
                    assert_eq!(*left, pure1);
                    match &***right {
                        [FreeVec::Pure(left), FreeVec::Pure(right)] => {
                            assert_eq!(*left, pure2);
                            assert_eq!(*right, pure3);
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            },
            FreeVec::Pure(_) => unreachable!(),
        };
        //now, where are those sub-trees exactly, in this monstrosity?
        match free_monad {
            FreeVec::Free(vector) => match &**vector {
                [FreeVec::Free(left), right] => {
                    match &***left {
                        [a, b] => {
                            check_mlike_structure(a, 10i64, 12i64, 14i64);
                            check_mlike_structure(b, 7i64, 8i64, 9i64);
                        }
                        _ => unreachable!(),
                    }
                    check_mlike_structure(right, 0i64, 1i64, 2i64);
                }
                _ => unreachable!(),
            },
            FreeVec::Pure(_) => unreachable!(),
        }
    }

    //and the same for the with-dependent-lifetime case.

    use std::rc::Rc;

    #[derive(Clone)]
    struct Conti<'a, A, B>(Rc<dyn Fn(B) -> A + 'a>, Rc<dyn Fn(B) -> A + 'a>); //two fields, to make apply testable.
    impl<'a, A: 'a, B: 'a> Functor<'a, A> for Conti<'a, A, B> {
        type Target<T> = Conti<'a, T, B>;

        fn fmap<C, F>(self, f: F) -> Self::Target<C>
        where
            F: Fn(A) -> C + 'a,
        {
            let f = Rc::new(f);
            let g = f.clone();
            Conti(
                Rc::new(move |x| f((self.0)(x))),
                Rc::new(move |x| g((self.1)(x))),
            )
        }
    }

    //need Bind and Pure to test retract. This is dumb, but it should fulfill the monad laws:
    impl<'a, A: 'a, B: Clone + 'a> Bind<'a, A> for Conti<'a, A, B> {
        type Target<T> = Conti<'a, T, B>;

        fn bind<C, F>(self, f: F) -> Self::Target<C>
        where
            F: Fn(A) -> Self::Target<C> + 'a,
        {
            let f = Rc::new(f);
            let g = f.clone();
            let l = move |x| f((self.0)(x));
            let r = move |x| g((self.1)(x));
            Conti(
                Rc::new(move |x| (l(x.clone())).0(x)),
                Rc::new(move |x| (r(x.clone())).1(x)),
            )
        }
    }
    impl<'a, A: Clone + 'a, B: 'a> Pure<A> for Conti<'a, A, B> {
        fn pure(value: A) -> Self {
            let v2 = value.clone();
            Conti(
                Rc::new(move |_| value.clone()),
                Rc::new(move |_| v2.clone()),
            )
        }
    }

    //I really am not certain if the Pure and Bind above are correct. Sooo, why not test those too, while we are at it?
    #[test]
    fn test_conti_monad_laws() {
        let binder = |x: u32| {
            let y = x;
            Conti::<u32, u32>(Rc::new(move |a| x + a * 2), Rc::new(move |a| y * a + 3))
        };
        let test1 = Conti::pure(7u32);
        let v1 = test1.bind(binder);
        let v2 = binder(7u32);
        assert_eq!((v1.0)(13), (v2.0)(13));
        assert_eq!((v1.1)(17), (v2.1)(17));

        let test2 = Conti(Rc::new(|a| 31 + a * 5), Rc::new(|b| 32 * b + 3));
        let bound_with_pure = test2.clone().bind(Conti::pure);
        assert_eq!((test2.0)(3), (bound_with_pure.0)(3));
        assert_eq!((test2.1)(5), (bound_with_pure.1)(5));

        let test3 = Conti(Rc::new(|a| 32 + (a * 2)), Rc::new(|b| 32 * b + 7));
        let g =
            |x: u32| Conti::<u32, u32>(Rc::new(move |a| x * 2 + a), Rc::new(move |a| x * a + 7));
        let h = |x: u32| Conti::<u32, u32>(Rc::new(move |a| x + a), Rc::new(move |a| x * a + 12));

        let v1 = (test3.clone().bind(g)).bind(h);
        let v2 = test3.bind(|a| (g(a).bind(h)));
        assert_eq!((v1.0)(37), (v2.0)(37));
        assert_eq!((v1.1)(41), (v2.1)(41));

        //well, looks monadic enough to me. Let's use it for the unit test of retract below.
    }

    free!(<'a>, FreeConti<'a,A,B>, Conti<'a,FreeConti<'a,A,B>,B>);

    #[test]
    fn test_lift_f_lifetime() {
        let f = FreeConti::lift_f(Conti(
            Rc::new((|x| x * 2) as fn(u32) -> u32),
            Rc::new((|x| x + 5) as fn(u32) -> u32),
        ));
        match f {
            FreeConti::Free(m) => {
                match (m.0)(4) {
                    FreeConti::Pure(v) => assert_eq!(v, 8),
                    FreeConti::Free(_) => unreachable!(),
                }
                match (m.1)(4) {
                    FreeConti::Pure(v) => assert_eq!(v, 9),
                    FreeConti::Free(_) => unreachable!(),
                }
            }
            FreeConti::Pure(_) => unreachable!(),
        }
    }

    #[test]
    fn test_retract_lifetime() {
        let f = FreeConti::lift_f(Conti(
            Rc::new((|x| x * 2) as fn(u32) -> u32),
            Rc::new((|x| x + 5) as fn(u32) -> u32),
        ));
        let r = f.retract();
        assert_eq!((r.0)(4), 8);
        assert_eq!((r.1)(4), 9);
    }

    #[test]
    fn test_fmap_lifetime() {
        let functor = Conti(
            Rc::new(|x: u32| i32::try_from(x).unwrap() * 3 + 2),
            Rc::new(|x| (i32::try_from(x).unwrap() + 2) * 5),
        );
        let free_monad = FreeConti::lift_f(functor);
        let free_monad = free_monad.fmap(|x: i32| f64::from(x) * 0.25f64);
        match free_monad {
            FreeConti::Free(f) => {
                let left = (f.0)(7);
                match left {
                    FreeConti::Pure(v) => {
                        assert_nearly_equal!(v, 5.75f64, f64::EPSILON);
                    }
                    FreeConti::Free(_) => unreachable!(),
                }
                let right = (f.1)(7);
                match right {
                    FreeConti::Pure(v) => {
                        assert_nearly_equal!(v, 11.25f64, f64::EPSILON);
                    }
                    FreeConti::Free(_) => unreachable!(),
                }
            }
            FreeConti::Pure(_) => unreachable!(),
        }
    }

    #[test]
    fn test_pure_lifetime() {
        let f: FreeConti<_, ()> = FreeConti::pure(27);
        match f {
            FreeConti::Pure(v) => assert_eq!(v, 27),
            FreeConti::Free(_) => unreachable!(),
        }
    }

    #[test]
    fn test_bind_lifetime() {
        let functor = Conti(
            Rc::new(|x: u32| i32::try_from(x).unwrap() * 3 + 2),
            Rc::new(|x| (i32::try_from(x).unwrap() + 2) * 5),
        );
        let free_monad = FreeConti::lift_f(functor);
        let free_monad = free_monad.bind(|y| {
            let z = y;
            FreeConti::lift_f(Conti(
                Rc::new(move |x| f64::from(x) * 0.25f64 + f64::from(y)),
                Rc::new(move |x| f64::from(x) * 0.5f64 - f64::from(z)),
            ))
        });
        match free_monad {
            FreeConti::Free(f) => {
                let left = (f.0)(4);
                match left {
                    FreeConti::Free(f) => {
                        //14i32
                        let left = (f.0)(5);
                        match left {
                            FreeConti::Pure(v) => assert_nearly_equal!(v, 15.25f64, f64::EPSILON),
                            FreeConti::Free(_) => unreachable!(),
                        }
                        let right = (f.1)(5);
                        match right {
                            FreeConti::Pure(v) => assert_nearly_equal!(v, -11.5f64, f64::EPSILON),
                            FreeConti::Free(_) => unreachable!(),
                        }
                    }
                    FreeConti::Pure(_) => unreachable!(),
                }
                let right = (f.1)(4);
                match right {
                    FreeConti::Free(f) => {
                        //30i32
                        let left = (f.0)(5);
                        match left {
                            FreeConti::Pure(v) => assert_nearly_equal!(v, 31.25f64, f64::EPSILON),
                            FreeConti::Free(_) => unreachable!(),
                        }
                        let right = (f.1)(5);
                        match right {
                            FreeConti::Pure(v) => assert_nearly_equal!(v, -27.5f64, f64::EPSILON),
                            FreeConti::Free(_) => unreachable!(),
                        }
                    }
                    FreeConti::Pure(_) => unreachable!(),
                }
            }
            FreeConti::Pure(_) => unreachable!(),
        }
    }

    #[test]
    fn test_apply_lifetime() {
        //oh, god, please no.
        let free_monad_input = FreeConti::lift_f(Conti(
            Rc::new(|x: u32| i32::try_from(x).unwrap() * 3 + 2),
            Rc::new(|x| (i32::try_from(x).unwrap() + 2) * 5),
        ));
        let functions = FreeConti::lift_f(Conti(
            Rc::new(|x: u32| -> ApplyFn<i32, f64> {
                (move |y: i32| f64::from(y + i32::try_from(x).unwrap())).into()
            }),
            Rc::new(|x: u32| (move |y: i32| f64::from(y * i32::try_from(x).unwrap())).into()),
        ));
        //make it stop!
        let free_monad = free_monad_input.apply(functions);

        match free_monad {
            FreeConti::Free(m) => {
                let left = (m.0)(5u32);
                match left {
                    FreeConti::Free(m) => {
                        let left = (m.0)(7u32);
                        match left {
                            FreeConti::Pure(v) => assert_nearly_equal!(v, 28f64, f64::EPSILON),
                            FreeConti::Free(_) => unreachable!(),
                        }
                        let right = (m.1)(7u32);
                        match right {
                            FreeConti::Pure(v) => assert_nearly_equal!(v, 50f64, f64::EPSILON),
                            FreeConti::Free(_) => unreachable!(),
                        }
                    }
                    FreeConti::Pure(_) => unreachable!(),
                }
                let right = (m.1)(5u32);
                match right {
                    FreeConti::Free(m) => {
                        let left = (m.0)(7u32);
                        match left {
                            FreeConti::Pure(v) => {
                                assert_nearly_equal!(v, 5f64 * 23f64, f64::EPSILON);
                            }
                            FreeConti::Free(_) => unreachable!(),
                        }
                        let right = (m.1)(7u32);
                        match right {
                            FreeConti::Pure(v) => {
                                assert_nearly_equal!(v, 5f64 * 45f64, f64::EPSILON);
                            }
                            FreeConti::Free(_) => unreachable!(),
                        }
                    }
                    FreeConti::Pure(_) => unreachable!(),
                }
            }
            FreeConti::Pure(_) => unreachable!(),
        }
        //let's never speak of this again.
    }
}
