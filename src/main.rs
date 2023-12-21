pub mod decexplorer;
use crate::decexplorer::Decexplorer;

fn main() {
    //    let x = Decexplorer{
    //        value : String::from("45"),
    //    };
    //    println!("{}  to binary -> {}",x.value,  x.dec_to_bin());
    //    let x = Decexplorer{
    //value : String::from("010"),
    //};
    //println!("{}  to decimal -> {}",x.value,  x.bin_to_dec());
    //let x = Decexplorer{
    //value : String::from("11110010"),
    //};
    //println!("{}  to hex -> {}",x.value,  x.bin_to_hex());
    //let x = Decexplorer{
    //value : String::from("10"),
    //};
    //println!("{}  to hex -> {}",x.value,  x.bin_to_hex());

    //let x = Decexplorer{
    //value : String::from("FF"),
    // };
    //  println!("{}  to dec -> {}",x.value,  x.hex_to_dec());

    //    let x = Decexplorer{
    //       value : String::from("255"),
    //   };
    //    println!("{}  to hex -> {}",x.value,  x.dec_to_hex());

    //let x = Decexplorer{
    //value : String::from("101"),
    //};
    //println!("{}  to oct -> {}",x.value, x.bin_to_oct());

    let x = Decexplorer {
        value: String::from("10101"),
    };
    println!("{}  to oct -> {}", x.value, x.bin_to_oct());
}
