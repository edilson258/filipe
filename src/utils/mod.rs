use std::fs::File;
use std::io::Read;

pub fn read_file(path: &str) -> Option<String> {
    match File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                Some(contents)
            } else {
                eprintln!("[IOError]: Couldn't read from file {path}");
                None
            }
        }
        Err(_) => {
            eprintln!("[IOError]: Couldn't open file {path}");
            None
        },
    }
}
