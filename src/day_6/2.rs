use std::fs;

#[derive(Debug)]
struct Position {
    x:usize,
    y:usize,
    sx:usize,
    sy:usize,
    direction: char,
    sdirection: char,
    matrix: Vec<Vec<char>>,
    height:usize,
    width: usize


}
const up: &char = &'^';
const right: &char = &'>';
const left: &char = &'<';
const down: &char = &'v';
const stop: &char = &'#';

const outside: &char = &'!';



impl Position {

    fn peek(&self, direction: &char){
        

        
    }
    fn reset(&mut self){
        self.x = self.sx;
        self.y = self.sy;
        self.direction = self.sdirection;

    }

    fn next(&mut self){

        // go to next position
        if self.direction == *up {
            if self.y == 0{
                self.direction = *outside;
            }else{
                let y: usize = self.y - 1;
                if y < 0 {
                    self.direction = *outside;
                }else{
                    let peek = self.matrix[y][self.x];
                    if peek == *stop {
                        // go right 
                        self.direction = *right

                    }else{
                        //continue
                        self.y = y;
                    }

                }

            }        
            
        }else if self.direction == *right {        
            let x = self.x + 1;
            if x < self.width {
                let peek = self.matrix[self.y][x];
                if peek == *stop {
                    // go right 
                    self.direction = *down

                }else{
                    //continue
                    self.x = x;
                }
               
            }else{
                self.direction = *outside;
            }
        
        }else if self.direction == *down {        
            let y = self.y + 1;
            if y < self.height {
                let peek = self.matrix[y][self.x];
                if peek == *stop {
                    // go right 
                    self.direction = *left

                }else{
                    //continue
                    self.y = y;
                }
               
            }else{
                self.direction = *outside;
            }
        }else if self.direction == *left {   
            if self.x == 0{
                self.direction = *outside; 
            }else{
                let x = self.x - 1;
                if x >= 0 {
                    let peek = self.matrix[self.y][x];
                    if peek == *stop {
                        // go right 
                        self.direction = *up

                    }else{
                        //continue
                        self.x = x;
                    }
                
                }else{
                    self.direction = *outside;
                }
                
            }   
            
        
        }

    
}

}
fn main() {
    let contents = fs::read_to_string("/home/farhan/code/advent_of_code/day_1/src/input.txt")
        .expect("File not found");

    let lines : Vec<&str> = contents.split("\n").collect();


    let mut vov: Vec<Vec<char>> = [].to_vec();

    let mut p =  Position{
        x:0,
        y:0,
        sx:0,
        sy:0,
        sdirection:*right,
        direction: *right,
        matrix: vov.clone(),
        height:0,
        width: 1
    };


    for (y, line) in lines.iter().enumerate() {
        let voc: Vec<char> = line.chars().collect();
        if voc.len() > 0 {
            if voc.contains(left) {
                let x = voc.iter().position(|&i| i == *left).unwrap();
                p.x = x;
                p.y = y;
                p.sx = x;
                p.sy = y;
                p.sdirection = *left;
                
                p.direction = *left;


            }else if voc.contains(up){
                println!("{:?}", voc);
                let x = voc.iter().position(|&i| i == *up).unwrap();
                p.x = x;
                p.y = y;
                p.sx = x;
                p.sy = y;
                p.sdirection = *up;
                p.direction = *up;
            } else if voc.contains(right){
                let x = voc.iter().position(|&i| i == *right).unwrap();
                p.x = x;
                p.y = y;
                p.sx = x;
                p.sy = y;
                p.sdirection = *right;
                p.direction = *right;
            } else if  voc.contains(down){
                let x = voc.iter().position(|&i| i == *down).unwrap();
                p.x = x;
                p.y = y;
                p.sx = x;
                p.sy = y;
                p.sdirection = *down;
                p.direction = *down;                

            }
                vov.push(voc);

            
        }
    }

    let height = vov.len();
    let width = vov[0].len();
    p.matrix = vov;
    p.height = height;
    p.width = width;

    let mut sum = 0;
    let mut ox= 0;
    let mut oy = 0;
    'outer: loop {
        
        println!("{}, {}, {}",ox, oy, p.matrix[oy][ox]);
        if p.matrix[oy][ox] == '.'{
            let mut visited: Vec<String> = Vec::new();
            p.matrix[oy][ox] = '#';
            p.reset();
            // println!("x: {} y: {} direction: {:?}, {:?}", p.x, p.y, p.direction, p.matrix);

            loop {
                if p.direction == *outside {
                    p.matrix[oy][ox] = '.';
                    break
                }else{
                    let x = p.x;
                    let y = p.y;
                    let d = p.direction;
                    let position = format!("{x}|{y}|{d}");
                    if visited.contains(&position) {
                        //loop detected
                        println!("Loop detected");
                        sum += 1;
                        p.matrix[oy][ox] = '.';
                        break;
                    }else{
                        visited.push(position);
                    }
                }
                p.next();
            }
        }
        ox += 1;

        if ox < p.width {
            println!(">>>{}, {}", ox, p.width);


        }else{
            println!(">>>{}, {}", ox, p.width);

            ox = 0;
            oy += 1;

    }
    if oy < p.height {
    }else{
        break;
    }
    
       

        
    }
   

 


    println!("sum: {}", sum)
}
