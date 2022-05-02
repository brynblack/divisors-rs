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
    divisors.sort();
    return divisors;
}

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1
    {
        println!("Usage: divisors <integral>");
        std::process::exit(1);
    }
    for div in trial_division(args[1].parse::<u64>().unwrap()).iter()
    {
        println!("{}", div);
    }
}
