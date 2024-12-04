use std::fs;


fn is_number(s : &str,  r: &mut usize) -> i32 {
    let mut  num: String = "".to_owned();
    loop {
        let next: char = s.chars().nth(*r).unwrap();
        if next.is_digit(10){

            num.push_str(&next.to_string());
            *r += 1;
        }else{
            break;
        }
    }
    let n : i32 = num.parse().unwrap();
    return n;
}

fn is_valid(s : &str) -> (bool, Vec<i32>){
    //"507,423)"

    let mut r = 0; 

    
    let mut nums : Vec<i32> = Vec::new();
    let mut next = s.chars().nth(r).unwrap();
    println!("{:?}", next);
    if next.is_digit(10){
        let number = is_number(s, &mut r);
        nums.push(number);
        println!("num: {:?}, r:{:?}", number, r);
        
        // expect a comma
        next = s.chars().nth(r).unwrap();
        println!("next: {:?}, contains: {:?}", next, next == ',');

        if next == ',' {
            r += 1;
            next = s.chars().nth(r).unwrap();
            if next.is_digit(10){
                let number = is_number(s, &mut r);
                nums.push(number);



            }else{
                return (false, [].to_vec());
            }
            // expect a )

            next = s.chars().nth(r).unwrap();
            if next == ')'{
                println!("{:?}", nums);
                return   (true, nums);
            }
            println!("next num: {:?}, r:{:?}", number, r);



        }else{
            return  (false, [].to_vec());
        }
        println!("num: {:?}, r:{:?}", next, r);
        
    } 
    return   (false, [].to_vec());
    
}


fn main() {

    //mul(427,266)

    



    let contents = fs::read_to_string("/home/farhan/code/advent_of_code/day_1/src/input.txt")
        .expect("File not found");

    // let contents = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let li = contents.split("mul(");
    let mut  sum = 0;
    for x in li {


        if s.len() > 3 {
            
        println!("{:?}, {:?}", x, x.len());
        println!("{:?}", &x);
        let (res, v )= is_valid(&x);
        if res {
            sum = sum +  (v[0] * v[1]);

        }
    
        }

           
    }
    println!("{:?}", sum)
 


    

   
    


    



    

}
