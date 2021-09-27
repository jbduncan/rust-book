// This file is an "integration test" in Rust
// terminology. It's an alternative to writing
// tests in the main source file (see src/lib.rs
// for an example of this sort of testing, which
// Rust calls "unit testing".)
//
// This allows us to check the public APIs of our
// code and to check that two or more APIs
// "integrate" well together.

use tests; // The module we want to integration-test

#[test]
fn it_adds_two() {
    assert_eq!(4, tests::add_two(2));
}
