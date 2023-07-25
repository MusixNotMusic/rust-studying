#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;


    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    let rect = (30, 50);

    println!(
        "The area1 of the rectangle is {} square pixels.",
        area1(rect)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area2 of the rectangle is {} square pixels.",
        area2(&rect2)
    );

    println!("rect2 is {:?}", rect2);

    let scale = 2;
    let rect3 = Rectangle{
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
