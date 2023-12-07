extern crate referals;

use referals::{ Referal, CreatedUsers };
// use std::rc::Rc;

fn main() {
    let users = CreatedUsers::new();

    // Creating a new user - Taking user name
    println!("Please enter your name: ");
    let mut user = String::new();
    referals::read_input(&mut user);

    // Creating a new user - Taking referee name
    println!("Enter name of Referee: ");
    let mut user_referee = String::new();
    referals::read_input(&mut user_referee);

    println!("User: {}, Referee: {}", user, user_referee);

    let parent = Referal::new(String::from("Allan"), &users);

    let child = Referal::new(String::from("Levy"), &users);
    let child2 = Referal::new(String::from("Brayan"), &users);
    let child3 = Referal::new(String::from("Adambo"), &users);
    let child4 = Referal::new(String::from("Linusy"), &users);

    parent.add_child(child);
    parent.add_child(child2);
    parent.add_child(child3);
    parent.add_child(child4);

    // println!("{:?}", parent);
    // println!("{:?}", Rc::strong_count(&parent));
    // println!("{:?}", users);
    // println!("{:?}", parent.check_balance());
    println!("{:?}", parent.check_children());
}

