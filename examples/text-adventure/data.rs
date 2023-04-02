#[derive(Clone, Copy)]
pub enum Speaker{
    Partner,
    DeliLady,
    Cashier,
}

impl Speaker{
    pub fn text_description(self)->&'static str{
        match self{
            Speaker::Partner => "Your partner",
            Speaker::DeliLady => "The lady behind the deli counter",
            Speaker::Cashier => "The cashier",
        }
    }
}

#[derive(Clone, Copy)]
pub enum Mood{
    Friendly,
    Confused,
    Happy,
    Amused,
    Annoyed,
    Apologetic,
}

impl Mood{
    pub fn text_description(self)->&'static str{
        match self{
            Mood::Friendly => "a friendly expression",
            Mood::Confused => "a confused expression",
            Mood::Happy => "a happy expression",
            Mood::Amused => "an amused expression",
            Mood::Annoyed => "an annoyed expression",
            Mood::Apologetic => "an apologetic expression",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Location{
    Entrance,
    Deli,
    Checkout,
    Refrigerators,
    Shelves,
}

impl Location{
    //Used if we want to present the location in text. If we were writing a visual novel, we would offer another function that returns the respective assets.
    pub fn get_text_description(self)->&'static str{
        match self {
            Location::Entrance => "You are at the entrance area of the super market. Behind you is the parking lot, in front the inviting automated doors of the entrance. Your partner is here with you.",
            Location::Deli => "This is the area with the deli counter. There is a lady wearing a hair protector and plastic gloves standing behind the presentation tray.",
            Location::Checkout => "You have reached the checkout area of the super market. Stands full of sweets and other stuff that might attract the attention of people waiting to pay dominate this area. There is an employee sitting at one of the counters.",
            Location::Refrigerators => "This is the are where fresh products are waiting to be picked up. Refrigerators with milk, cheese and similar stuff are lined along the wall.",
            Location::Shelves => "This is the main area of the super market. Here you find several shelves filled with more or less useful stuff, ranging from conserved vegetables to cleaning utensils.",
        }
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
            Item::SausageRoll | Item::FishSandwich | Item::Shots => 300,
            Item::Pickles | Item::Pulp => 250,
            Item::Milk | Item::Yoghurt | Item::Beer => 125,
            Item::Cheese => 750,
            Item::CatFood => 2500,
            Item::ToiletPaper => 500,
            Item::ChewingGum => 100,
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
#[derive(Clone, Default)]
pub struct Inventory {
    pub items : Vec<Item>,
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
            Ok(Inventory{items})
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
        self.items.iter().copied().map(Item::price).sum::<usize>()
    }
    pub fn can_afford(&self) -> bool {
        self.total_price() <= 1000_usize
    }
}