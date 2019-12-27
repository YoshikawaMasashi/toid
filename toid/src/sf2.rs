use std::fs::File;
use std::io::Read;

use toid::data::riff;

fn main() {
    let mut f = File::open("../SGM-V2.01.sf2").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    let buffer = buffer.as_slice();

    let chank = riff::RiffChank::parse(buffer);
    chank.print();
}
