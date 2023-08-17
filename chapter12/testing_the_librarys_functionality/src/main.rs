use std::env;
use std::process;
use testing_the_librarys_functionality::Config;
use testing_the_librarys_functionality::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    run(config);
}