use zephyr_common;

fn main() {
    let num: i32 = 10;
    println!("Hello, world!");
    println!("Hello, world! {num} plus one is {}!", zephyr_common::add_one(num));
}
