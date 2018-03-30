extern crate png_crop;

use std::env;
use std::io::{Read,Write};
use std::fs::File;
use png_crop::{crop,Rect};

fn main() {
    let in_path = env::args().nth(1).expect("Input file not given");
    let out_path = env::args().nth(2).expect("Output file not given");

    let mut in_file = File::open(in_path).expect("Unable to open input file");
    let mut input_bytes = vec!();
    in_file.read_to_end(&mut input_bytes).expect("Unable to read PNG file");

    let bounds = Rect::new(0, 0, 128, 128);
    let mut output_bytes = vec!();
    crop(input_bytes, &bounds, &mut output_bytes);

    let mut out_file = File::open(out_path).expect("Unable to open output file");
    out_file.write_all(&output_bytes).expect("Unable to write PNG file");
}
