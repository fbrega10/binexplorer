use binexplorer::decexplorer::Decexplorer;

pub mod decexplorer;

fn main() {
    println!(
        "{}",
        Decexplorer::from(String::from("01001110")).bin_to_oct()
    );
}
