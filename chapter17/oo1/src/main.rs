use oo1::AveragedCollection;

fn main() {
    let mut ac = AveragedCollection{
        list: vec![1, 10, 100, 1000],
        average: 0.0
    };

    let num = 50;

    ac.add(num);
    println!("AveragedCollection list = {:?}, average = {}", ac.list, ac.average);
}
