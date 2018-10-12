extern crate lepton;

use std::env::args;
use std::f32::consts::PI;

use lepton::*;
use lepton::skew::*;
use lepton::rotate::*;

fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();        // Unused: executable name.
    let file_in = args.next().expect("Expected an input file");
    let file_out = args.next().expect("Expected an output file");

    println!("Attempting to open \"{}\"", file_in);
    let pix = Pix::read(file_in).expect("Could not read image");
    let mut pix = pix.convert(Conversion::OneBit { threshold: 130, factor: None }).unwrap();

    let skew = pix.find_skew().expect("Could not find skew");

    println!("Determined angle {} and confidence {}", skew.degrees, skew.confidence);
    let mut pix = pix.rotate(skew.degrees * PI / 180., Rotation::AREA_MAP, Background::WHITE, None).expect("Could not rotate");

    pix.write(file_out, Format::PNG).expect("Could not write image");
    println!("Done");
}