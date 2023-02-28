use std::rc::Rc;
use std::cell::RefCell;

struct Cat {
    name: RefCell<String>
}

impl Cat {
    fn new(name: &str) -> Self {
        Self {
            name: RefCell::new(name.to_string())
        }
    }
}

impl Drop for Cat {
    fn drop(&mut self) {
        println!("{:?} was dropped!", self.name);
    }
}

struct CatOwner {
    cat: Rc<Cat>,
}

impl CatOwner {
    fn feed(&self) {
        let mut name_borrow = self.cat.name.borrow_mut();
        *name_borrow += "(purring)";
    }
}

fn main() {
    let cats = vec![
        Rc::new(Cat::new("Frodo")),
        Rc::new(Cat::new("Bilbo")),
        Rc::new(Cat::new("Pippin")),
    ];
    
    let mut owners = Vec::new();
    for cat in cats.iter() {
        owners.push(CatOwner{ cat: cat.clone() });
    }
    
    for owner in owners.iter() {
        owner.feed();
    }
    
    for owner in owners.iter() {
        println!("{}", owner.cat.name.borrow());
    }
}