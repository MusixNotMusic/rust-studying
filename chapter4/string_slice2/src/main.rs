fn main() {
    let my_string = String::from("hello world");

    let word1 = first_word(&my_string[0..6]);
    let word2 = first_word(&my_string[..]);

    let word3 = first_word(&my_string);
    println!("word1 ==> {}, {}, {}", word1, word2, word3);

    let my_string_literal = "hello world";

    let word1 = first_word(&my_string_literal[0..6]);
    let word2 = first_word(&my_string_literal[..]);

    let word3 = first_word(&my_string_literal);

    println!("word2 ==> {}, {}, {}", word1, word2, word3);
}

fn first_word (s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}