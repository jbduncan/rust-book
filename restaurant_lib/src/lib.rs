use std::collections::HashMap;

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_wishlist() {}

//         fn seat_at_table() {}
//     }

//     pub mod serving {
//         fn take_order() {}

//         pub fn serve_order() {}

//         fn take_payment() {}
//     }
// }

mod front_of_house; // `mod` with no inner block brings the contents of src/front_of_house.rs into this scope in this file

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetiser {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting;
// The following is also valid:
// use front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_wishlist();

    // Relative path
    front_of_house::hosting::add_to_wishlist();

    // Using `use` import above
    hosting::add_to_wishlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let _order1 = back_of_house::Appetiser::Soup;
    let _order2 = back_of_house::Appetiser::Salad;

    let mut map = HashMap::new();
    map.insert(1, 2);
}

fn using_use() {
    // For conflicting names, both the following scopes' usages of `use`
    // are considered idiomatic. Use whichever you like!
    {
        use std::fmt;
        use std::io;

        fn function1() -> fmt::Result {
            Result::Ok(())
        }

        fn function2() -> io::Result<()> {
            Result::Ok(())
        }
    }

    {
        use std::fmt::Result;
        use std::io::Result as IoResult;

        fn function1() -> Result {
            Result::Ok(())
        }

        fn function2() -> IoResult<()> {
            IoResult::Ok(())
        }
    }
}

// Using `pub use` allows exposing a certain module or item to external callers,
// whilst also allowing us to `use` the item inside this module:
// pub use crate::front_of_house::hosting;

// Example usage, both internal and external: 
// hosting::add_to_waitlist();

// The advantage of `pub use` for library maintainers is they can organise code
// more logically depending on their use case.
// E.g. for a restaurant, it may make sense to only expose
// `front_of_house::hosting::*` functions to customers, but writing out
// `front_of_house::hosting::` is too verbose and slightly confusing; why would
// the customer be concerned about front_of_house vs. back_of_house?
// So using `pub use crate::front_of_house::hosting;` allows external users to just
// specify `use hosting::*`.
//
// In other words, we can write our code with one structure but expose it with a
// different structure!

fn long_use_lists() {
    {
        // What if our `use` list ends up quite long?
        // 
        // use std::io;
        // use std::cmp::Ordering;
        // --snip--
        //
        // Well, we can then use nested paths:
        use std::{cmp::Ordering, io};
    }

    {
        // What if we want to bring both `std::io` and `std::io::Write` into scope?
        use std::io::{self, Write};
    }

    {
        // What if we want to bring all items in a path into scope?
        use std::collections::*;
        // But be careful, this can lead to unreadable code!
        //
        // Prefer to use it for testing, where everything in a module would need to
        // be brought in for testing purposes.
    }
}
