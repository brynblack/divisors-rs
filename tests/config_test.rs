use divisors_rs::algorithms;
use divisors_rs::Config;

#[test]
fn invalid_syntax() {
    let args: Vec<String> = vec![
        "path_to_the_program".to_string(),
        "thiswillbreak123".to_string(),
    ];
    assert_eq!(Config::new(&args).is_err(), true);

    let args: Vec<String> = vec![
        "path_to_the_program".to_string(),
        "123123aaa123".to_string(),
    ];
    assert_eq!(Config::new(&args).is_err(), true);

    let args: Vec<String> = vec!["path_to_the_program".to_string(), "-100".to_string()];
    assert_eq!(Config::new(&args).is_err(), true);
}

#[test]
fn large_integral() {
    let args: Vec<String> = vec![
        "path_to_the_program".to_string(),
        "18446744073709551616".to_string(),
    ];
    assert_eq!(Config::new(&args).is_err(), true);

    let args: Vec<String> = vec![
        "path_to_the_program".to_string(),
        "1844674407370955161662137125637213".to_string(),
    ];
    assert_eq!(Config::new(&args).is_err(), true);
}

#[test]
fn no_arguments() {
    let args: Vec<String> = vec!["path_to_the_program".to_string()];
    assert_eq!(Config::new(&args).is_err(), true);
}

#[test]
fn correct_output() {
    assert_eq!(algorithms::trial_division(1), vec![1]);
    assert_eq!(algorithms::trial_division(10), vec![1, 2, 5, 10]);
    assert_eq!(algorithms::trial_division(12345), vec![1, 3, 5, 15, 823, 2469, 4115, 12345]);
    assert_eq!(algorithms::trial_division(18446744073709551), vec![1, 3, 6148914691236517, 18446744073709551]);
}
