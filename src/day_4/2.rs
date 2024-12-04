use std::fs;





fn main() {


    



    let contents = fs::read_to_string("/home/farhan/code/advent_of_code/day_1/src/input.txt")
        .expect("File not found");

    let mut  sum = 0;

    let lines : Vec<&str> = contents.split("\n").collect();

  




    let mut vov = [].to_vec();


    for line in lines {
        let voc: Vec<char> = line.chars().collect();
        if voc.len() > 0 {
            vov.push(voc);
        }
    }


    println!("{:?}", vov);

    let height = vov.len();
    let width = vov[0].len();
    
    let mas = "MAS";
    let sam = "SAM";
    for y in 0..height {
        for x in 0..width {

            println!("{:?}", vov[y][x]);
            if vov[y][x] == 'M' || vov[y][x] == 'S' {
     

                //  diagonal Down right

                if y + 2 < height && x + 2 < width {
                    let mut s = String::new();

                    for i in 0..3 {
                        s.push(vov[y+i][x+i]);
                    }

                    if s == sam || s == mas {
                        println!("Digonal Down Right: {:?}", s);

                        if y + 2 < height && x+2 > 1 {
                            let mut s = String::new();
        
                            for i in 0..3 {
                                s.push(vov[y+i][x+2-i]);
                            }
        
                            if s == sam || s == mas  {
                                println!("Digonal Down left: {:?}", s);
                                sum += 1;
                            }
                        }

                        
                    }
                }
         



            }

        }

    }







    println!("{:?}", sum)
   


       
    }
        


    

   
    


    



    

