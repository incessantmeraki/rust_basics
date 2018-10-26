fn swap(a: &mut i32, b: &mut i32) {
    let t  = *a;
    *a = *b;
    *b = t;
}


fn main() {
    let mut a = 4;
    let mut b = 5;
    println!("Before a:{} b:{}",a,b);
    swap(&mut a, &mut b);
    println!("After a:{} b:{}",a,b);
}
