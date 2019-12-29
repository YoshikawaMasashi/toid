use std::fs::File;
use std::io::Read;

use toid::data::sf2;

fn main() {
    let mut f = File::open("../florestan-subset.sf2").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    let buffer = buffer.as_slice();

    /*
    let sf2_data = sf2::SF2::parse(buffer);
    println!("{}", sf2_data);
    */
    let own_sf2 = sf2::own::SF2::parse(buffer);
}
