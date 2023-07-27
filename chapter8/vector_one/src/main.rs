#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();

    let v1 = vec![1,2,3];


    println!("v = {:?}", v);
    println!("v1 = {:?}", v1);

    let mut v2 = Vec::new();

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);
    v2.push(-10);

    println!("v2 = {:?}", v2);

    {
        let v = vec![1,2,3,4];
        println!("v = {:?}", v);
    }

    let mut vv = vec![1, 2, 3, 4, 5];

    let first = &vv[0];

    // vv.push(6);

    println!("The first element is: {}", first);

    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
    }

    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
            println!("{}", i);
        }
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row = {:?}", row);
}
