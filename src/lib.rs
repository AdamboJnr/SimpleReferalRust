#![allow(dead_code)]
#![allow(unused_imports)]

use std::cell::RefCell;
use std::rc::{ Rc, Weak };
use std::iter::Map;
use std::io::stdin;
use std::process;

#[derive(Debug)]
pub struct Referal {
    name: String,
    parent: RefCell<Weak<Referal>>,
    children: RefCell<Vec<Rc<Referal>>>,
    commission: RefCell<i64>,
}

impl Referal {
    pub fn new(name: String, users: &CreatedUsers) -> Rc<Referal> {
       let new_referal = Rc::new(Referal { name , children: RefCell::new(vec![]), parent: RefCell::new(Weak::new() ), commission: RefCell::new(0) });

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

    pub fn check_children(&self) -> Vec<String>{
        self.children.borrow().iter().map(|child | {
            child.name.to_string()
        }).collect()
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

pub fn read_input(inp: &mut String) -> usize {
    stdin().read_line( inp).unwrap_or_else(| err | {
        println!("Error: {}", err);
        process::exit(1);
    })
}

#[cfg(test)]
mod tests {
    use super::*;

}
