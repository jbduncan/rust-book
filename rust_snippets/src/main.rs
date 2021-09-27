use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let text: &str = "Hello world, wonderful world! It's turtles all the way down.";

    let words = text.unicode_words(); // lazy

    let multiset =
        words
            .map(|word| word.to_lowercase()) //
            .fold(HashMap::new(), |mut map, word| {
                let count: &mut i32 = map.entry(word).or_insert(0);
                *count += 1;
                return map;
            });

    println!("words = {:?}", multiset)
}
