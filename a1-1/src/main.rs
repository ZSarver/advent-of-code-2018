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
    let mut first_accum = 0;
    for line in line_iterator {
        match line.parse::<i64>() {
            Err(why) => panic!("couldn't parse line {}: {}", line, why.description()),
            Ok(i) => first_accum += i,
        };
    }
    println!("{}", first_accum);

    // now onto the second challenge
    let mut accum_vec = Vec::new();
    let mut accum = 0;
    let mut result = 0;
    while result == 0 {
        for line in s.lines() {
            accum += line.parse::<i64>().unwrap();
            match find_in_vec(&accum_vec, &accum) {
                None => {},
                Some(v) => { result = v; break },
            };
            accum_vec.push(accum);
        }
    };
    println!("{}", result);
}

fn find_in_vec(vec: &Vec<i64>, i: &i64) -> Option<i64> {
    for item in vec.iter() {
        if *item == *i {
            return Some(*i);
        }
    }
    return None;
}
