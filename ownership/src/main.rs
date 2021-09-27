fn main() {
    {
        let _s = "hello!"; // string literal stored on the stack; compiled into program binary directly, and so always immutable.

        let mut s = String::from("hello!"); // String stored on the heap; potentially mutable, as with `mut` usage here; different type to string literals!
        s.push_str(" goodbye!");

        println!("{}", s);
    } // _s is removed from the stack and s is deallocated from the heap when we leave this scope.

    {
        let x = 5;
        let _y = x; // Creates copy of x and stores in _y.

        let s1 = String::from("hello");
        let s2 = s1; // Copies pointer in s1 to String "hello" and stores in s2 (shallow copy).

        // Ownership in Rust guarantees that only one variable can refer to - or "own" - this String.
        // Thus, attempting to use s1 now causes a compilation error.
        // So what really happened earlier wasn't a "shallow copy", but a "move"!
        // println!("{}", s1); // <- compilation error

        let s3 = s2.clone(); // Does a deep copy of the String; s3 is made to own this new String.
        println!("s2 = {}, s3 = {}", s2, s3);

        // Some simple types, like i32, bool, char, and others that are storable on the stack,
        // implement a trait called Copy. This trait is what allows values of such types to
        // be deeply copied to new variables rather than moved.
    }

    {
        let s = String::from("hello"); // s comes into scope.

        takes_ownership(s);            // s's value moves into the function...
                                       // ...and so is no longer valid here.

        // println!("{}", s);
        
        let x = 5;                     // x comes into scope.

        makes_copy(x);                 // x would move into the function,
                                       // but i32 is Copy, so it's okay to still
                                       // use x afterward.      
                                       
        println!("{}", x);
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.

    {
        let s1 = gives_ownership();    // gives_ownership moves its return
                                       // value into s1.
        
        let s2 = String::from("hello");    // s2 comes into scope.

        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3.
    }
}

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // gives_ownership will move its return
                                 // value into the function that calls it.
    
    let some_string = String::from("hello"); // some_string comes into scope.

    some_string // some_string is returned and moves out into the calling function.
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope.
    a_string // a_string is returned and moves out to the calling function.
}
