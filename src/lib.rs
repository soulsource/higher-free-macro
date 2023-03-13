pub extern crate higher;

#[macro_export]
macro_rules! free {
    ($v:vis $name:ident<$generic:ident>, $f:ty) => {
        #[derive(Clone)]
        $v enum $name<$generic> {
            Pure($generic),
            Free(Box<$f>)
        }
        impl<$generic> $name<$generic>{
            $v fn lift_f(command : <$f as $crate::higher::Functor<Self>>::Target<$generic>) -> Self{
                use $crate::higher::Functor;
                Self::Free(Box::new(command.fmap(|a| Self::Pure(a))))
            }

            $v fn retract<'a>(self) -> <$f as $crate::higher::Bind<'a,Self>>::Target<$generic> where $f : $crate::higher::Monad<'a,Self>, <$f as $crate::higher::Bind<'a,Self>>::Target<$generic> : $crate::higher::Pure<$generic> {
                use $crate::higher::{Bind, Pure};
                match self {
                    $name::Pure(a) => {<$f as $crate::higher::Bind<'a,Self>>::Target::<$generic>::pure(a)},
                    $name::Free(m) => {m.bind(|a| a.retract())}
                }
            }
        }
        
        impl<'a,A> $crate::higher::Functor<'a,A> for $name<A> {
            type Target<T> = $name<T>;
            fn fmap<B,F>(self, f: F) -> Self::Target<B> where F: Fn(A) -> B + 'a{
                fn __fmap_impl<'a, A, B, F>(s : $name<A>, f: &F) -> $name<B> where F: Fn(A) -> B + 'a{
                    use $crate::higher::Functor;
                    match s {
                        $name::Pure(a) => {$name::Pure(f(a))},
                        $name::Free(fa) => {$name::Free(Box::new(fa.fmap(|x| __fmap_impl(x, f))))},
                    }
                }
                __fmap_impl(self, &f)
            }
        }

        impl<A> $crate::higher::Pure<A> for $name<A> {
            fn pure(value : A) -> Self {
                Self::Pure(value)
            }
        }

        impl<'a, A> $crate::higher::Apply<'a, A> for $name<A> where A: 'a + Clone,{
            type Target<T> = $name<T> where T:'a;
            fn apply<B>(
                self,
                f: <Self as $crate::higher::Apply<'a, A>>::Target<$crate::higher::apply::ApplyFn<'a, A, B>>,
            ) -> <Self as $crate::higher::Apply<'a, A>>::Target<B>
            where
                B: 'a,
            {
                $crate::higher::apply::ap(f,self)
            }
        }

        impl<'a,A> $crate::higher::Bind<'a,A> for $name<A>{
            type Target<T> = $name<T>;
            fn bind<B, F>(self, f: F) -> Self::Target<B>
            where
                F: Fn(A) -> Self::Target<B>,
            {
                fn __bind_impl<'a, A, B, F>(s : $name<A>, f: &F) -> $name<B> where F: Fn(A) -> $name<B> + 'a{
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
    ($v:vis $name:ident<$a:lifetime, $($other_lifetimes:lifetime,)* $generic:ident>, $f:ty) =>{
        #[derive(Clone)]
        $v enum $name<$a, $($other_lifetimes,)* $generic> {
            Pure($generic),
            Free(Box<$f>)
        }
        impl<$a, $($other_lifetimes,)* $generic> $name<$a, $($other_lifetimes,)* $generic> where $generic : $a {
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
    
        impl<$a, $($other_lifetimes,)* A> $crate::higher::Functor<$a,A> for $name<$a, $($other_lifetimes,)*A> where A : $a {
            type Target<T> = $name<$a, $($other_lifetimes,)* T>;
            fn fmap<B,F>(self, f: F) -> Self::Target<B> 
                where F: Fn(A) -> B + $a
            {
                fn __fmap_impl<$a, $($other_lifetimes,)* A, B, F>(s : $name<$a, $($other_lifetimes,)* A>, f : std::rc::Rc<F>) -> $name<$a, $($other_lifetimes,)* B> where A : $a, F: Fn(A) -> B + $a{
                    use $crate::higher::Functor;
                    match s {
                        $name::Pure(a) => {$name::Pure(f(a))},
                        $name::Free(fa) => {$name::Free(Box::new(fa.fmap(move |x : $name<$a, $($other_lifetimes,)*A>| __fmap_impl(x, f.clone()))))},
                    }
                }

                let r = std::rc::Rc::new(f);
                __fmap_impl(self, r)
            }
        }

        impl<$a, $($other_lifetimes,)* A> $crate::higher::Pure<A> for $name<$a, $($other_lifetimes,)* A> {
            fn pure(value : A) -> Self {
                Self::Pure(value)
            }
        }

        impl<$a, $($other_lifetimes,)* A> $crate::higher::Apply<$a, A> for $name<$a, $($other_lifetimes,)* A> where A: $a + Clone,{
            type Target<T> = $name<$a, $($other_lifetimes,)* T> where T:$a;
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

        impl<$a, $($other_lifetimes,)* A> $crate::higher::Bind<$a,A> for $name<$a, $($other_lifetimes,)*A> where A : $a{
            type Target<T> = $name<$a, $($other_lifetimes,)* T>;
            fn bind<B, F>(self, f: F) -> Self::Target<B>
            where
                F: Fn(A) -> Self::Target<B> + $a,
            {
                fn __bind_impl<$a, $($other_lifetimes,)* A, B, F>(s : $name<$a, $($other_lifetimes,)*A>, f : std::rc::Rc<F>) -> $name<$a, $($other_lifetimes,)* B> where A : $a, F: Fn(A) -> $name<$a, $($other_lifetimes,)* B> + $a{
                    use $crate::higher::Functor;
                    match s {
                        $name::Pure(a) => {f(a)},
                        $name::Free(fa) => {$name::Free(Box::new(fa.fmap(move |x : $name<$a, $($other_lifetimes,)*A>| __bind_impl(x, f.clone()))))},
                    }
                }
                let r = std::rc::Rc::new(f);
                __bind_impl(self, r)
            }
        }
    };
}