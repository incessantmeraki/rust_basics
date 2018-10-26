fn good_or_bad(val: bool) -> Result<i32, String> {
    if val {
        Ok(42)
    } else {
        Err("Bad".to_string())
    }
}

fn main() {
    let val = false;

    //println!("The determined thing is {}", val);
    match good_or_bad(val) {
        Ok(val) => println!("Value is {}", val),
        Err(msg) => println!("Error message {}", msg)
    }
}
