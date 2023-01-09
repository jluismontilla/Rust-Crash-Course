#![deny(clippy::all)]

fn foo(_i: &i32) {
    let _z = 42;
}
fn main() {
    let x = 5;
    let y = &x;

    println!("X = {}", x);
    println!("Y = {}", y);

    foo(y);
}
