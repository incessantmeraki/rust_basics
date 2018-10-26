fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    let arr2 = arr;
    println!("first {:?}", arr);
    println!("second {:?}", arr2);
    arr[0] = 88;
    println!("first {:?}", arr);
    println!("second {:?}", arr2);
}
