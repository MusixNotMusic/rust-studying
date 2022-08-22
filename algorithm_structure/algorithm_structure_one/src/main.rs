// use static_func_call::static_func_call;
mod sum_of_n;
use sum_of_n::sum_of_n2;
use std::time::SystemTime;

fn main() {
    for _i in 0..5 {
        let now = SystemTime::now();
        sum_of_n2(1000000);
        let duration = now.elapsed().unwrap();
        let time = duration.as_millis();
        println!("func used {time} ms");
    }
}
