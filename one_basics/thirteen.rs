fn main() {
    let string = "this is a string";
    let mut newstring = string.to_string();
    newstring.push('X');
    println!("original string {}", string);
    println!("mutable string {}", newstring);
}
