fn main() {
    let mut str = String::from("hello");

    println!("{str}");

    change_str(&mut str);

    println!("{str}");

    // 动态遍历 竞争
    let r1 = &mut str;

    // let r2 = &mut str;

    println!("{}", r1);

    // println!("{}", r2);

    // 动态遍历 竞争
    // println!("{}, {}", r1, r2);


    // 作用域
    {
        let r11 = &mut str;
        println!("r11 = {}", r11);
    }

    let r21 = &str;
    let r22 = &str;
    // let r23 = &mut str;

    // println!("{}, {}, and {}", r21, r22, r23);
    println!("{}, {}", r21, r22);

    // errot
    // let ref_str = dangle();

    // println!("dangle ref -> {}", ref_str);

    let str1 = no_dangle();
    println!("no_dangle str -> {}", str1);
}

fn change_str(str: &mut String) {
    str.push_str(" abcd");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
