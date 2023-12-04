#[macro_export]
macro_rules! read_input {
    () => {{
        use std::io::{BufReader, BufRead};
        use std::fs::File;

        let file_path = file!();
        let mut path = std::path::PathBuf::from(file_path);
        path.set_file_name("input");
        let file = File::open(path).unwrap();
        let buffer = BufReader::new(file);
        buffer.lines().map(|line| line.unwrap()).collect::<Vec<String>>()
    }};
}
