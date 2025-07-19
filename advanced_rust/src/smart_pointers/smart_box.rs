// Box<T>, for allocating values on the heap
// Rc<T>, a reference counting type that enables multiple ownership
// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    if let Cons(head, tail) = &list {
        println!("The head of the list is: {}", head);
    }

    println!("Lists: {:?} ", list);

    // println!("Lists: {:?} ", list);
}
