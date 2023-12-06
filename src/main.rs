extern crate referals;

use referals::{ Referal, CreatedUsers };
fn main() {
    let users = CreatedUsers::new();

    let parent = Referal::new(String::from("Allan"), &users);

    let child = Referal::new(String::from("Levy"), &users);

    parent.add_child(child);

    println!("{:?}", parent);
}