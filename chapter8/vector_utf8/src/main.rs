fn main() {
    {
        let mut s = String::new();
    }

    {
        let data = "initial contents";
        let s = data.to_string();
        println!("data => {s}");
    }

    {
        let s = String::from("initial contents");
        println!("s ==> {s}");
    }

    {
        let mut s = String::from("foo");
        s.push_str(" bar");
        println!("s ==> {s}");
    }

    {
        let mut s1 = String::from("foo");
        let s2 = " bar";
        s1.push_str(s2);
        println!("s1 is {}", s1);
        println!("s2 is {}", s2);
    }

    {
        let mut s = String::from("lo");
        s.push('l');
        println!("s ==> {s}");
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;

        println!("s2 ==> {s2}");
        println!("s3 ==> {s3}");
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("s ==> {s}");
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("s ==> {s}");
        println!("s1 ==> {s1}");
        println!("s2 ==> {s2}");
        println!("s3 ==> {s3}");
    }
    // Rust 的字符串不支持索引
    // {
    //     let s1 = String::from("hello");
    //     let h = s1[0];
    //     println!("h ==> {h}");
    // }

    {
        let len = String::from("Hola").len();
        println!("len ==> {len}");

        let hello = "Здравствуйте";
        let len1 = hello.to_string().len();
        println!("len1 ==> {len1}");
    }

    {
        let hello = "Здравствуйте";
        // let answer = &hello[0];
        // println!("answer ==> {answer}");

        let s = &hello[0..4];
        println!("s ==> {s}");
    }

    {
        for c in "नमस्ते".chars() {
            println!("{}", c);
        }

        let len = String::from("नमस्ते").len();
        println!("len ==> {len}");

        for b in "नमस्ते".bytes() {
            println!("{}", b);
        }
    }

}
