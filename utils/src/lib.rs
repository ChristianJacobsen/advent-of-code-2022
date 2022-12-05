use std::{env::args, error::Error, fs::read_to_string};

pub fn read_input_file() -> Result<String, Box<dyn Error>> {
    let file_path = args().nth(1).ok_or("missing input file")?;
    let file_content = read_to_string(file_path)?;
    Ok(file_content)
}

pub fn get_two_mut<T>(slice: &mut [T], index_1: usize, index_2: usize) -> Option<(&mut T, &mut T)> {
    if index_1 == index_2 {
        None
    } else if index_1 < index_2 {
        let (start, end) = slice.split_at_mut(index_2);
        Some((&mut start[index_1], &mut end[0]))
    } else {
        let (start, end) = slice.split_at_mut(index_1);
        Some((&mut end[0], &mut start[index_2]))
    }
}
