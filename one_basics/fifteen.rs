fn main() {
    let mut garbage = String::new();
    garbage.push('a');
    garbage.push('b');
    garbage.push('c');
    garbage.push('d');
    garbage.push('e');
    
    let sslice = &garbage[1..];
    garbage.pop();
    println!("Before: String slice {}", sslice);
    println!("After: String slice {}", sslice);

}
