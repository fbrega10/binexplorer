use binexplorer::decexplorer::Decexplorer;

pub mod decexplorer;

fn main() {
    println!(
        "converting 01001110 to octal -> {}",
        Decexplorer::from(String::from("01001110")).bin_to_oct()
    );
    println!(
        "converting 01010101010 to octal -> {}",
        Decexplorer::from(String::from("01010101010")).bin_to_oct()
    );

    println!(
        "converting 01 to octal -> {}",
        Decexplorer::from(String::from("01")).bin_to_oct()
    );

    println!(
        "converting 7 to bin -> {}",
        Decexplorer::from(String::from("7")).dec_to_bin()
    );
}
