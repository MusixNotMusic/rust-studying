pub fn sum_of_n(n: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

pub fn sum_of_n2(n: i64) -> i64 {
     n * (n + 1) / 2
}