fn sum(slice: &[i32]) -> i32 {
    let mut sum = 0;
    let length = slice.len();
    for i in 0..length {
        sum += slice[i];
    }
    sum
}


fn main() {
    let arr = [1, 2, 3, 4, 5];
    let result = sum(&arr);
    println!("Result of sum is {}", result);
    println!("Entire array is {:?}", arr);
    println!("First two values: {:?}", &arr[0..2])
}
