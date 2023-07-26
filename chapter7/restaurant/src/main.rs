// #[derive(Debug)] // use struct enum union

use restaurant::front_of_house::hosting::add_to_waitlist;
use restaurant::eat_at_restaurant_3;
use std::collections::HashMap;
use rand::Rng;

fn main() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();

    eat_at_restaurant_3();

    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("map -> {:?}", map);

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret_number -> {}", secret_number);
}