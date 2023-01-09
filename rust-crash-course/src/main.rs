#![deny(clippy::all)]

fn foo() {
    let _z = 42;
    println!("Z = {}", _z);
}
fn main() {
    let x = 5;
    let y = &x;

    println!("X = {}", x);
    println!("Y = {}", y);

    foo();
}
