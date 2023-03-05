pub extern crate higher;
pub use paste::paste;

#[macro_export]
macro_rules! free {
    ($v:vis $name:ident<$generic:ident>, $f:ty) => {
        $crate::paste! {
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
            mod [<$name _module>] {
                use $crate::higher::{Functor, Apply, Bind, Applicative, Monad, Pure, apply::ApplyFn, apply::ap};
                use super::$name;
                impl<$generic> $name<$generic> {
                    fn __fmap_impl<'a, B, F>(self, f: &F) -> $name<B> where F: Fn($generic) -> B + 'a{
                        match self {
                            $name::Pure(a) => {$name::Pure(f(a))},
                            $name::Free(fa) => {$name::Free(Box::new(fa.fmap(|x| x.__fmap_impl(f))))},
                        }
                    }
                    fn __bind_impl<'a, B, F>(self, f: &F) -> $name<B> where F: Fn($generic) -> $name<B> + 'a{
                        match self {
                            $name::Pure(a) => {f(a)},
                            $name::Free(m) => {$name::Free(Box::new(m.fmap(|x| x.__bind_impl(f))))},
                        }
                    }
                }
        
                impl<'a,A> Functor<'a,A> for $name<A> {
                    type Target<T> = $name<T>;
                    fn fmap<B,F>(self, f: F) -> Self::Target<B> where F: Fn(A) -> B + 'a{
                        self.__fmap_impl(&f)
                    }
                }
        
                impl<A> Pure<A> for $name<A> {
                    fn pure(value : A) -> Self {
                        Self::Pure(value)
                    }
                }
        
                impl<'a, A> Apply<'a, A> for $name<A> where A: 'a + Clone,{
                    type Target<T> = $name<T> where T:'a;
                    fn apply<B>(
                        self,
                        f: <Self as Apply<'a, A>>::Target<ApplyFn<'a, A, B>>,
                    ) -> <Self as Apply<'a, A>>::Target<B>
                    where
                        B: 'a,
                    {
                        ap(f,self)
                    }
                }
        
                impl<'a,A> Bind<'a,A> for $name<A>{
                    type Target<T> = $name<T>;
                    fn bind<B, F>(self, f: F) -> Self::Target<B>
                    where
                        F: Fn(A) -> Self::Target<B>,
                    {
                        self.__bind_impl(&f)
                    }
                }
            }
         }
    };
    ($v:vis $name:ident<$a:lifetime, $($other_lifetimes:lifetime,)* $generic:ident>, $f:ty) =>{
        $crate::paste! {
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
            mod [<$name _module>] {
                use $crate::higher::{Functor, Apply, Bind, Applicative, Monad, Pure, apply::ApplyFn, apply::ap};
                use super::$name;
                impl<$a, $($other_lifetimes,)* $generic> $name<$a, $($other_lifetimes,)* $generic> where $generic : $a {
                    fn __fmap_impl<B, F>(self, f: std::rc::Rc<F>) -> $name<$a, $($other_lifetimes,)* B> where F: Fn($generic) -> B + $a{
                        match self {
                            $name::Pure(a) => {$name::Pure(f(a))},
                            $name::Free(fa) => {$name::Free(Box::new(fa.fmap(move |x : Self| x.__fmap_impl(f.clone()))))},
                        }
                    }
                    fn __bind_impl<B, F>(self, f: std::rc::Rc<F>) -> $name<$a, $($other_lifetimes,)* B> where F: Fn($generic) -> $name<$a, $($other_lifetimes,)* B> + $a{
                        match self {
                            $name::Pure(a) => {f(a)},
                            $name::Free(m) => {$name::Free(Box::new(m.fmap(move |x| x.__bind_impl(f.clone()))))},
                        }
                    }
                }
        
                impl<$a, $($other_lifetimes,)* A> Functor<$a,A> for $name<$a, $($other_lifetimes,)*A> where A : $a {
                    type Target<T> = $name<$a, $($other_lifetimes,)* T>;
                    fn fmap<B,F>(self, f: F) -> Self::Target<B> 
                        where F: Fn(A) -> B + $a
                    {
                        let r = std::rc::Rc::new(f);
                        self.__fmap_impl(r)
                    }
                }
        
                impl<$a, $($other_lifetimes,)* A> Pure<A> for $name<$a, $($other_lifetimes,)* A> {
                    fn pure(value : A) -> Self {
                        Self::Pure(value)
                    }
                }
        
                impl<$a, $($other_lifetimes,)* A> Apply<$a, A> for $name<$a, $($other_lifetimes,)* A> where A: $a + Clone,{
                    type Target<T> = $name<$a, $($other_lifetimes,)* T> where T:$a;
                    fn apply<B>(
                        self,
                        f: <Self as Apply<$a, A>>::Target<ApplyFn<$a, A, B>>,
                    ) -> <Self as Apply<$a, A>>::Target<B>
                    where
                        B: $a,
                    {
                        ap(f,self)
                    }
                }
        
                impl<$a, $($other_lifetimes,)* A> Bind<$a,A> for $name<$a, $($other_lifetimes,)*A> where A : $a{
                    type Target<T> = $name<$a, $($other_lifetimes,)* T>;
                    fn bind<B, F>(self, f: F) -> Self::Target<B>
                    where
                        F: Fn(A) -> Self::Target<B> + $a,
                    {
                        let r = std::rc::Rc::new(f);
                        self.__bind_impl(r)
                    }
                }
            }
        }
    };
}