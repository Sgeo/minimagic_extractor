use anyhow::{Context, Result, bail};
use std::io::{Read};
use std::fs::File;
use regex::bytes::Regex;
use byteorder::{ByteOrder, BigEndian, LittleEndian};

fn main() -> Result<()> {
    let BMCF = Regex::new(r"BMCF")?;

    let filename = std::env::args().nth(1).context("Need an argument!")?;
    let mut input_file = File::open(&filename).context("Unable to open file!")?;
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;
    
    // First BMCF always seems to be a dummy
    for (index, bmcf_match) in BMCF.find_iter(&input_data).skip(1).enumerate() {
        let bmcf_location = bmcf_match.start();
        let bm_header_size = BigEndian::read_u32(&input_data[bmcf_location+4..bmcf_location+8]) as usize;
        let flash_location = bmcf_location + bm_header_size;
        if &input_data[flash_location..flash_location + 3] != b"FWS" {
            println!("BMCF does not contain valid Flash!");
            continue;
        }
        let flash_size = LittleEndian::read_u32(&input_data[flash_location+4..flash_location+8]) as usize;
        let end_data_location = bmcf_location + bm_header_size + flash_size;
        let mut output_data = &input_data[bmcf_location..end_data_location];
        let output_filename = format!("{}.{}.bm", filename, index);
        let mut output_file = File::create(output_filename)?;
        std::io::copy(&mut output_data, &mut output_file)?;
    }

    Ok(())
}
