fn main() {
    let value = 10;
    if value % 4 == 0 {
        println!("The number is divisible by 4");
    } else if value % 3 == 0 {
        println!("The number is divisible by 3");
    } else if value % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number isn't divisible by 4, 3, or 2")
    }

    // -----

    let condition = true;
    let value = if condition {
        5
    } else {
        10
    };
    println!("The value is: {}", value);
    
    // -----

    // loop {
    //     println!("again!");
    // }

    // -----

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    // -----

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!");

    // -----

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    // -----

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // -----

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
