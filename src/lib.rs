use std::fs::File;
use std::io::Read;

pub fn get_input(day: u32) -> File {
    let mut path = std::env::current_dir().unwrap();
    path.push("inputs");
    path.push(format!("day{}.txt", day));

    File::open(path).unwrap()
}

pub fn get_input_as_str(day: u32) -> String {
    let mut file = get_input(day);
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
