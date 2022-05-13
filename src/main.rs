use std::env;
use std::process;

use divisors_rs::algorithms;
use divisors_rs::Config;

fn main() {
    // Collect provided arguments into vector
    let args: Vec<String> = env::args().collect();

    // Create a new config
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    // Print out each divisor of the given integral
    algorithms::trial_division(config.integral)
        .iter()
        .for_each(|div| {
            println!("{}", div);
        });
}
