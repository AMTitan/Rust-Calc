mod do_calc;
use std::env;
fn main() {
    loop {
        let mut line = String::new();
        println!();
        println!("Enter you calculation:");
        std::io::stdin().read_line(&mut line);
        line = line.replace("\n", "");
        do_calc::calc(&line);
    }
}
