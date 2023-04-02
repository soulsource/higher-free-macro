//This module defines the domain specific language.

use std::{rc::Rc, borrow::Cow};

use higher_free_macro::free;
use std::convert::identity;
use higher::Functor;
use super::data::{Location, Mood, Speaker};

#[derive(Clone)]
pub enum SausageRoll<'a, 's,A>{
    SayDialogueLine{
        speaker: Speaker,
        text : Cow<'s, str>, //In a real project I would just make this a String. Here it's a reference to show off lifetime support.
        mood : Mood,
        next : A
    },
    GivePlayerOptions{
        options : Vec<&'s str>,
        next : Rc<dyn Fn(usize)-> A + 'a> //let's just assume that the interpreter validates input.
    },
    PresentLocation{
        location : Location,
        next : A,
    },
    Exposition{
        text : &'s str,
        next : A,
    }
}

impl<'a,'s, A : 'a> Functor<'a,A> for SausageRoll<'a,'s,A>{
    type Target<T> = SausageRoll<'a,'s,T>;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        F: Fn(A) -> B + 'a {
        match self {
            SausageRoll::SayDialogueLine { speaker, text, mood, next } => SausageRoll::SayDialogueLine { speaker, text, mood, next: f(next) },
            SausageRoll::GivePlayerOptions { options, next } => SausageRoll::GivePlayerOptions { options, next: Rc::new(move |x| f(next(x))) },
            SausageRoll::PresentLocation { location, next } => SausageRoll::PresentLocation { location, next: f(next) },
            SausageRoll::Exposition { text, next } => SausageRoll::Exposition { text, next: f(next) },
        }
    }
}

free!(<'a>, pub FreeSausageRoll<'a,'s,A>, SausageRoll<'a, 's, FreeSausageRoll<'a,'s,A>>);

pub fn say_dialogue_line<'a,'s:'a>(speaker : Speaker, text : Cow<'s,str>, mood : Mood) -> FreeSausageRoll<'a, 's, ()>{
    FreeSausageRoll::lift_f(SausageRoll::SayDialogueLine { speaker, text, mood, next: () })
}

pub fn give_player_options<'a,'s:'a>(options : Vec<&'s str>) -> FreeSausageRoll<'a,'s, usize>{
    FreeSausageRoll::lift_f(SausageRoll::GivePlayerOptions { options, next: Rc::new(identity) })
}

pub fn present_location<'a, 's:'a>(location : Location) -> FreeSausageRoll<'a,'s, ()>{
    FreeSausageRoll::lift_f(SausageRoll::PresentLocation { location, next: () })
}

pub fn exposition<'a,'s:'a>(text : &'s str) -> FreeSausageRoll<'a,'s,()>{
    FreeSausageRoll::lift_f(SausageRoll::Exposition { text, next: () })
}
