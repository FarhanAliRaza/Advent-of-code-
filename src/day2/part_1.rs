use std::fs;


fn is_valid(v: Vec<i32>)-> bool{

    let  is_inc: bool ;

    let mut l: usize = 0;
    let mut r= 1; 
    
    if v[r] > v[l] {
        is_inc = true;
    }else{
        is_inc = false;
    }
    //0 1
    //[16, 18, 20, 22, 23, 22],
    while r < v.len() {
        
        if is_inc {
            //should be increasing
            if v[r] - v[l] > 3 || v[r] - v[l] < 1 {
                return false;
            }

        } else {
            // should be decreasing
            //[10,9,2,1]
            if v[l] - v[r] > 3 ||  v[l] - v[r] < 1{
                return false
            }
        }

        l += 1;
        r += 1;
    }
    return true;
    



}
fn main() {

    let contents = fs::read_to_string("/home/farhan/code/advent_of_code/day_1/src/input.txt")
        .expect("File not found");

    let mut tests: Vec<Vec<i32>> = Vec::new();
    for line in  contents.lines() {
        let test: Vec<i32>  = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        tests.push(test);
    }
 


    //testing
    // let a: Vec<i32> = [16, 18, 20, 22, 23, 22].to_vec();
    // let b: Vec<i32> = [10,9,2,1].to_vec();
    // let c: Vec<i32> = [10,9,8,7].to_vec();
    // let d: Vec<i32> = [3,4,5].to_vec();

    // assert!(is_valid(a) == false);
    // assert!(is_valid(b) == false);
    // assert!(is_valid(c) == true);
    // assert!(is_valid(d) == true);

    let mut valid = 0;
    for test in tests {
        if is_valid(test) {
            valid += 1;
        }
    }
    println!("{valid}")
    // println!("{:?}", tests);



    

   
    


    



    

}
