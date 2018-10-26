fn main() {
    let vals = [1, 2, 3, 4, 5]; 
    let slice = &vals;

    for i in slice.windows(2) {
        println!("window {:?}", i);
    }

    for i in slice.chunks(2) {
        println!("Chunks {:?}", i);
    }
    println!("Iterator of vals {:?}", vals.iter());
}
