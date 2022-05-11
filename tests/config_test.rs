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
