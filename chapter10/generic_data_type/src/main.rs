// #[derive(Debug)]

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

struct Point<T, U> {
    x: T,
    y: U,
}

struct Point1<T> {
    x: T,
    y: T,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    {
        let munber_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&munber_list);
        println!("The largest number is {}", result);
    }

    {
        let char_list = vec!['y', 'z', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }

    // {
    //     let number_list = vec![20, 512, 554, 1000];

    //     let result = largest(&number_list);
    //     println!("The largest number is {}", result);

    //     let char_list = vec!['y', 'y', 'd', 's', '+'];
    //     let result = largest(&char_list);
    //     println!("The largest char is {}", result);
    // }

    {
        let both_integer = Point{ x:5, y: 10 };
        let both_float = Point{ x: 1.0, y: 4.0 };
        let integer_and_float = Point{ x: 5, y: 4.0 };

        println!("both_integer = {}", both_integer.x);
        println!("both_float = {}", &both_float.x);
        println!("integer_and_float = {}", &integer_and_float.x);
    }

    {
        let p = Point { x: 5, y: 10 };

        println!("p.x = {}", p.x());
    }

    {
        let point = Point{ x: 3.0, y: 4.0 };
        println!("Point mod = {}", &point.x());
        println!("distance_from_origin = {}", &point.distance_from_origin());
    }

    {
        let p1 = Point { x: 5, y: 4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

    {
        let integer = Option_i32::Some(5);
        let float = Option_f64::Some(5.0);
    }
}
 