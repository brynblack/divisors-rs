fn main()
{
    // Collect the supplied arguments.
    let args: Vec<String> = std::env::args().collect();

    // Create a new config.
    let config = Config::new(&args);

    // Print each divisor.
    for div in trial_division(config.integral).iter()
    {
        println!("{}", div);
    }
}

fn trial_division(n: u64) -> Vec<u64>
{
    let mut divisors = Vec::new();
    let limit = (n as f64).sqrt() as u64;
    for i in 1..=limit
    {
        if n % i == 0
        {
            divisors.push(i);
            if n / i != i { divisors.push(n / i); }
        }
    }
    divisors.sort_unstable();
    divisors
}


struct Config
{
    integral: u64,
}

impl Config
{
    fn new(args: &[String]) -> Config
    {
        if args.len() < 2
        {
            println!("Usage: divisors <integral>");
            std::process::exit(1);
        }

        let integral = &args[1];

        let integral: u64 = match integral.parse()
        {
            Ok(value) => value,
            Err(err) => match err.kind()
            {
                std::num::IntErrorKind::PosOverflow | std::num::IntErrorKind::NegOverflow =>
                {
                    println!("Error: Integral provided is too large");
                    std::process::exit(1);
                },
                std::num::IntErrorKind::InvalidDigit =>
                {
                    println!("Error: Argument provided is not an integral");
                    std::process::exit(1);
                },
                other_err =>
                {
                    panic!("Error: {:?}", other_err);
                },
            },
        };

        Config { integral }
    }
}
