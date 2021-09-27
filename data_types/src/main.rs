fn main() {
    let a: i8 = 127;
    let b: u8 = 255;
    // ...
    let c: u128 = 100_000_000_000_000_000_000;
    let d: isize = 123;
    let e: usize = 456;

    let hex: u64 = 0xff;
    let octal: u64 = 0o77;
    let binary: u64 = 0b1111_0000;
    let byte: u8 = b'A';

    let f: f64 = 2.0;
    let g: f32 = 3.0;

    5 + 10;
    95.5 - 10.4;
    4 * 30;
    34.2 / 3.4;
    43 % 5; // remainder
    5 + 10;

    let char = 'z';

    let char = 'z';
    let Z = 'Z';
    let emoji = '😻'; // chars in Rust are 4 bytes long, so they can't represent _all_ emojis, such as ✌️

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple; // destructuring through pattern matching
    let five_hundred = tuple.0; // accessing tuple element through index
    let six_point_four = tuple.1;
    let one = tuple.2;

    let array: [i32; 5] = [1, 2, 3, 4, 5]; // a fixed-size array of i32s with a length of 5
    let array = [3; 5]; // same as [3, 3, 3, 3, 3]
    let element = array[1]; // Access second element
}
