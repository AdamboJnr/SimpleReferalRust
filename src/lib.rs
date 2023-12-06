#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Referal {
    name: String,
    children: RefCell<Vec<Rc<Referal>>>,
}

impl Referal {
    pub fn new(name: String, users: &CreatedUsers) -> Rc<Referal> {
       let new_referal = Rc::new(Referal { name , children: RefCell::new(vec![]) });

       users.add_user(new_referal);

        new_referal
    }

    pub fn add_child(&self, child: Rc<Referal>) {
        self.children.borrow_mut().push(Rc::clone(&child))
    }
}

pub struct CreatedUsers {
    users: RefCell<Vec<Rc<Referal>>>
}

impl CreatedUsers {
    pub fn new() -> CreatedUsers {
        CreatedUsers { users: RefCell::new(vec![]) }
    }

    pub fn add_user(&self, user: Rc<Referal>) {
        self.users.borrow_mut().push(Rc::clone(&user));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
