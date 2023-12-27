use binexplorer::decexplorer::Decexplorer;

#[test]
pub fn bin_to_octal_test1() {
    assert_eq!(
        Decexplorer::from(String::from("01001110")).bin_to_oct(),
        String::from("116")
    );
}

#[test]
pub fn bin_to_octal_test2() {
    assert_eq!(
        Decexplorer::from(String::from("01")).bin_to_oct(),
        String::from("1")
    );
}
