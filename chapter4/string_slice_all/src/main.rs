fn main() {
    let mut s = String::from("hello");

    let len = s.len();
    
    println!("s length = {len}");

    let word = first_word(&s);

    println!("the first word = {word}");

    s.clear();

    println!("the first s = {s}");

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

