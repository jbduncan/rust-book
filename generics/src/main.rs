use std::fmt::Display;

fn _largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn _largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: Ord>(list: &[T]) -> Option<&T> {
    list.iter().max()
}

fn main() {
    let numbers = [34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result.unwrap());

    let chars = ['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("The largest char is {}", result.unwrap());

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let _integer_and_float = Point { x: 5, y: 4.0 };
    println!("integer.x() = {}", integer.x());
    println!("float.distance_from_origin() = {}", float.distance_from_origin());
    let mixup = integer.mixup(float);
    println!("integer.mixup(float) = ({}, {})", mixup.x, mixup.y);

    Point { x: 5, y: 10 }.cmp_display();
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T: Display + PartialOrd> Point<T, T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
