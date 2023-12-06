use std::cell::RefCell;
use std::rc::Rc;

pub struct Referal {
    name: String,
    children: RefCell<Vec<Rc<Referal>>>
}

impl Referal {
    pub fn new(name: String) -> Rc<Referal> {
       Rc::new(Referal { name , children: RefCell::new(vec![]) }) 
    }

    pub fn add_child(&self) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
