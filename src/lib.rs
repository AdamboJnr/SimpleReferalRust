#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Referal {
    name: String,
    children: RefCell<Vec<Rc<Referal>>>
}

impl Referal {
    pub fn new(name: String) -> Rc<Referal> {
       Rc::new(Referal { name , children: RefCell::new(vec![]) }) 
    }

    pub fn add_child(&self, child: Rc<Referal>) {
        self.children.borrow_mut().push(child)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
