fn main() {
    let arr = [1, 2, 3, 4, 5];
    let optval = arr.get(1).unwrap();
    println!("Second value is {}", optval);
}
