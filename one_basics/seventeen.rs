fn main() {
    let sentence = "this is the entire sentence that i am after";
    let words: Vec<&str> = sentence.split_whitespace().collect();

    println!("Vector of words {:?}", words);
}
