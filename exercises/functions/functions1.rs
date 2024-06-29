// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.

fn call_me (a: &str) {
    println!("Call me maybe, {}", a);
}

fn main() {
    let x = 3;
    println!("Number {}", x);
    call_me("Fima");
}
