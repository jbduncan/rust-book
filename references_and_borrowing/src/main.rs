fn main() {
    {
        let s = String::from("hello");
        
        let len = calculate_length(&s);

        println!("The length of '{}' is {}.", s, len);
    }

    {
        let mut s = String::from("hello");

        change(&mut s);
    }

    {
        let mut s = String::from("hello");

        {
            let _r1 = &mut s;
        
        } // r1 goes out of scope here, so we can make a new reference with no problems.
        
        let _r2 = &mut s;
    }

    {
        let mut s = String::from("hello");

        let _r1 = &s; // no problem
        let _r2 = &s; // no problem
        let _r3 = &mut s; // BIG PROBLEM; this mut ref makes the following code illegal

        // println!("{}, {}, {}", _r1, _r2, _r3)
    }

    {
        // However, using immutable and mutable refs like this is fine!

        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // r1 and r2 are no longer used after this point

        let r3 = &mut s; // no problem
        println!("{}", r3);
    }

    {
        // dangle();
        no_dangle();
    }
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(string: &mut String) {
    string.push_str(", world!");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
     // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// RULES: 
// - At any given time, you can have either one mutable reference or any number of immutable references.
// - References must always be valid.
