use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    {
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        generate_workout(simulated_user_specified_value, simulated_random_number);
    }

    // -----

    {
        let x = 4;

        let equal_to_x = |z| z == x;

        // The following doesn't compile, because functions can't use
        // variables from their outer scope, only closures can.
        //
        // fn equal_to_x(z: i32) -> bool {
        //     z == x
        // }

        println!("{}", x);

        let y = 4;

        assert!(equal_to_x(y));
    }

    {
        let x = vec![1, 2, 3];

        // This closure takes ownership of x, so x can't be used anywhere else. This causes the
        // closure to implicitly implement the `FnOnce` closure. See [1], FnOnce for a more detailed
        // discussion on this.
        let equal_to_x = move |z| z == x;

        // Because x is now owned by `equal_to_x`, it can't be printed here.
        // println!("{:?}", x);

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }
}

// We create a "Cacher" class here for demonstration purposes, to show how
// to do memoization with a closure and any input that is a reference and
// implements (Eq + Hash). It doesn't work for types that don't have a known
// size at compile-time.
//
// To memoize a closure that doesn't accept any inputs and just returns a
// single value (think Java's Guava's `Suppliers.memoize` or Kotlin's
// `by lazy`) in production code, use Rust's SyncLazy
// (https://doc.rust-lang.org/std/lazy/struct.SyncLazy.html)
// or a crate like "lazy_static" or "once_cell". (Although note that at
// the time of writing, these two crates use `unsafe` code in small amounts.)
//
// To memoize a closure that accepts an input and returns an output from
// that input (i.e. key-value-based caching) in production code, consider
// a crate like "cached" or "lru".
struct Cacher<'a, T, F>
where
    &'a T: Eq + Hash,
    F: Fn(&'a T) -> &'a T, // [1]
{
    calculation: F,
    input_to_value: HashMap<&'a T, &'a T>,
}

impl<'a, T, F> Cacher<'a, T, F>
where
    &'a T: Eq + Hash,
    F: Fn(&'a T) -> &'a T, // [1]
{
    fn new(calculation: F) -> Cacher<'a, T, F> {
        Cacher {
            calculation,
            input_to_value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &'a T) -> &'a T {
        match self.input_to_value.get(&arg) {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.input_to_value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} push-ups!",
            expensive_closure.value(&intensity)
        );
        println!("Next, do {} sit-ups!", expensive_closure.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(&intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_single_value() {
        let mut c = Cacher::new(|a| a);

        let v = c.value(&1);

        assert_eq!(v, &1);
    }

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(&1);
        let v2 = c.value(&2);

        assert_eq!(v1, &1);
        assert_eq!(v2, &2);
    }
}

// [1] Fn, FnMut and FnOnce are all traits that specify the behaviour of a closure.
//     - Fn: borrows values from the environment [2] immutably.
//     - FmMut: can change the environment because it mutably borrows values.
//     - FnOnce: takes ownership of and thus consumes variables it uses from the environment.
//       The Once part of the name represents the fact that the closure can't take ownership of the
//       same variables twice, so it can only be called once.
//
// [2] The "environment" is the outer scope that a closure sits in. Closures can use variables from
//     the environment without having to specify any input parameters.