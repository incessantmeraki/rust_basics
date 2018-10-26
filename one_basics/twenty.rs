use std::env;

fn main() {
    let n = env::args().nth(1).parse().expect("Integer value expected");

    let choice = match n % 2 {
        0 => "Even",
        _ =>  "Odd"
    };

    println!("Choice is {}", choice);
}
