use std::fs;





fn main() {


    



    let contents = fs::read_to_string("/home/farhan/code/advent_of_code/day_1/src/input.txt")
        .expect("File not found");

    let mut  sum = 0;

    let lines : Vec<&str> = contents.split("\n").collect();

    let loc = contents.len();
  
    // M M M S X X M A S M
    // M S A M X M S M S A
    // A M X S X M A A M M
    // M S A M A S M S M X


    let len_of_lines  = lines.len();

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
    
    let xmas = "XMAS";
    let samx = "SAMX";
    let sum_vec: Vec<i32> = [].to_vec();
    for y in 0..height {
        for x in 0..width {

            println!("{:?}", vov[y][x]);
            if vov[y][x] == 'X' {
                //horizontal
                if x + 3 < width {
                    let s = &vov[y][x..x+4].iter().collect::<String>();
                    if s == xmas  {
                        println!("Horizontal: {:?}", s);
                        
                        sum += 1;
                    }
                }
                //vertical Down
                if y + 3 < height {
                    let mut s = String::new();

                    for i in 0..4 {
                        s.push(vov[y+i][x]);
                    }
                    if s == xmas {
                        println!("Vertical Down: {:?}", s);

                        sum += 1;
                    } 
                }

                // Vertical Up

                // if y > 2 {
                //     let mut s = String::new();

                //     for i in 0..4 {
                //         s.push(vov[y-i][x]);
                //     }
                //     if s == xmas  {
                //         println!("Vertical Up: {:?}", s);

                //         sum += 1;
                //     } 
                // }


                //  diagonal Down right

                if y + 3 < height && x + 3 < width {
                    let mut s = String::new();

                    for i in 0..4 {
                        s.push(vov[y+i][x+i]);
                    }

                    if s == xmas  {
                        println!("Digonal Down Right: {:?}", s);

                        sum += 1;
                    }
                }
                //  diagonal Down left
                if y + 3 < height && x > 2 {
                    let mut s = String::new();

                    for i in 0..4 {
                        s.push(vov[y+i][x-i]);
                    }

                    if s == xmas  {
                        println!("Digonal Down left: {:?}", s);
                        sum += 1;
                    }
                }
                //Digonal Up
                // if y > 2  && x  > 2 {
                //     let mut s = String::new();

                //     for i in 0..4 {
                //         s.push(vov[y-i][x-i]);
                //     }

                //     if s == xmas  {
                //         println!("Digonal Up: {:?}", s);

                //         sum += 1;
                //     }


                // }



            }else if vov[y][x] == 'S' {
                if x + 3 < width {
                    let s = &vov[y][x..x+4].iter().collect::<String>();
                    if  s == samx {
                        println!("Horizontal: {:?}", s);

                        sum += 1;
                    }
                }
                //vertical Down
                if y + 3 < height {
                    let mut s = String::new();

                    for i in 0..4 {
                        s.push(vov[y+i][x]);
                    }
                    if  s == samx {
                        println!("Vertical Down: {:?}", s);

                        sum += 1;
                    } 
                }

                //Vertical Up

                // if y > 2 {
                //     let mut s = String::new();

                //     for i in 0..4 {
                //         s.push(vov[y-i][x]);
                //     }
                //     if s == samx {
                //         println!("Vertical Up: {:?}", s);

                //         sum += 1;
                //     } 
                // }


                //  diagonal Down

                if y + 3 < height && x + 3 < width {
                    let mut s = String::new();

                    for i in 0..4 {
                        s.push(vov[y+i][x+i]);
                    }

                    if  s == samx {
                        println!("Digonal Down Right: {:?}", s);

                        sum += 1;
                    }

                
                }
                //  diagonal Down Right
                if y + 3 < height && x > 2 {
                    let mut s = String::new();

                    for i in 0..4 {
                        s.push(vov[y+i][x-i]);
                    }

                    if s == samx  {
                        println!("Digonal Down Left: {:?}", s);
                        sum += 1;
                    }
                }
                //Digonal Up
                // if y > 2  && x  > 2 {
                //     let mut s = String::new();

                //     for i in 0..4 {
                //         s.push(vov[y-i][x-i]);
                //     }

                //     if  s == samx {
                //         println!("Digonal Up: {:?}", s);

                //         sum += 1;
                //     }
                // }
            }

        }

    }



    // for (no, line) in lines.iter().enumerate(){

    //     let len_of_line = line.len();
    //     let mut l = 0;
    //     let mut r =  0;

    //     loop   {

    //         let mut ch = line.chars().nth(l).unwrap_or('1');
    //         if ch == '1'{
    //             break;
    //         }

    //         println!("{:?}", ch);

    //         // XMAS
    //         if ch == 'X' {

    //             // horizontal 
    //             if l + 3 < len_of_line {
    //                 println!("{:?}", &line[l..l+4]);
    //                 if line[l..l+4] == *xmas {
    //                     sum += 1;
    //                 }
    //             }

    //             //vertical 

    //             if l + (3 *len_of_line) <  loc {
    //                 println!("{:?}", &line[l..l+4]);
             
    //             }



    //         }else if ch == 'S'{
    //             //hor-reversse // SAMX

    //             if l + 3 < len_of_line {
    //                 println!("{:?}", &line[l..l+4]);
    //                 if line[l..l+4] == *samx {
    //                     sum += 1;
    //                 }
    //             }
    //         }

    //         //vertical

            





            





    //         l += 1;
    //     }
        











    // }





    println!("{:?}", sum)
   


       
    }
        


    

   
    


    



    

