fn main() {
    {
        #[derive(Debug)]
        enum UsState {
            Alaska,
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState)
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("Ooh, a quarter from {:?}!", state);
                    25
                },
            }
        }

        println!("value in cents = {}", value_in_cents(Coin::Quarter(UsState::Alaska)))
    }

    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                // Pattern matching and decomposition!
                Some(i) => Some(i + 1),
                None => None,
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }

    {
        let some_value = 0u8;
        match some_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            _ => () // On anything else, do nothing
        }
    }
}
