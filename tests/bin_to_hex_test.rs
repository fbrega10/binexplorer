use binexplorer::decexplorer::Decexplorer;

#[test]
pub fn bin_to_hex_test1() {
    assert_eq!(
        Decexplorer::from(String::from("01001110")).bin_to_hex(),
        String::from("4E")
    );
}

#[test]
pub fn bin_to_hex_test2() {
    assert_eq!(
        Decexplorer::from(String::from("1010101001001110")).bin_to_hex(),
        String::from("AA4E")
    );
}

#[test]
pub fn bin_to_hex_test3() {
    assert_eq!(
        Decexplorer::from(String::from(
            "1101010100100111010101010010011101010101001001110010101001001110"
        ))
        .bin_to_hex(),
        String::from("D527552755272A4E")
    );
}

#[test]
pub fn bin_to_hex_test4() {
    assert_eq!(
        Decexplorer::from(String::from(
            "010011101010101001001110101010100111010101010010011100101010010011100101001001110010101001001110"
        ))
        .bin_to_hex(),
        String::from("4EAA4EAA755272A4E5272A4E")
    );
}
