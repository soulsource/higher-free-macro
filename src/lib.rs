pub extern crate higher;

#[macro_export]
macro_rules! free {
    ($v:vis $name:ident<$($other_lifetimes:lifetime,)* $generic:ident>, $f:ty) => {
        #[derive(Clone)]
        $v enum $name<$($other_lifetimes,)* $generic> {
            Pure($generic),
            Free(Box<$f>)
        }
        impl<$($other_lifetimes,)* $generic> $name<$($other_lifetimes,)* $generic>{
            $v fn lift_f(command : <$f as $crate::higher::Functor<Self>>::Target<$generic>) -> Self{
                use $crate::higher::Functor;
                Self::Free(Box::new(command.fmap(|a| Self::Pure(a))))
            }

            $v fn retract<'free_macro_reserved_lifetime>(self) -> <$f as $crate::higher::Bind<'free_macro_reserved_lifetime,Self>>::Target<$generic> where $f : $crate::higher::Monad<'free_macro_reserved_lifetime,Self>, <$f as $crate::higher::Bind<'free_macro_reserved_lifetime,Self>>::Target<$generic> : $crate::higher::Pure<$generic> {
                use $crate::higher::{Bind, Pure};
                match self {
                    $name::Pure(a) => {<$f as $crate::higher::Bind<'free_macro_reserved_lifetime,Self>>::Target::<$generic>::pure(a)},
                    $name::Free(m) => {m.bind(|a| a.retract())}
                }
            }
        }
        
        impl<'free_macro_reserved_lifetime, $($other_lifetimes,)* A> $crate::higher::Functor<'free_macro_reserved_lifetime,A> for $name<$($other_lifetimes,)* A> {
            type Target<T> = $name<$($other_lifetimes,)* T>;
            fn fmap<B,F>(self, f: F) -> Self::Target<B> where F: Fn(A) -> B + 'free_macro_reserved_lifetime{
                fn __fmap_impl<'free_macro_reserved_lifetime, $($other_lifetimes,)* A, B, F>(s : $name<$($other_lifetimes,)* A>, f: &F) -> $name<$($other_lifetimes,)* B> where F: Fn(A) -> B + 'free_macro_reserved_lifetime{
                    use $crate::higher::Functor;
                    match s {
                        $name::Pure(a) => {$name::Pure(f(a))},
                        $name::Free(fa) => {$name::Free(Box::new(fa.fmap(|x| __fmap_impl(x, f))))},
                    }
                }
                __fmap_impl(self, &f)
            }
        }

        impl<$($other_lifetimes,)* A> $crate::higher::Pure<A> for $name<$($other_lifetimes,)* A> {
            fn pure(value : A) -> Self {
                Self::Pure(value)
            }
        }

        impl<'free_macro_reserved_lifetime, $($other_lifetimes,)* A> $crate::higher::Apply<'free_macro_reserved_lifetime, A> for $name<$($other_lifetimes,)* A> where A: 'free_macro_reserved_lifetime + Clone,{
            type Target<T> = $name<$($other_lifetimes,)* T> where T:'free_macro_reserved_lifetime;
            fn apply<B>(
                self,
                f: <Self as $crate::higher::Apply<'free_macro_reserved_lifetime, A>>::Target<$crate::higher::apply::ApplyFn<'free_macro_reserved_lifetime, A, B>>,
            ) -> <Self as $crate::higher::Apply<'free_macro_reserved_lifetime, A>>::Target<B>
            where
                B: 'free_macro_reserved_lifetime,
            {
                $crate::higher::apply::ap(f,self)
            }
        }

        impl<'free_macro_reserved_lifetime, $($other_lifetimes,)* A> $crate::higher::Bind<'free_macro_reserved_lifetime,A> for $name<$($other_lifetimes,)* A>{
            type Target<T> = $name<$($other_lifetimes,)* T>;
            fn bind<B, F>(self, f: F) -> Self::Target<B>
            where
                F: Fn(A) -> Self::Target<B>,
            {
                fn __bind_impl<'free_macro_reserved_lifetime, $($other_lifetimes,)* A, B, F>(s : $name<$($other_lifetimes,)* A>, f: &F) -> $name<$($other_lifetimes,)* B> where F: Fn(A) -> $name<$($other_lifetimes,)* B> + 'free_macro_reserved_lifetime{
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
    (<$a:lifetime>, $v:vis $name:ident<$($other_lifetimes:lifetime,)+ $generic:ident>, $f:ty) =>{
        #[derive(Clone)]
        $v enum $name<$($other_lifetimes,)* $generic> {
            Pure($generic),
            Free(Box<$f>)
        }
        impl<$($other_lifetimes : $a,)* $generic> $name<$($other_lifetimes,)* $generic> where $generic : $a {
            $v fn lift_f(command : <$f as $crate::higher::Functor<$a, Self>>::Target<$generic>) -> Self{
                use $crate::higher::Functor;
                Self::Free(Box::new(command.fmap(|a| Self::Pure(a))))
            }

            $v fn retract(self) -> <$f as $crate::higher::Bind<$a,Self>>::Target<$generic> where $f : $crate::higher::Monad<$a,Self>, <$f as $crate::higher::Bind<$a,Self>>::Target<$generic> : $crate::higher::Pure<$generic> {
                use $crate::higher::{Bind, Pure};
                match self {
                    $name::Pure(a) => {<$f as $crate::higher::Bind<$a,Self>>::Target::<$generic>::pure(a)},
                    $name::Free(m) => {m.bind(|a| a.retract())}
                }
            }
        }
    
        impl<$($other_lifetimes : $a,)* A> $crate::higher::Functor<$a,A> for $name<$($other_lifetimes,)*A> where A : $a {
            type Target<T> = $name<$($other_lifetimes,)* T>;
            fn fmap<B,F>(self, f: F) -> Self::Target<B> 
                where F: Fn(A) -> B + $a
            {
                fn __fmap_impl<$($other_lifetimes : $a,)* A, B, F>(s : $name<$($other_lifetimes,)* A>, f : std::rc::Rc<F>) -> $name<$($other_lifetimes,)* B> where A : $a, F: Fn(A) -> B + $a{
                    use $crate::higher::Functor;
                    match s {
                        $name::Pure(a) => {$name::Pure(f(a))},
                        $name::Free(fa) => {$name::Free(Box::new(fa.fmap(move |x : $name<$($other_lifetimes,)*A>| __fmap_impl(x, f.clone()))))},
                    }
                }

                let r = std::rc::Rc::new(f);
                __fmap_impl(self, r)
            }
        }

        impl<$($other_lifetimes,)* A> $crate::higher::Pure<A> for $name<$($other_lifetimes,)* A> {
            fn pure(value : A) -> Self {
                Self::Pure(value)
            }
        }

        impl<$($other_lifetimes : $a,)* A> $crate::higher::Apply<$a, A> for $name<$($other_lifetimes,)* A> where A: $a + Clone,{
            type Target<T> = $name<$($other_lifetimes,)* T> where T:$a;
            fn apply<B>(
                self,
                f: <Self as $crate::higher::Apply<$a, A>>::Target<$crate::higher::apply::ApplyFn<$a, A, B>>,
            ) -> <Self as $crate::higher::Apply<$a, A>>::Target<B>
            where
                B: $a,
            {
                $crate::higher::apply::ap(f,self)
            }
        }

        impl<$($other_lifetimes : $a,)* A> $crate::higher::Bind<$a,A> for $name<$($other_lifetimes,)*A> where A : $a{
            type Target<T> = $name<$($other_lifetimes,)* T>;
            fn bind<B, F>(self, f: F) -> Self::Target<B>
            where
                F: Fn(A) -> Self::Target<B> + $a,
            {
                fn __bind_impl<$($other_lifetimes : $a,)* A, B, F>(s : $name<$($other_lifetimes,)*A>, f : std::rc::Rc<F>) -> $name<$($other_lifetimes,)* B> where A : $a, F: Fn(A) -> $name<$($other_lifetimes,)* B> + $a{
                    use $crate::higher::Functor;
                    match s {
                        $name::Pure(a) => {f(a)},
                        $name::Free(fa) => {$name::Free(Box::new(fa.fmap(move |x : $name<$($other_lifetimes,)*A>| __bind_impl(x, f.clone()))))},
                    }
                }
                let r = std::rc::Rc::new(f);
                __bind_impl(self, r)
            }
        }
    };
}