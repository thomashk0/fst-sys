pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::env::args().nth(1).expect("Need 1 argument");
    let reader = fst_sys::FstReader::from_file(&input, false).expect("unable to open reader");
    println!("{:?}", reader);
    println!("end_time = {}\nfile_type = {:?}", reader.end_time(), reader.file_type());
    let n = reader.iter_block();
    println!("block iteration = {}", n);
    Ok(())
}
