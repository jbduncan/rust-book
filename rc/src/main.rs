use std::rc::Rc;

fn main() {
    // To copy what the Rust Book, Chapter 15.4 says,
    // "In the majority of cases, ownership is clear: you know exactly
    // which variable owns a given value. However, there are cases when
    // a single value might have multiple owners. For example, in graph
    // data structures, multiple edges might point to the same node, and
    // that node is conceptually owned by all of the edges that point to
    // it. A node shouldn’t be cleaned up unless it doesn't have any
    // edges pointing to it."
    //
    // Rc allows us to do this multi-ownership. It's short for
    // "reference counting".
    //
    // Although note that Rc is only to be used in single-threaded
    // contexts.

    {
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        use List::{Cons, Nil};

        let _a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        let _b = Cons(3, Box::new(_a));
        // let _c = Cons(4, Box::new(_a));
    }

    // The above code doesn't compile because _b and _c both try to
    // own _a. This is something we want to allow, however, so that
    // two or more Cons Lists can share data. This is where Rc comes
    // into play.

    // Why not use references? The Rust Book explains:
    //
    // "We could change the definition of Cons to hold references
    // instead, but then we would have to specify lifetime parameters.
    // By specifying lifetime parameters, we would be specifying that
    // every element in the list will live at least as long as the
    // entire list. The borrow checker wouldn't let us compile
    // `let a = Cons(10, &Nil);` for example, because the temporary Nil
    // value would be dropped before a could take a reference to it."

    {
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

        use List::{Cons, Nil};

        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}
