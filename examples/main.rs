pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::env::args().nth(1).expect("Need 1 argument");
    let mut reader = fst_sys::FstReader::from_file(&input, false).expect("unable to open reader");
    println!("FstReader@{:?}", reader);
    println!("file_type = {:?}", reader.file_type());
    println!("version_string = {:?}", reader.version_string());
    println!("date_string = {:?}", reader.date_string());
    println!("end_time = {}", reader.end_time());
    Ok(())
}
