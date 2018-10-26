use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        println!("Argument vector is {:?}", args);
    }

    let second = env::args().nth(2).expect("Provide second argument");
    println!("Second argument is {}", second);
}
