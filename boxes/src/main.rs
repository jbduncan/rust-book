fn main() {
    {
        let b = Box::new(5); // Stores value on heap
        println!("b = {}", b);
    }

    {
        // Using Box allows us to write "recursive types": types that
        // refer to themselves, like the functional "List" type below.

        #[derive(Debug)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        use List::{Cons, Nil};

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

        println!("{:?}", list);
    }
}
