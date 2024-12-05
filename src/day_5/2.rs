use std::fs;

fn main() {
    let contents = fs::read_to_string("/home/farhan/code/advent_of_code/day_1/src/input.txt")
        .expect("File not found");
    let new: Vec<i32> = Vec::new();

    let mut sum = 0;
    // let mut rules: HashMap<i32, HashMap<&str, Vec<i32>>> = HashMap::new();
    let mut rules: Vec<&str> = Vec::new();
    for line in contents.split("\n") {
        if line.contains("|") {
            rules.push(line);
        
        } else if line.contains(",") {
            let mut numbers: Vec<&str> = line.split(",").collect();
            let mut attempts = 0;
            let max_attempts = numbers.len() * numbers.len();
            
            'outer: loop {
                let mut passed = true;
                
                for i in 0..numbers.len() {
                    for j in i+1..numbers.len() {
                        let false_condition = format!("{}|{}", numbers[j], numbers[i]);
                        
                        if rules.contains(&false_condition.as_str()) {
                            numbers.swap(i, j);
                            passed = false;
                            attempts += 1;
                            
                            if attempts >= max_attempts {
                                println!("No valid arrangement found for: {}", line);
                                passed = false;
                                break 'outer;
                            }
                            
                            continue 'outer;
                        }
                    }
                }
                
                if passed  {
                    if attempts > 0 {
                        let middle = numbers.len() / 2;
                        sum += numbers[middle].parse::<i32>().unwrap();
                    }
             
                    println!("Valid arrangement found: {:?}", numbers);
                    break;
                }
            }
        }
    }
    println!("{:?}", rules);

    println!("{:?}", sum)
}
