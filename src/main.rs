use std::fs;
use std::io::BufRead;

fn main() {
    let file = fs::read("./input2").unwrap();

    println!("{}", day_one(file))
}

fn day_one(file: Vec<u8>) -> i32 {
    let mut total = 0;
    let mut temp = 0;

    for line in file.lines() {

        let line_blah = line.unwrap();

        if line_blah.is_empty() {
            if temp > total {
                total = temp;
            }
            temp = 0;
        } else {
            temp += line_blah.parse::<i32>().unwrap();
        }
    }

    if temp > total {
        return temp;
    }

    total
}


