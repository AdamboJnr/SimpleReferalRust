extern crate referals;

use referals::Referal;
fn main() {
    let parent = Referal::new(String::from("Allan"));

    let child = Referal::new(String::from("Levy"));

    parent.add_child(child);

    println!("{:?}", parent);
}