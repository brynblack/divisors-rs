/// Implementation of the trial division algorithm.
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
