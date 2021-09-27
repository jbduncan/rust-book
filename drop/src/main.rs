fn main() {
    // Drop is like a "destructor" in languages like C++, and has a
    // similar purpose to try-with-resources in Java and defer in Go:
    // it allows resources to be released when going out of scope.
    //
    // Drop, along with Deref, are two of the pieces of the puzzle
    // for implementing "smart pointers".

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    {
        let _c = CustomSmartPointer {
            data: "my stuff".to_string(),
        };
        let _d = CustomSmartPointer {
            data: "other stuff".to_string()
        };
        println!("CustomSmartPointers created.");
        // Note that the "drop"s are called in reverse order, just like
        // defer in Go and try-with-resources in Java.
    }

    {
        // Following call to "drop" is not allowed, as the smart
        // pointer would be implicitly dropped again after it goes
        // out of scope, causing a "double free error".
        //
        // However, if we legitimately need to drop a piece of data
        // early, we can use std::mem::drop, which safely drops the
        // data early.
        let _c = CustomSmartPointer {
            data: "some data".to_string(),
        };
        println!("CustomSmartPointer created.");
        // _c.drop();
        std::mem::drop(_c);
        println!("CustomSmartPointer dropped before end of main.");
    }
}
