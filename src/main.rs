use std::fs;



fn main() {

    let contents = fs::read_to_string("/home/farhan/code/advent_of_code/day_1/src/input.txt")
        .expect("File not found");

    let mut tests: Vec<Vec<i32>> = Vec::new();
    for line in  contents.lines() {
        let test: Vec<i32>  = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        tests.push(test);
    }
 



}
