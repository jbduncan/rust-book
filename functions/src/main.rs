fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    let x = 5;
    let y = {
        // This is a block; blocks can return values!
        let x = 3;
        // This is an expression; they return a value, and they don't end with semi-colons.
        x + 1
    };
    println!("The value of y is: {}", y);

    // The following is invalid because `let` is a statement, not an expression, so it returns nothing.
    // let x = (let y = 5);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("Hello, again!");
    println!("The value of x and y are: {}, {}", x, y);
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}
