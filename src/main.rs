mod do_calc;
use std::env;
fn main() {
    let mut line = String::new();
    println!("Enter you calculation:");
    std::io::stdin().read_line(&mut line);
    do_calc::calc(&line);
}
