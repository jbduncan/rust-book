fn main() {
    // The code below doesn't compile, because x goes
    // out of scope before r, which contains the same
    // reference, can be printed.
    //
    // In a C/C++ program, this would cause undefined
    // behaviour.
    //
    // The way Rust reports this at compile time is by
    // telling us that x, the "borrowed" value, doesn't
    // "live" as long r does. Therefore, when r is
    // printed, x no longer exists and so printing it
    // otherwise would either print the value we expect
    // or, if the computer has reassigned that part of
    // the memory already, a different value. In other
    // words, when x goes out of scope, we can't rely
    // on r holding the value we expect.
    /*
    {
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }
    */

    // ----------

    // This function accepts two references as inputs,
    // and returns one of those references as the output.
    //
    // The 'a, a lifetime annotation, tells Rust that the output
    // reference will live at least as long as the input reference
    // that will "die" (be unallocated) first.
    //
    // Therefore, in the code between lines 57-62, we can print
    // 'result' in the inner scope, because it is the place
    // where both string1 and string2 will still be allocated,
    // meaning that 'result' will still be allocated as well.
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(&string1, string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        println!("The longest string is {}", result);
    }

    // This code doesn't compile because when 'result' is
    // printed, the compiler doesn't know for sure if it
    // refers to either string1 or string2.
    // Since string2 will be out of scope by the time
    // 'result' is printed, the compiler takes the
    // conservative approach, lets us know and stops.
    /*
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
    }
    println!("The longest string is {}", result);
    */

    // We can use lifetime annotations in struct definitions
    // too.
    // In this case, 'a tells Rust that an instance of
    // ImportantExcerpt can't outlive the reference it holds
    // in its 'part' field.
    // Therefore the bottom-most println! in the comment
    // won't compile, because by that time, 'novel',
    // which is referenced indirectly by i.part, has already
    // been deallocated.
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let mut i = ImportantExcerpt { part: "Not really important..." };
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: &first_sentence
        };
        println!("{:?}", i);
    }
    // println!("{:?}", i);
}
