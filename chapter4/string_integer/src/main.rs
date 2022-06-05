fn main() {
    let s = String::from("hello");
    let s1 = s.clone();

    takes_ownership(s);

    println!("main ==> {}", s1);

    let i = 5;

    make_copy(i);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}
