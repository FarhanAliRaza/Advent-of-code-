use std::fs;

fn main() {

    let contents = fs::read_to_string("/home/farhan/code/advent_of_code/day_1/src/input.txt")
        .expect("File not found");

    
    let mut vec_a: Vec<i32> = Vec::new();
    let mut vec_b: Vec<i32> = Vec::new();
    for line in  contents.lines() {
        let lon: Vec<&str>  = line.split_whitespace().collect();
        let a:i32 = lon[0].parse().unwrap();
        let b: i32 = lon[1].parse().unwrap();
        vec_a.push(a);
        vec_b.push(b);
    }
    vec_a.sort();
    vec_b.sort();
    let mut sim : i32 = 0; 
    //part 1
    for (_pos, v) in vec_a.iter().enumerate() {


        let tor: i32 = vec_b.iter().filter(|&n| *n == *v).count().try_into().unwrap();
        
        sim  = sim + (v * tor)


        
    }
    println!("{sim}");



    

   
    


    



    

}
