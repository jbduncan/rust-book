pub mod hosting; // `pub mod` with no inner block brings `src/front_of_house/hosting.rs` into this file and re-exports it.

pub mod serving {
    fn take_order() {}

    pub fn serve_order() {}

    fn take_payment() {}
}