use binexplorer::decexplorer::Decexplorer;

#[test]
pub fn bin_to_dec_test1() {
    assert_eq!(
        Decexplorer::from(String::from("01001110")).bin_to_dec(),
        78.to_string()
    );
}

#[test]
pub fn bin_to_dec_test2() {
    assert_eq!(
        Decexplorer::from(String::from("010")).bin_to_dec(),
        2.to_string()
    );
}

#[test]
pub fn bin_to_dec_test3() {
    assert_eq!(
        Decexplorer::from(String::from("100000")).bin_to_dec(),
        32.to_string()
    );
}

#[test]
pub fn bin_to_dec_test4() {
    assert_eq!(
        Decexplorer::from(String::from("1000000000")).bin_to_dec(),
        512.to_string()
    );
}
