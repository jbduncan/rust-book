use std::collections::HashMap;
use unicode_segmentation::{GraphemeCursor, UnicodeSegmentation};

fn main() {
    // Vectors
    {
        let v: Vec<i32> = Vec::new();
        println!("v = {:?}", v);
        let v = vec![1, 2, 3];
        println!("v = {:?}", v);

        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        println!("v = {:?}", v);

        // ---

        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element"),
        }

        // ---

        let v = vec![1, 2, 3, 4, 5];
        // let does_not_exist = &v[100]; // This line would panic
        let does_not_exist = v.get(100);

        // ---

        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        // v.push(6); // This won't compile because we're borrowing mutably here and immutably in `first`
        println!("The first element is {}", first);

        // ---

        let v: Vec<i32> = (0..10).collect();
        println!("v = {:?}", v);
    }

    // Strings
    {
        let mut s = String::new(); // empty String

        let data = "initial contents"; // non-empty &str, compiled into the binary
        let s = data.to_string(); // String initialized from the &str above
        let s = String::from(data); // Equivalent to .to_string(); choose whichever you like!

        // Strings in Rust are UTF-8 encoded, so we can include any text that can be encoded as such.
        let _hello = String::from("السلام عليكم");
        let _hello = String::from("Dobrý den");
        let _hello = String::from("Hello");
        let _hello = String::from("שָׁלוֹם");
        let _hello = String::from("नमस्ते");
        let _hello = String::from("こんにちは");
        let _hello = String::from("안녕하세요");
        let _hello = String::from("你好");
        let _hello = String::from("Olá");
        let _hello = String::from("Здравствуйте");
        let _hello = String::from("Hola");
        let hello = String::from("👋");
        println!("{}", hello);

        let mut s = "foo".to_string();
        s.push_str("bar");
        println!("{}", s);

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2); // .push_str doesn't take ownership of s2
        println!("s2 is {}", s2);

        let mut s = "lo".to_string();
        s.push('l');
        println!("{}", s);

        let s1 = "hello, ".to_string();
        let s2 = "world".to_string();
        let s3 = s1 + &s2; // note: s1 has been moved here and can no longer be used
                           // The '+' operator takes s1, mutably adds the contents of s2 to itself, and returns a new un-owned string s3.
        println!("{}", s3);

        let s1 = "tic".to_string();
        let s2 = "tac".to_string();
        let s3 = "toe".to_string();
        let s = format!("{}-{}-{}", s1, s2, s3);
        // Same as s1 + &s2 + &s3, but doesn't take ownership of any of s1, s2 and s3.
        // This means we can continue to use s1, s2 and s3 elsewhere in the code.
        println!("{}", s);

        // Strings in Rust can't be indexed like the following, because Rust acknowledges that
        // strings are tricky data structures and that not everyone wants the nth byte instead
        // of, say, the nth grapheme ("letter").
        //
        // let h = s[0];
        //
        // But if you really, _really_ want to view the nth byte, you can use a String slice:
        //
        let h = &s[0..1]; // t (first letter of tic-tac-toe)
        println!("h = {}", h);
        // Just note it will panic if the byte doesn't form a whole letter on its own

        // Returns number of bytes (not letters):
        let len = "Hola".len(); // 4
        println!("\"Hola\".len() = {}", len);

        let len = "Здравствуйте".len(); // 24, rather than 12
        println!("\"Здравствуйте\".len() = {}", len);

        println!("{}", "नमस्ते"); // prints whole string
                                // println!("{}", &"नमस्ते"[0..1]); // this panics because the first letter in this word can't be represented in a single byte

        // Iteration methods:
        for c in "नमस्ते".chars() {
            println!("{}", c); // prints each scalar value (so not quite "letters" in the human sense...)
        }

        for b in "नमस्ते".bytes() {
            println!("{}", b); // prints each byte
        }

        use unicode_segmentation::UnicodeSegmentation; // a third-party crate
        for g in UnicodeSegmentation::graphemes("नमस्ते", /* is_extended= */ true) {
            println!("{}", g); // prints each grapheme (letter)
        }
    }

    // Hash maps
    {
        let mut scores = HashMap::new();

        scores.insert("Blue".to_string(), 10);
        scores.insert("Green".to_string(), 50);
        println!("{:?}", scores);

        // ---

        let teams = vec!["Blue".to_string(), "Green".to_string()];
        let scores = vec![10, 50];
        let scores = teams.iter().zip(scores.iter()).collect::<HashMap<_, _>>();
        println!("{:?}", scores);

        // ---

        let field_name = String::from("Favourite colour");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // This code is not valid because String doesn't implement Copy, and so field_name is owned by map.
        // println!("{}", field_name);

        // ---

        let mut scores = HashMap::new();
        scores.insert("Blue".to_string(), 10);
        scores.insert("Green".to_string(), 50);

        let score = scores.get(&"Blue".to_string()); // Can be shortened to 'scores.get("Blue")' in this case
        println!("{:?}", score);

        // ---

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        // ---

        println!(
            "Yellow: {}",
            scores.entry("Yellow".to_string()).or_insert(50)
        );
        println!("Blue: {}", scores.entry("Blue".to_string()).or_insert(42));
        println!("{:?}", scores);

        // ---

        let text = "Hello world, wonderful world!";

        let mut multiset = HashMap::new();
        for word in text.unicode_words() {
            let count: &mut i32 = multiset.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", multiset);

        // Alternative using functional folding
        let multiset = text.unicode_words().fold(HashMap::new(), |mut map, word| {
            let count: &mut i32 = map.entry(word).or_insert(0);
            *count += 1;
            map
        });
        println!("{:?}", multiset);
    }

    // Exercises
    {
        // 1. Given a list of integers, use a vector and return the mean (the average value), median
        // (when sorted, the value in the middle position), and mode (the value that occurs most
        // often; a hash map will be helpful here) of the list.
        let numbers = vec![1, 1, 1, 1, 3, 3];

        let average = average(&numbers);
        let median = median(&numbers);
        let mode = mode(&numbers);
        println!(
            "average = {:?}, median = {:?}, mode = {:?}",
            average, median, mode
        );

        fn average(numbers: &[i32]) -> f32 {
            let sum = numbers.iter().sum::<i32>() as f32;
            let size = numbers.len();
            match size {
                positive if (positive > 0) => sum / (size as f32),
                _ => 0.0,
            }
        }

        fn median(numbers: &[i32]) -> Option<f32> {
            let mut numbers = numbers.to_vec();
            numbers.sort_unstable();
            match numbers.len() {
                0 => None,
                even if (even % 2 == 0) => {
                    let middle_upper = numbers.len() / 2;
                    let middle_lower = middle_upper - 1;
                    Some(((numbers[middle_lower] + numbers[middle_upper]) as f32 / 2.0))
                }
                _ => Some(numbers[numbers.len() / 2] as f32),
            }
        }

        fn mode(numbers: &[i32]) -> Option<i32> {
            // TODO: Find a way of turning this "multiset" functionality into a generic function
            //       that can accept slices of any type.
            let multiset = numbers.iter().fold(HashMap::new(), |mut map, word| {
                let count: &mut i32 = map.entry(word).or_insert(0);
                *count += 1;
                map
            });
            multiset
                .into_iter()
                .max_by_key(|&(_, count)| count)
                .map(|(value, _)| *value)
        }
    }
    {
        // 2. Convert strings to pig latin. The first consonant of each word is moved to the end of
        // the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel
        // have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the
        // details about UTF-8 encoding!
        for word in &["first", "apple", "human", "chimp", "uh", "onomatopoeia"] {
            println!("{:?}", to_pig_latin(&word))
        }

        fn to_pig_latin(string: &str) -> String {
            // TODO: Find a way of turning these asserts into `Result` return values, where in the
            //       error case they contain a specialised error enum type with two values: "not all
            //       characters are ASCII alphabetic" and "number of characters is less than 2".
            assert!(string.chars().all(|c| c.is_ascii_alphabetic()));
            assert!(string.chars().count() >= 2);
            let mut chars = string.chars();
            let first_char = chars.next().unwrap();
            match first_char {
                c if is_vowel(c) => string.to_owned() + "-hay",
                _ => {
                    let other_chars = chars.as_str();
                    return other_chars.to_string() + "-" + &first_char.to_string() + "ay";
                }
            }
        }

        fn is_vowel(character: char) -> bool {
            VOWELS.contains(&character)
        }

        static VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    }
}
