use std::fs;
fn is_increasing (v: Vec<i32>)-> bool{
    let mut res: Vec<bool> = [].to_vec();
    let  iter= v.windows(2);
    
    for slice in iter {
        if slice[1] > slice[0] {
            res.push(true); 
        }else{
            res.push(false);
        }
    }
    
    let true_count = res.iter().filter(|x| **x == true).count();
    let false_count = res.iter().filter(|x| **x == false).count();

    if true_count > false_count {
        return  true;
    }
    else{
        return  false;
    }
   
}


fn is_valid(mut v: Vec<i32>)-> (bool, Vec<i32>){


    let mut l: usize = 0;
    let mut r= 1; 

    let is_inc = is_increasing(v.clone());
    while r < v.len() {
        
        if is_inc {
            //should be increasing
            //1, 2, 4, 5

            let diff =   v[r] - v[l];
            if !(diff <= 3 && diff >= 1) {
                if (r == v.len() - 1){
                    v.remove(r);

                }else if v[r+1] -  v[l] <= 3 && v[r+1] -  v[l] >= 1 {
                    //1,2,3,8,5
                    // 2
                    v.remove(r);

                }else{
                    v.remove(l); // remove can be l or

                }
                return  (false,v);
            }

        } else {
            // should be decreasing
            //5 4 3
            let diff =   v[l] - v[r];
            if !(diff <= 3 && diff >= 1) {
                // check which elment to remove
                if (r == v.len() - 1){
                    v.remove(r);
                }
                else if v[l] -  v[r+1]  <= 3 &&  v[l] -  v[r+1]  >= 1 {
                    //[75, 78, 76, 70, 69, 68, 69]
                    v.remove(r);
                }else{
                    v.remove(l);
                }
                return  (false, v);               
            }
        }

        l += 1;
        r += 1;
    }
    return (true, v);
    



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
    // let a: Vec<i32> = [1,  2, 4, 5].to_vec();
    // let a: Vec<i32> = [75, 78, 76, 75].to_vec();
    // let a: Vec<i32> = [ 11, 8, 6, 4, 7].to_vec();
    // let a: Vec<i32> = [1,2,3,8, 5].to_vec();
    // let a: Vec<i32> = [84, 86, 84, 82, 78, 75, 71].to_vec();
    // let c: Vec<i32> = [10,9,8,7].to_vec();
    // let d: Vec<i32> = [3,4,5].to_vec();
    // println!(" v: {:?}",  a);

    // let (b, v) = is_valid(a);
    // println!("b: {:?} v: {:?}", b, v);

    // let (b, v) = is_valid(v);
    // println!("b: {:?} v: {:?}", b, v);
    

    let mut valid = 0;
    for test in tests {
        let (b, v) = is_valid(test.clone());

        if  b{
            valid += 1;
        }else{
            let (c, _v) = is_valid(v.clone());
            if c {
                valid += 1
            }else{
   
            }
        }
    }
    //[75, 78, 76, 70, 69, 68, 69]
    println!("{valid}")
    // println!("{:?}", tests);

}
