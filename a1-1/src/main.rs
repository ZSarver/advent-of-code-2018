use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;

fn main() {
    let path = Path::new("input");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    
    let line_iterator = s.lines();
    let mut accum = 0;
    for line in line_iterator {
        match line.parse::<i64>() {
            Err(why) => panic!("couldn't parse line {}: {}", line, why.description()),
            Ok(i) => accum += i,
        };
    }
    println!("{}", accum);
}
