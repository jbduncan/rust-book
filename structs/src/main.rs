fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("abc@def.com");

    println!("{:?}", user1);

    println!("{:?}", build_user(String::from("foo@abc.net"), String::from("another_user")));

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // active: user1.active,
        // sign_in_count: user1.sign_in_count
        ..user1
    };

    println!("{:#?}", user2);

    #[derive(Debug)] struct Colour(i32, i32, i32);
    #[derive(Debug)] struct Point(i32, i32, i32);

    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", black);
    println!("{:#?}", origin);

    {
        let width = 30;
        let height = 50;

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width, height));

        fn area(width: u32, height: u32) -> u32 {
            width * height
        }
    }

    {
        let rectangle = (30, 50);

        println!(
            "The area of the rectangle is {} square pixels.",
            area(rectangle));

        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }
    }

    {
        let rectangle = Rectangle { width: 30, height: 50 };

        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rectangle));

        struct Rectangle {
            width: u32,
            height: u32
        }

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }
    }

    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width >= other.width && self.height >= other.height
            }

            // This is not a method, but an associated function. 
            // Useful for things like "constructors" (or "static method factories" from Java).
            fn square(size: u32) -> Rectangle {
                Rectangle { width: size, height: size }
            }
        }

        let rectangle = Rectangle { width: 30, height: 50 };
        println!("The area of the rectangle is {} square pixels.", rectangle.area());

        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        let rect3 = Rectangle { width: 60, height: 45 };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

        println!("The square of {} is {}.", 12, Rectangle::square(12).area());
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
