#![allow(dead_code)]
#![allow(unused_imports)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Referal {
    name: String,
    children: RefCell<Vec<Rc<Referal>>>,
    commission: RefCell<i64>,
}

impl Referal {
    pub fn new(name: String, users: &CreatedUsers) -> Rc<Referal> {
       let new_referal = Rc::new(Referal { name , children: RefCell::new(vec![]), commission: RefCell::new(0) });

       users.add_user(Rc::clone(&new_referal));

        new_referal
    }

    pub fn add_child(&self, child: Rc<Referal>) {
        self.children.borrow_mut().push(Rc::clone(&child))
    }

    pub fn check_balance(&self) -> &RefCell<i64> {
        if self.children.borrow().len() >= 4 {
            *self.commission.borrow_mut() += 400
        }

        &self.commission
    }
}

#[derive(Debug)]
pub struct CreatedUsers {
    users: RefCell<Vec<Rc<Referal>>>
}

impl CreatedUsers {
    pub fn new() -> CreatedUsers {
        CreatedUsers { users: RefCell::new(vec![]) }
    }

    pub fn add_user(&self, user: Rc<Referal>) {
        self.users.borrow_mut().push(user);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
