//This module defines the domain specific language.

use std::{rc::Rc, borrow::Cow};

use higher_free_macro::free;
use std::convert::identity;
use higher::Functor;

#[derive(Clone)]
pub enum Speaker{
    Partner,
    DeliLady,
    Cashier,
}

#[derive(Clone)]
pub enum Mood{
    Friendly,
    Confused,
    Happy,
    Amused,
    Annoyed,
    Apologetic,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Location{
    Entrance,
    Deli,
    Checkout,
    Refrigerators,
    Shelves,
}

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
#[derive(Copy,Clone,PartialEq,Eq)]
pub enum Item{
    //refrigerators
    Milk,
    Yoghurt,
    Cheese,
    //shelves
    Pickles,
    CatFood,
    Beer,
    ToiletPaper,
    //deli
    SausageRoll,
    FishSandwich,
    //cashier
    ChewingGum,
    Shots,
    Pulp,
}

impl Item {
    fn price(self) ->  usize {
        match self {
            Item::SausageRoll => 300,
            Item::Pickles => 250,
            Item::Milk => 125,
            Item::Yoghurt => 125,
            Item::Cheese => 750,
            Item::CatFood => 2500,
            Item::Beer => 125,
            Item::ToiletPaper => 500,
            Item::FishSandwich => 300,
            Item::ChewingGum => 100,
            Item::Shots => 300,
            Item::Pulp => 250,
            
        }
    }
    pub fn description(self) -> &'static str {
        match self {
            Item::SausageRoll => "A sausage roll, costing €3.00.",
            Item::Pickles => "A glass of pickles, costing €2.50",
            Item::Milk => "A bottle of milk, costing €1.25",
            Item::Yoghurt => "A cup of yoghurt, costing €1.25",
            Item::Cheese => "A block of expensive grey cheese, costing €7.50",
            Item::CatFood => "A bag of cat food, costing €25.00",
            Item::Beer => "A bottle of beer, for €1.25",
            Item::ToiletPaper => "A package of toilet paper, costing €5.00",
            Item::FishSandwich => "A fish sandwich, emitting a tasty smell, costing €3.00",
            Item::ChewingGum => "A pack of chewing gum, costing €1.00",
            Item::Shots => "A shot of a sad excuse for whisky, costing €3.00",
            Item::Pulp => "A pulp novel called \"Aliens ate my trashbin\", which should not cost the €2.50 it does",
            
        }
    }
}

impl Location{
    pub fn items(self) -> Vec<Item> {
        match self {
            Location::Entrance => Vec::default(),
            Location::Deli => vec![], //must talk to deli lady to get the sausage roll. This also means it cannot be returned.
            Location::Checkout => vec![Item::ChewingGum, Item::Shots, Item::Pulp],
            Location::Refrigerators => vec![Item::Milk, Item::Yoghurt, Item::Cheese],
            Location::Shelves => vec![Item::Pickles, Item::CatFood, Item::Beer, Item::ToiletPaper],
        }
    }
}

//In a real project I would probably aim to make this Copy as well, especially if it's as small as this.
//I left it as Clone intentionally, to illustrate how one can work around the limitation of it not being Copy.
#[derive(Clone)]
pub struct Inventory {
    pub items : Vec<Item>,
}

impl Default for Inventory{
    fn default() -> Self {
        Self { items: Default::default() }
    }
}

impl Inventory{
    pub fn has_item_from_room(&self, room : Location) -> bool {
        let items_from_room = room.items();
        self.items.iter().any(|i| items_from_room.contains(i))
    }
    pub fn try_add(self, item : Item) -> Result<Self, Self>{
        if self.items.len() < 3 {
            let mut items = self.items;
            items.push(item); //am I the only one that hates that push doesn't return the updated vec?
            Ok(Inventory{items, ..self})
        } else {
            Err(self)
        }
    }
    pub fn try_remove(mut self, item : Item) -> Result<Self, Self>{
        let idx = self.items.iter().position(|i| *i == item);
        match idx {
            Some(idx) => {
                self.items.swap_remove(idx);
                Ok(self)
            },
            None => Err(self),
        }
    }
    pub fn get_money() -> &'static str{
        "€10.00" //It doesn't change. Can as well be a constant.
    }
    pub fn total_price(&self) -> usize {
        self.items.iter().cloned().map(Item::price).sum::<usize>()
    }
    pub fn can_afford(&self) -> bool {
        self.total_price() <= 1000_usize
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
