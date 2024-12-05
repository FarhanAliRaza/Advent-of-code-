use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/farhan/code/advent_of_code/day_1/src/input.txt")
        .expect("File not found");

    let mut sum = 0;


    println!("{:?}", sum)
}
