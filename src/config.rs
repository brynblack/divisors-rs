//! A module for parsing command line arguments into a configuration.
use std::error::Error;
use std::num;

/// A struct to store the configuration.
pub struct Config {
    pub integral: u64,
}

impl Config {
    /// Returns a new config with the supplied arguments.
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        // Check if an insufficient number of arguments were provided
        if args.len() < 2 {
            return Err("Usage: divisors <integral>".into());
        }

        // Parse the 2nd argument into an integral
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
