// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; //分号
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
