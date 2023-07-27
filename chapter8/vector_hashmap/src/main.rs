use std::collections::HashMap;

fn main() {
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        println!("scores ==> {:?}", scores);
    }

    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![100, 50];

        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("scores ==> {:?}", scores);
    }

    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        println!("map ==> {:?}", map);

    }
}
