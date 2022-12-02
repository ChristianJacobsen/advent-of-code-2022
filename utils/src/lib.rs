use std::{env::args, error::Error, fs::read_to_string};

pub fn read_input_file() -> Result<String, Box<dyn Error>> {
    let file_path = args().nth(1).ok_or("missing input file")?;
    let file_content = read_to_string(file_path)?;
    Ok(file_content)
}
