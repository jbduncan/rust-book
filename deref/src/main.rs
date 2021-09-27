use std::ops::Deref;

fn main() {
    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y); // This is the deref operator; turns reference into value it points to.
    }

    {
        // "Smart pointers", that is types implementing Deref (and Drop),
        // can also be dereferenced with *.
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        // Here we build our own smart pointer: a Box-like type that
        // stores things on the stack rather than the heap.

        struct MyBox<T>(T); // Side note: this is a tuple struct

        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }

        impl<T> Deref for MyBox<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);

        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }

        let m = MyBox::new("Rust".to_string());
        hello(&m); // Deref coercion: passing &MyBox<String> automatically dereferences it to &String, and then &str (because String has a Deref impl that turns Strings into string slices (&str))
        // Above is equivalent to:
        // hello(&(*m)[..]);
        //   or
        // hello(&**m);

        // No penalty for using .deref(), because it's resolved at
        // compile time!
    }
}
