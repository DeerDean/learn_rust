use std::ops::{Deref, DerefMut};
use std::fmt;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// -----------------------

struct MyBox<T: fmt::Display> (T);

impl<T: fmt::Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: fmt::Display> fmt::Display for MyBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// &T -> &U  or  &mut T -> &U
impl<T: fmt::Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // the first element
    }
}

// &mut T -> &mut U
impl<T: fmt::Display> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0 // the first element
    }
}

impl<T: fmt::Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping Mybox with data '{}'", &self);
    }
}

// -------------------------

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn hello_mut(name: &mut str) {
    println!("Hello, {}!", name);
}

// -------------------------

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

#[derive(Debug)]
enum RefCellList {
    RefCellCons(Rc<RefCell<i32>>, Rc<RefCellList>),
    RefCellNil,
}

#[derive(Debug)]
enum LeakList {
    LeakCons(i32, RefCell<Rc<LeakList>>),
    LeakNil,
}

use crate::LeakList::LeakCons;
impl LeakList {
    fn tail(&self) -> Option<&RefCell<Rc<LeakList>>> {
        match self {
            LeakCons(_, item) => Some(item),
            LeakNil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}


fn main() {

    // Box ================
    use crate::List::{Cons, Nil};
    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil))))));

    let a = 5;
    let b = &a;
    let c = Box::new(a);

    assert_eq!(5, a);
    assert_eq!(5, *b);
    assert_eq!(5, *c);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));
    // drop(y); // Early drop
    println!("Now about deref");

    let mut m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);

    hello_mut(&mut m);
    hello_mut(&mut (*m)[..]);

    
    // Rc ================
    use crate::RcList::{RcCons, RcNil};
    let a = Rc::new(RcCons(5, 
        Rc::new(RcCons(10,
            Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));


    // RefCell  ================
    use crate::RefCellList::{RefCellCons, RefCellNil};
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(RefCellCons(Rc::clone(&value), Rc::new(RefCellNil)));
    let b = RefCellCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RefCellCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);


    // Mem Leak =======================
    use crate::LeakList::{LeakCons, LeakNil};
    // a: (5, Nil)
    let a = Rc::new(LeakCons(5, RefCell::new(Rc::new(LeakNil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    // b: (10, &a)
    let b = Rc::new(LeakCons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    // a: (5, &b) 
    // b: (10, &a)
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // Weak ====================
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}




