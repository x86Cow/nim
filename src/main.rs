use rand::prelude::*;
use std::io;

#[derive(Debug)]

struct Piles {
    one: i32,
    two: i32,
    three: i32
}

#[macro_use]
extern crate text_io;
fn main() -> io::Result<()> {

    let mut pile: Piles = pile_gen();
    
    let mut playerIndicator = true;
    while pile.one > 0 || pile.two > 0 || pile.three > 0 {
        playerIndicator = !playerIndicator;
        player_turn(&mut pile);
    }  
   
    Ok(())

}

fn pile_gen() -> Piles {
    let mut rng = thread_rng();
    let pile = Piles {
        one: rng.gen_range(3..=10),
        two: rng.gen_range(3..=10),
        three: rng.gen_range(3..=10),
    };
    pile
}

fn player_turn(pile : &mut Piles) {
    println!("Pile 1: {}", pile.one);
    println!("Pile 2: {}", pile.two);
    println!("Pile 3: {}", pile.three);
    println!("Which pile would you like to take from?");
    let pile_choice: i32 = read!();
    println!("How many would you like to take?");
    let take_choice: i32 = read!();
    if take_choice > 3 || take_choice < 1 || take_choice {
        println!("You can only take up to 3");
    } else if pile_choice == 1 {
        let new_pile = pile.one - take_choice;
        pile.one = new_pile;
    } else if pile_choice == 2 {
        let new_pile = pile.two - take_choice;
        pile.two = new_pile;
    } else if pile_choice == 3 {
        let new_pile = pile.three - take_choice;
        pile.three = new_pile;
        } else {
        println!("Please choose a pile between 1 and 3");
    }
}