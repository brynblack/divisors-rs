use std::error::Error;
use std::num;

pub struct Config {
    pub integral: u64,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        // Check if an insufficient number of arguments were provided
        if args.len() < 2 {
            return Err("Usage: divisors <integral>".into());
        }

        // Parse the 2nd argument to an integral
        let integral: u64 = match args[1].parse() {
            Ok(num) => num,
            Err(err) => {
                return match err.kind() {
                    num::IntErrorKind::InvalidDigit => {
                        Err("Argument provided is not a positive integral".into())
                    }
                    num::IntErrorKind::PosOverflow => Err("Integral provided is too large".into()),
                    _ => Err(err.into()),
                };
            }
        };

        Ok(Config { integral })
    }
}

pub mod algos {
    pub fn trial_division(n: u64) -> Vec<u64> {
        let mut divisors: Vec<u64> = Vec::new();
        let limit = (n as f64).sqrt() as u64;
        for i in 1..=limit {
            if n % i == 0 {
                divisors.push(i);
                if n / i != i {
                    divisors.push(n / i);
                }
            }
        }
        divisors.sort_unstable();
        divisors
    }
}
