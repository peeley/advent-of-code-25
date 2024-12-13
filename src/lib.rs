use std::{fs::File, io::Read};

pub fn get_day_input(day_num: i32) -> String {
    let file_path = format!("./inputs/day{}", day_num);
    let mut input_file = File::open(file_path).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}
