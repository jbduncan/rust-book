fn main() {
    {
        enum IpAddrKind {
            V4,
            V6,
        }

        let _four = IpAddrKind::V4;
        let _six = IpAddrKind::V6;

        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let _home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let _loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1")
        };
    }

    {
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let _home = IpAddr::V4(String::from("127.0.0.1"));
        let _loopback = IpAddr::V6(String::from("::1"));
    }

    {
        // Each variant in an enum can accept different types and amounts of data!
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let _home = IpAddr::V4(127, 0, 0, 1);
        let _loopback = IpAddr::V6(String::from("::1"));
    }

    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        // equivalent to following (except grouped under same `Message` type)... 
        struct QuitMessage; // unit struct
        struct MoveMessage {
            x: i32,
            y: i32,
        }
        struct WriteMessage(String); // tuple struct
        struct ChangeColorMessage(i32, i32, i32); // tuple struct

        // Enums can also have methods
        impl Message {
            fn call(&self) {
                // method body would go here
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();
    }

    {
        // Example of using the `Option` enum built into the standard library.
        // (Equivalent to null in other languages.)

        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<i32> = None;
    }
}
