fn main() {
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("some_u8_value contains 3");
    }

    // The above is sugar for the following match expression:
    match some_u8_value {
        Some(3) => println!("some_u8_value contains 3"),
        _ => ()
    }
}
