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

#[test]
pub fn bin_to_octal_test3() {
    assert_eq!(
        Decexplorer::from(String::from("101")).bin_to_oct(),
        String::from("5")
    );
}

#[test]
pub fn bin_to_octal_test4() {
    assert_eq!(
        Decexplorer::from(String::from("1010001010")).bin_to_oct(),
        String::from("1212")
    );
}

#[test]
pub fn bin_to_octal_test5() {
    assert_eq!(
        Decexplorer::from(String::from("11100011010")).bin_to_oct(),
        String::from("3432")
    );
}
