fn main() {
    let mut myvec = Vec::new();
    myvec.push(1);
    myvec.push(2);
    myvec.push(3);

    let myslice = &myvec;

    println!("Contents of vector is {:?}", myvec);
    println!("Contents of slice is {:?}", myslice);
}
