//! A macro that uses the traits from the [higher] crate and generates a Free [`Monad`][higher::Monad] type for a given [`Functor`][higher::Functor].
//! 
//! # Free Monad? What is that?
//! A Free Monad is the left-adjoint to the Forget-Functor from the category of Monads into the category of Endofunctors.
//! 
//! This of course doesn't explain what one can do with a Free Monad data type. There are plenty of blog posts and websites that
//! explore different use cases. A very good explanation is given by Nikolay Yakimov's
//! [Introduction to Free Monads](https://serokell.io/blog/introduction-to-free-monads).
//! 
//! # Why a Macro?
//! Until [non-lifetime binders](https://github.com/rust-lang/rust/issues/108185) become stable, this seems to be the easiest way.
//! In generic code, the type signature would be `enum Free<A,F> where F : Functor<Free<A,F>>`. If one now wants to implement the [`Functor`][higher::Functor]
//! trait for this, it is not really possible to express the `Target<T> = Free<A,F::Target<Free<A,F::Target<...>>>>` generic associated type.
//! 
//! See the [blog post about this crate](https://www.grois.info/posts/2023-03/2023-03-11-adventures-with-free-monads-and-higher.xhtml)
//! for a more detailed explanation.

pub extern crate higher;

#[macro_export]
macro_rules! free {
    ($v:vis $name:ident<$($other_lifetimes:lifetime,)* $generic:ident $(,$other_generics:ident)*>, $f:ty) => {
        #[derive(Clone)]
        $v enum $name<$($other_lifetimes,)* $generic $(,$other_generics)*> {
            Pure($generic),
            Free(Box<$f>)
        }
        impl<$($other_lifetimes,)* $generic $(,$other_generics)*> $name<$($other_lifetimes,)* $generic $(,$other_generics)*>{
            $v fn lift_f(functor : <$f as $crate::higher::Functor<Self>>::Target<$generic>) -> Self{
                use $crate::higher::Functor;
                Self::Free(Box::new(functor.fmap(|a| Self::Pure(a))))
            }

            $v fn retract<'free_macro_reserved_lifetime>(self) -> <$f as $crate::higher::Bind<'free_macro_reserved_lifetime,Self>>::Target<$generic> where $f : $crate::higher::Monad<'free_macro_reserved_lifetime,Self>, <$f as $crate::higher::Bind<'free_macro_reserved_lifetime,Self>>::Target<$generic> : $crate::higher::Pure<$generic> {
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
                    use $crate::higher::Functor;
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
        impl<$($other_lifetimes : $a,)* $generic $(,$other_generics)*> $name<$($other_lifetimes,)* $generic $(,$other_generics)*> where $generic : $a {
            $v fn lift_f(functor : <$f as $crate::higher::Functor<$a, Self>>::Target<$generic>) -> Self{
                use $crate::higher::Functor;
                Self::Free(Box::new(functor.fmap(|a| Self::Pure(a))))
            }

            $v fn retract(self) -> <$f as $crate::higher::Bind<$a,Self>>::Target<$generic> where $f : $crate::higher::Monad<$a,Self>, <$f as $crate::higher::Bind<$a,Self>>::Target<$generic> : $crate::higher::Pure<$generic> {
                use $crate::higher::{Bind, Pure};
                match self {
                    $name::Pure(a) => {<$f as $crate::higher::Bind<$a,Self>>::Target::<$generic>::pure(a)},
                    $name::Free(m) => {m.bind(|a| a.retract())}
                }
            }
        }
    
        impl<$($other_lifetimes : $a,)* $generic $(,$other_generics)*> $crate::higher::Functor<$a,$generic> for $name<$($other_lifetimes,)* $generic $(,$other_generics)*> where $generic : $a {
            type Target<FreeMacroReservedType> = $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*>;
            fn fmap<FreeMacroReservedType,F>(self, f: F) -> Self::Target<FreeMacroReservedType> 
                where F: Fn($generic) -> FreeMacroReservedType + $a
            {
                fn __fmap_impl<$($other_lifetimes : $a,)* $generic $(,$other_generics)*, FreeMacroReservedType, F>(s : $name<$($other_lifetimes,)* $generic $(,$other_generics)*>, f : std::rc::Rc<F>) -> $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*> where $generic : $a, F: Fn($generic) -> FreeMacroReservedType + $a{
                    use $crate::higher::Functor;
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

        impl<$($other_lifetimes : $a,)* $generic $(,$other_generics)*> $crate::higher::Apply<$a, $generic> for $name<$($other_lifetimes,)* $generic $(,$other_generics)*> where $generic: $a + Clone, Self : Clone{
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

        impl<$($other_lifetimes : $a,)* $generic $(,$other_generics)*> $crate::higher::Bind<$a,$generic> for $name<$($other_lifetimes,)* $generic $(,$other_generics)*> where $generic : $a{
            type Target<FreeMacroReservedType> = $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*>;
            fn bind<FreeMacroReservedType, F>(self, f: F) -> Self::Target<FreeMacroReservedType>
            where
                F: Fn($generic) -> Self::Target<FreeMacroReservedType> + $a,
            {
                fn __bind_impl<$($other_lifetimes : $a,)* $generic $(,$other_generics)*, FreeMacroReservedType, F>(s : $name<$($other_lifetimes,)* $generic $(,$other_generics)*>, f : std::rc::Rc<F>) -> $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*> where $generic : $a, F: Fn($generic) -> $name<$($other_lifetimes,)* FreeMacroReservedType $(,$other_generics)*> + $a{
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