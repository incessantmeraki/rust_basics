fn main() {
    let text = "the red fox and the lazy dog";
    let mut finalstring = String::new();
    for c in text.chars(){
        if c != ' ' {
            finalstring.push(c);
        }
    }
    println!("String after processing is {}", finalstring);

}
