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

fn main()
{
    // Collect the supplied arguments.
    let args: Vec<String> = std::env::args().collect();

    // Check if the number of arguments is insufficient.
    if args.len() <= 1
    {
        panic!("Usage: divisors <integral>");
    }

    // Try to convert the second argument to an integral.
    let integral: u64 = match args[1].parse()
    {
        Ok(num) => num,
        Err(err) => match err.kind()
        {
            std::num::IntErrorKind::InvalidDigit => panic!("Error: Input provided is not an integral"),
            std::num::IntErrorKind::PosOverflow => panic!("Error: Integral provided is too large"),
            other_err => panic!("Error: {:?}", other_err),
        },
    };

    for div in trial_division(integral).iter()
    {
        println!("{}", div);
    }
}
