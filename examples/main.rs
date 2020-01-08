use fst_sys::fstReaderOpen;
use std::ffi::{CStr, CString};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::env::args().nth(1).expect("Need 1 argument");
    let reader = fst_sys::FstReader::from_file(&input).expect("unable to open reader");
    println!("{:?}", reader);
    println!("end_time = {}\nfile_type = {:?}", reader.end_time(), reader.file_type());
    Ok(())
}
