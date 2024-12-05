use std::{fs};





fn main() {


    



    let contents = fs::read_to_string("/home/farhan/code/advent_of_code/day_1/src/input.txt")
        .expect("File not found");

    let mut  sum = 0;
    let mut rules: Vec<&str> = Vec::new();
    for line in contents.split("\n"){
        if line.contains("|"){

            rules.push(line);
            

        }else if line.contains(","){

            //tests
            println!("testing: {:?}", line);
            let mut passed = true;

            let str_no : Vec<&str> = line.split(",").collect();
            for (idx, no) in str_no.iter().enumerate() {
                
                    let mut r = idx + 1;
                    loop {
                        if r < str_no.len(){
                            let next_no = str_no[r];
                            let true_condition_str = format!("{no}|{next_no}");
                            let true_condition = true_condition_str.as_str();
                            let false_condition_str = format!("{next_no}|{no}");
                            let false_condition: &str = false_condition_str.as_str();
             
                            if rules.contains(&false_condition){
                                println!("false condition !!!! : {:?}", false_condition);
                                passed = false;
                                break;
                            }
                            r += 1;
    
                        }else{
                            break;
                        }
                    }
                    if !passed{
                        break;
                    }
                }

                if passed {
                    println!(
                        "Passed: {:?}", line
                    );

                    let mut middle: f32 = str_no.len() as f32 / 2.0;
                    let m: usize = middle.floor() as usize;
                    println!("middle: {:?}",  str_no[m]);
                    sum +=  str_no[m].parse::<i32>().unwrap();

                }else{
                    println!(
                        "Failed: {:?}", line
                    )

                }
            }

            



        }
        println!("{:?}", rules);

    




        println!("{:?}", sum)
    }


    
    
    
  
   


       
        


    

   
    


    



    

