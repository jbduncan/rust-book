fn main() {
    {
        let s = String::from("hello world");

        // All string slices are of type '&str'
        let hello: &str = &s[..5];
        let world: &str = &s[6..];
        println!("words: '{}', '{}'", hello, world)
    }

    {
        let mut s = String::from("hello world");
        
        // String implements `Deref<Target=str>`, so they can be passed to functions accepting &str.
        let word = first_word(&s);
        println!("first word = {}", word);

        // s.clear(); // ILLEGAL, since this would cause both an immutable and mutable borrow to occur.

        println!("s = {}", s);
    }

    {
        // String literals are stored in the binary.
        // Type &str is actually a String slice that points to a location in the binary.
        // Since binaries are immutable, this explains why &str, an immutable reference, is used.
        let _s: &str = "Hello, world!";
    }

    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        println!("slice = {:?}", slice)
    }
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     return s.len();
// }

fn first_word(s: &str) -> &str {
    for (i, item) in s.bytes().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
