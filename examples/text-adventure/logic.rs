//! This module does nothing. It just creates the game's high level flow encoded as Free Monad.


use std::borrow::Cow;

use higher::{run, Functor, Pure, Bind};
use super::data::*;
use super::dsl::*;

//Haskell has a when function, and it's nice. Sooo, copy that.
macro_rules! when {
    {$a:expr => {$($b:tt)*}} => {
        if($a){
            run!{$($b)*}
        } else {
            run!{ yield () }
        }
    };
}


pub fn game<'a,'s : 'a>() -> FreeSausageRoll<'a, 's, ()>{
    run!{
        c <= intro();
        when!{c => {
            //handle_rooms is the main game loop: Go from room to room.
            c <= handle_rooms(Location::Refrigerators, Default::default()); //can't destructure in assignment in higher-0.2. Maybe later.
            //if we ended up here, we left the supermarket.
            ending(c.1)
        }}
    }
}

fn intro<'a,'s:'a>() -> FreeSausageRoll<'a, 's, bool>{
    run!{
        present_location(Location::Entrance);
        say_dialogue_line(Speaker::Partner, Cow::from("Would you be so kind as to quickly grab me a sausage roll from the supermarket? With pickle if possible?"), Mood::Friendly);
        say_dialogue_line(Speaker::Partner, Cow::from("I'd meanwhile go over to the pharmacy, and buy some pills against headache."), Mood::Friendly);
        c <= give_player_options(vec!["Say yes and enter the supermarket.", "Say no."]);
        if c == 0 {
            say_dialogue_line(Speaker::Partner, Cow::from("Thanks! We'll meet here in a couple of minutes then."), Mood::Friendly)
        }
        else { //as already stated: Let's just assume the interpreter validates input.
            say_dialogue_line(Speaker::Partner, Cow::from("Well, I won't force you. But if I get hangry, it's going to be your problem."), Mood::Annoyed)
        };
        yield c == 0
    }
}

fn ending<'a,'s:'a>(inventory : Inventory) -> FreeSausageRoll<'a, 's, ()>{
    if inventory.items.contains(&Item::SausageRoll) {
        if inventory.items.contains(&Item::Pickles){
            run!{
                say_dialogue_line(Speaker::Partner, Cow::from("Wait, seriously? You bought a glass of pickles and a sausage roll without pickle?"), Mood::Confused);
                exposition("You explain that the deli counter had run out of pickles.");
                say_dialogue_line(Speaker::Partner, Cow::from("Well, that's a creative solution."), Mood::Amused);
                say_dialogue_line(Speaker::Partner, Cow::from("Thanks a lot, let's move on."), Mood::Happy)
            }
        } else {
            run!{
                say_dialogue_line(Speaker::Partner, Cow::from("Thanks for the sausage roll, but there are no pickles in it?"), Mood::Annoyed);
                exposition("You explain that the deli counter had run out of pickles.");
                say_dialogue_line(Speaker::Partner, Cow::from("Well, that can't be helped then. Thanks a lot, let's move on."), Mood::Happy)
            }
        }
    } else {
        run!{
            say_dialogue_line(Speaker::Partner, Cow::from("What did you do in there? I asked you to bring me a sausage roll..."), Mood::Annoyed);
            when!{inventory.items.len() > 0 => {
                say_dialogue_line(Speaker::Partner, Cow::from("Also, why did you buy all that other stuff?"), Mood::Annoyed)
            }};
            say_dialogue_line(Speaker::Partner, Cow::from("Well, let's move on, but don't complain if I get hangry on the way."), Mood::Annoyed)
        }
    }
}

fn handle_rooms<'a,'s:'a>(room: Location, inventory : Inventory) -> FreeSausageRoll<'a, 's, (Location, Inventory)>{
    run!{
        c <= handle_room(room, inventory);
        if c.0 != Location::Entrance {
            //If this were an actual game, we could put a save-point here. At this location the next room to handle is just determined by room and inventory.
            handle_rooms(c.0, c.1)
        } else {
            run!{
                yield c
            }
        }
    }
}

fn handle_room<'a,'s:'a>(room : Location, inventory : Inventory) -> FreeSausageRoll<'a, 's, (Location, Inventory)> {
    run!{
        present_location(room);
        match room {
            Location::Refrigerators => handle_refrigerators(inventory.clone()),
            Location::Shelves => {handle_shelves(inventory.clone())},
            Location::Deli => {handle_deli(inventory.clone())},
            Location::Checkout => {handle_checkout(inventory.clone())},
            Location::Entrance => unreachable!(), //if we are at the entrance, we won.
        }
    }
} 

fn handle_refrigerators<'a, 's: 'a>(inventory : Inventory) -> FreeSausageRoll<'a, 's, (Location, Inventory)>{
    run!{
        c <= {
            let options = if inventory.has_item_from_room(Location::Refrigerators) {
                vec!["Move on to the Shelves.", "Move to the deli counter.", "Check Inventory", "Take an item", "Return an item"]
            } else {
                vec!["Move on to the Shelves.", "Move to the deli counter.", "Check Inventory", "Take an item"]
            };
            give_player_options(options)
        };
        match c {
            0 => { FreeSausageRoll::pure((Location::Shelves, inventory.clone())) },
            1 => { FreeSausageRoll::pure((Location::Deli, inventory.clone()))},
            2 => { 
                let inventory = inventory.clone();
                run!{
                    check_inventory(inventory.clone());
                    handle_refrigerators(inventory.clone())
                }
            }
            3 => { run!{
                i <= try_take_item(inventory.clone(), Location::Refrigerators.items());
                handle_refrigerators(i)
            } },
            4 => { run!{
                i <= return_item(inventory.clone(), Location::Refrigerators);
                handle_refrigerators(i)
            } },
            _ => unreachable!()
        }
    }
}

fn handle_shelves<'a, 's: 'a>(inventory : Inventory) -> FreeSausageRoll<'a, 's, (Location, Inventory)>{
    //this is rather similar to refrigerators. Just different items.
    run!{
        c <= {
            let options = if inventory.has_item_from_room(Location::Shelves) {
                vec!["Move on to the Refrigerators.", "Move to the deli counter.", "Move to the checkout.", "Check Inventory","Take an item", "Return an item"]
            } else {
                vec!["Move on to the Shelves.", "Move to the deli counter.", "Move to the checkout.", "Check Inventory","Take an item"]
            };
            give_player_options(options)
        };
        match c {
            0 => { FreeSausageRoll::pure((Location::Refrigerators, inventory.clone())) },
            1 => { FreeSausageRoll::pure((Location::Deli, inventory.clone()))},
            2 => { FreeSausageRoll::pure((Location::Checkout, inventory.clone()))}
            3 => {
                let inventory = inventory.clone();
                run!{
                    check_inventory(inventory.clone());
                    handle_shelves(inventory.clone())
                }
            }
            4 => { run!{
                i <= try_take_item(inventory.clone(), Location::Shelves.items());
                handle_shelves(i)
            } },
            5 => { run!{
                i <= return_item(inventory.clone(), Location::Shelves);
                handle_shelves(i)
            } },
            _ => unreachable!()
        }
    }
}

fn handle_deli<'a,'s:'a>(inventory : Inventory) -> FreeSausageRoll<'a, 's, (Location, Inventory)>{
    run!{
        c <= give_player_options(vec!["Move on to refrigerators.", "Move on to shelves.", "Check Inventory", "Talk to the lady behind the counter"]);
        match c{
            0 => FreeSausageRoll::pure((Location::Refrigerators, inventory.clone())),
            1 => FreeSausageRoll::pure((Location::Shelves, inventory.clone())),
            2 => {
                let inventory = inventory.clone();
                run!{
                    check_inventory(inventory.clone());
                    handle_deli(inventory.clone())
                }
            },
            3 => {
                run!{
                    i <= talk_to_deli_lady(inventory.clone());
                    handle_deli(i.clone())
                }
            },
            _ => unreachable!()
        }
    }
}

fn handle_checkout<'a,'s:'a>(inventory : Inventory) -> FreeSausageRoll<'a, 's, (Location, Inventory)>{
    run!{
        c <= {
            let options = if inventory.has_item_from_room(Location::Checkout) {
                vec!["Move back to the shelves.", "Pay for your stuff and leave.", "Check Inventory", "Take an item", "Return an item"]
            } else {
                vec!["Move back to the shelves.", "Pay for your stuff and leave.", "Check Inventory", "Take an item"]
            };
            give_player_options(options)
        };
        match c {
            0 => { FreeSausageRoll::pure((Location::Shelves, inventory.clone())) },
            1 => { 
                run!{
                    r <= try_pay(inventory.clone());
                    match r {
                        Ok(inventory) => {
                            run!{
                                exposition("You leave the supermarket. Your partner is already waiting outside.");
                                FreeSausageRoll::pure((Location::Entrance, inventory.clone()))
                            }
                        },
                        Err(inventory) => handle_checkout(inventory.clone()),
                    }
                }
            },
            2 => {
                let inventory = inventory.clone();
                run!{
                    check_inventory(inventory.clone());
                    handle_checkout(inventory.clone())
                }
            }
            3 => { run!{
                i <= try_take_item(inventory.clone(), Location::Checkout.items());
                handle_checkout(i)
            } },
            4 => { run!{
                i <= return_item(inventory.clone(), Location::Shelves);
                handle_checkout(i)
            } },
            _ => unreachable!()
        }
    }
}

fn try_take_item<'a, 's: 'a>(inventory : Inventory, options : Vec<Item>) -> FreeSausageRoll<'a, 's,Inventory>{
    //here we run into an "interesting" issue with Rust's ownership and do-notation.
    //We would like to capture inventory and use it in bind-notation, but that doesn't work (except in the first 2 lines), because Inventory isn't Copy.
    //This leaves us with a couple of options: We can either pass it through by repeated cloning (done here), or leave do-notation before capturing it.
    run!{
        i <= exposition("You look around and these items nearby catch your attention.").fmap(move |_| (inventory.clone(), options.clone()));
        o <= give_player_options(i.1.iter().map(|o| o.description()).chain(std::iter::once("Cancel")).collect()).fmap(move |c| (c, i.0.clone(), i.1.clone()));
        {
            let aborted = o.2.len() <= o.0;
            let updated_inventory = o.2.get(o.0).ok_or_else(|| o.1.clone()).and_then(|c| o.1.clone().try_add(*c));
            match updated_inventory {
                Ok(i) => {
                    run! {
                        exposition("You take the item.");
                        yield (i.clone())
                    }
                },
                Err(i) => {
                    run! {
                        if aborted {
                            exposition("You changed your mind, and didn't take an item.")
                        } else {
                            exposition("You try to pick up the item, but your hands are full.")
                        };
                        yield i.clone()
                    }
                }
            }
        }
    }
}

fn return_item<'a,'s:'a>(inventory : Inventory, room : Location) -> FreeSausageRoll<'a,'s, Inventory>{
    //similar to try_take_item here the inventory can't easily be captured. For illustration purposes, here we
    //don't pass it along as clones, but rather return from do-notation to capture it.
    let items_in_room = room.items();
    let carried_items_from_here = inventory.items.iter().filter(|i| items_in_room.contains(i)).cloned().collect::<Vec<_>>();
    //ownership shmownership
    let carried_items_from_here2 = carried_items_from_here.clone();
    let chosen = run! {
        exposition("You check which items you can return here. You find places where you can return the following:");
        give_player_options(carried_items_from_here.iter().map(|o| o.description()).chain(std::iter::once("Cancel")).collect())
    };
    chosen.bind(move |c| {
        match carried_items_from_here2.get(c).ok_or_else(|| inventory.clone()).and_then(|i| inventory.clone().try_remove(*i)) {
            Ok(i) => {
                run!{
                    exposition("You put back the item.");
                    yield i.clone()
                }
            },
            Err(i) => {
                run! {
                    exposition("You decided to not return an item."); //good enough. We filtered for valid items beforehand.
                    yield i.clone()
                }
            },
        }
    })
}

fn check_inventory<'a, 's:'a>(inventory : Inventory) -> FreeSausageRoll<'a,'s, ()>{
    let c = inventory.items.len();
    run!{
        exposition("You look at the items you carry. You are holding:");
        list_inventory_items(inventory.clone(),0);
        if c < 2 {
            run!{
                exposition("You check your pocket to see how much money you have.");
                exposition(Inventory::get_money())
            }
        } else {
            exposition("You would like to check how much money you have on you, but you need both hands to carry all the stuff you gathered.")
        }
    }
}

fn list_inventory_items<'a,'s:'a>(inventory : Inventory, index : usize) -> FreeSausageRoll<'a,'s, ()>{
    if index < inventory.items.len() {
        run!{
            exposition(inventory.items[index].description());
            list_inventory_items(inventory.clone(), index+1)
        }
    } else {
        FreeSausageRoll::pure(())
    }
}

fn talk_to_deli_lady<'a,'s:'a>(inventory : Inventory) -> FreeSausageRoll<'a,'s,Inventory>{
    run!{
        exposition("You greet the lady at the deli counter.");
        say_dialogue_line(Speaker::DeliLady, Cow::from("Hi! How can I help you, dear?"), Mood::Friendly);
        say_dialogue_line(Speaker::DeliLady, Cow::from("We have the most awesome fish sandwiches today. Would you like one?"), Mood::Friendly)
    }.bind(move |_| deli_lady_loop(inventory.clone()))
}

fn deli_lady_loop<'a, 's: 'a>(inventory : Inventory) -> FreeSausageRoll<'a,'s,Inventory>{
    let has_deli_item = inventory.has_item_from_room(Location::Deli);
    let c = run! {
        if has_deli_item {
            give_player_options(vec!["Yes, please!", "No, thanks. I'd rather buy a sausage roll with pickle.", "Nothing, thanks.", "Do you take stuff from the deli back?"])
        } else {
            give_player_options(vec!["Yes, please!", "No, thanks. I'd rather buy a sausage roll with pickle.", "Nothing, thanks."])
        }
    };
    c.bind(move |c| {
        match c {
            0 => {
                let inventory = inventory.clone().try_add(Item::FishSandwich);
                match inventory{
                    Ok(inventory) => {
                        run!{
                            say_dialogue_line(Speaker::DeliLady, Cow::from("Here you go! Is there anything else I can help you with? Maybe another fish sandwich?"), Mood::Happy);
                            deli_lady_loop(inventory.clone())
                        }
                    },
                    Err(inventory) => {
                        run!{
                            say_dialogue_line(Speaker::DeliLady, Cow::from("I would love to hand it to you, but your hands seem kinda full. Please come back later, when you can actually carry the food I sell."), Mood::Annoyed);
                            yield inventory.clone()
                        }
                    },
                }
            },
            1 => {
                let inventory = inventory.clone().try_add(Item::SausageRoll);
                match inventory{
                    Ok(inventory) => {
                        let d = run!{
                            say_dialogue_line(Speaker::DeliLady, Cow::from("I'm sorry, but I don't have any pickles here right now. But you can take a glass from the shelf over there."), Mood::Apologetic);
                            say_dialogue_line(Speaker::DeliLady, Cow::from("I'll put in extra sausage to make up for it."), Mood::Apologetic);
                            say_dialogue_line(Speaker::DeliLady, Cow::from("Here you go! Is there anything else I can help you with? Maybe a fish sandwich?"), Mood::Happy)
                        };
                        d.bind(move |_| deli_lady_loop(inventory.clone()))
                    },
                    Err(inventory) => {
                        run!{
                            say_dialogue_line(Speaker::DeliLady, Cow::from("I would love to hand it to you, but your hands seem kinda full. Please come back later, when you can actually carry the food I sell."), Mood::Annoyed);
                            yield inventory.clone()
                        }
                    },
                }
            },
            2 => {
                let inventory = inventory.clone();
                say_dialogue_line(Speaker::DeliLady, Cow::from("So, you are just here to steal my time? I've got other customers to serve."), Mood::Annoyed)
                .bind(move |_| FreeSausageRoll::pure(inventory.clone()))
            },
            3 => {
                let inventory = inventory.clone();
                say_dialogue_line(Speaker::DeliLady, Cow::from("No, that would be gross. Would you buy a sandwich handed back by some other random customer?"), Mood::Confused)
                .bind(move |_| FreeSausageRoll::pure(inventory.clone()))
            }
            _ => unreachable!()
        }
    })
}

fn try_pay<'a, 's:'a>(inventory : Inventory) -> FreeSausageRoll<'a,'s,Result<Inventory,Inventory>>{
    let total_price = inventory.total_price();
    let can_afford = inventory.can_afford();
    run!{
        exposition("You put your items onto the conveyor and wait until the cashier scans them.");
        {
            let line = format!("That would be €{}.{}, please.", total_price/100usize, total_price%100usize);
            say_dialogue_line(Speaker::Cashier, Cow::from(line), Mood::Friendly)
        };
        if can_afford {
            run!{
                exposition("You hand the cashier the required amount of money.");
                say_dialogue_line(Speaker::Cashier, Cow::from("Thank you very much, have a nice day!"), Mood::Friendly);
                yield Ok(())
            }
        } else {
            run!{
                exposition("When you hear the total amount you need to pay, you blush.");
                say_dialogue_line(Speaker::Cashier, Cow::from("I know that face. You haven't got enough money on you, right?"), Mood::Annoyed);
                say_dialogue_line(Speaker::Cashier, Cow::from("Please bring back some items to where you took them from, and come back when you can actually pay the stuff you want to buy."), Mood::Annoyed);
                yield Err(())
            }
        }
    }.fmap(move |e| e.map(|_| inventory.clone()).map_err(|_| inventory.clone()))
}