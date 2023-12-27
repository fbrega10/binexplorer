use std::collections::HashMap;

pub struct Decexplorer {
    pub value: String,
}

impl Decexplorer {
    pub fn from(s: String) -> Self {
        Decexplorer { value: s }
    }

    pub fn dec_to_bin(&self) -> String {
        const TWO: i32 = 2;
        const ZERO: i32 = 0;
        let mut self_container: i32 = self.value.parse().expect("failed to read struct number");
        let mut diff: i32 = 0;
        let mut final_string = String::new();
        let one: String = String::from("1");
        let zero: String = String::from("0");
        while self_container > ZERO {
            diff = self_container % TWO;
            self_container /= TWO;
            match diff {
                0 => final_string.push_str(&zero),
                1 => final_string.push_str(&one),
                _ => println!("something went wrong"),
            }
        }
        reverse_string(final_string)
    }

    pub fn bin_to_dec(&self) -> i32 {
        let mut power_index: u32 = 0;
        let mut result: i32 = 0;
        let two: i32 = 2;
        for letter in self.value.chars() {
            match letter {
                '1' => {
                    result += two.pow(power_index);
                    power_index += 1;
                }
                '0' => power_index += 1,
                _ => panic!("unable to parse number, not a binary format"),
            }
        }
        result
    }

    pub fn bin_to_hex(&self) -> String {
        let mut tmp = String::new();
        let mut hex = String::new();
        let _four: u32 = 4;
        let final_char_position: u32 = self.value.len() as u32;
        let mut index: u32 = 1;

        for letter in self.value.chars() {
            if index == 4 && index < final_char_position {
                tmp.push_str(&letter.to_string());
                index = 1;
                hex.push_str(&hex_converter(&reverse_string(tmp.clone())));
                tmp = String::new();
            }
            if index < 4 && index < final_char_position {
                tmp.push_str(&letter.to_string());
                index += 1;
            } else if index < 4 && index == final_char_position {
                tmp.push_str(&letter.to_string());
                let no_zeroes = 4 - tmp.len() as u32;
                match no_zeroes {
                    3 => {
                        let temp = String::from("000");
                        tmp = temp + &tmp.clone();
                    }
                    2 => {
                        let temp = String::from("00");
                        tmp = temp + &tmp.clone();
                    }
                    1 => {
                        let temp = String::from("0");
                        tmp = temp + &tmp.clone();
                    }
                    _ => panic!("something went wrong, while encoding to binary"),
                }
                hex.push_str(&hex_converter(&tmp.clone()));
            }
        }
        hex
    }

    pub fn hex_to_dec(&self) -> String {
        const ONE: u32 = 1;
        let mut sum = 0;
        let mut index = 0;
        let hex: u32 = 16;
        for letter in self.value.chars() {
            sum += letter
                .to_digit(hex)
                .expect("incorrect hexadecimal number parsing")
                * (hex.pow(index));
            index += ONE;
        }
        sum.to_string()
    }

    pub fn dec_to_hex(&self) -> String {
        let x = &self.dec_to_bin();
        let a = Decexplorer {
            value: x.to_string(),
        };
        a.bin_to_hex()
    }

    pub fn bin_to_oct(&self) -> String {
        const OCTAL_VALUE: usize = 3;
        reverse_string(
            split_by_nth(reverse_string((*self.value).to_string()), OCTAL_VALUE)
                .into_iter()
                .map(|c| oct_converter(&reverse_string(c)))
                .collect(),
        )
    }
}

pub fn split_by_nth(s: String, splitter: usize) -> Vec<String> {
    //s is the string we're trying to convert from binary to octal, the conversion is done by
    //groups of 3 bits each starting from the left side to the right.
    //In case the remaining bits would be less than 3, then a padding is applied to the left adding zeroes
    //The splitter is the n number representing the number of bits to be grouped together
    let mut s = s;
    const MINIMAL_LENGTH: usize = 1;
    const ZERO_BIT: &str = "0";
    const ZERO_VALUE: usize = 0;
    assert!(s.len() >= MINIMAL_LENGTH);
    let mut vector: Vec<String> = Vec::new();
    while s.len() > splitter {
        let (chunk, rest) = s.split_at(splitter);
        vector.push(String::from(chunk));
        s = String::from(rest);
    }
    if s.len() > ZERO_VALUE {
        for _ in ZERO_VALUE..s.len() - MINIMAL_LENGTH {
            s.push_str(ZERO_BIT);
        }
        vector.push(s);
    }
    vector
}

pub fn reverse_string(s: String) -> String {
    s.chars().rev().collect()
}

pub fn hex_converter(s1: &String) -> String {
    assert_eq!(4, s1.len());
    let mut map: HashMap<String, String> = HashMap::with_capacity(16);
    map.insert(String::from("0000"), String::from("0".to_string()));
    map.insert(String::from("0001"), String::from("1".to_string()));
    map.insert(String::from("0010"), String::from("2".to_string()));
    map.insert(String::from("0011"), String::from("3".to_string()));
    map.insert(String::from("0100"), String::from("4".to_string()));
    map.insert(String::from("0101"), String::from("5".to_string()));
    map.insert(String::from("0110"), String::from("6".to_string()));
    map.insert(String::from("0111"), String::from("7".to_string()));
    map.insert(String::from("1000"), String::from("8".to_string()));
    map.insert(String::from("1001"), String::from("9".to_string()));
    map.insert(String::from("1010"), String::from("A".to_string()));
    map.insert(String::from("1011"), String::from("B".to_string()));
    map.insert(String::from("1100"), String::from("C".to_string()));
    map.insert(String::from("1101"), String::from("D".to_string()));
    map.insert(String::from("1110"), String::from("E".to_string()));
    map.insert(String::from("1111"), String::from("F".to_string()));
    map.get(s1)
        .expect("error occurred transforming binary to hex")
        .to_string()
}

pub fn oct_converter(s1: &String) -> String {
    assert_eq!(3, s1.len());
    let mut map: HashMap<String, String> = HashMap::with_capacity(8);
    map.insert(String::from("000"), String::from("0".to_string()));
    map.insert(String::from("001"), String::from("1".to_string()));
    map.insert(String::from("010"), String::from("2".to_string()));
    map.insert(String::from("011"), String::from("3".to_string()));
    map.insert(String::from("100"), String::from("4".to_string()));
    map.insert(String::from("101"), String::from("5".to_string()));
    map.insert(String::from("110"), String::from("6".to_string()));
    map.insert(String::from("111"), String::from("7".to_string()));

    map.get(s1)
        .expect("error occurred transforming binary to hex")
        .to_string()
}
