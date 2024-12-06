use std::fs;

#[derive(Debug)]
struct Position {
    x:usize,
    y:usize,
    direction: char,
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

    fn next(&mut self){

        // go to next position
        if self.direction == *up {        
            let y = self.y - 1;
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
fn main() {
    let contents = fs::read_to_string("/home/farhan/code/advent_of_code/day_1/src/input.txt")
        .expect("File not found");

    let lines : Vec<&str> = contents.split("\n").collect();


    let mut vov: Vec<Vec<char>> = [].to_vec();

    let mut p =  Position{
        x:0,
        y:0,
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
                p.direction = *left;
            }else if voc.contains(up){
                println!("{:?}", voc);
                let x = voc.iter().position(|&i| i == *up).unwrap();
                p.x = x;
                p.y = y;
                p.direction = *up;
            } else if voc.contains(right){
                let x = voc.iter().position(|&i| i == *right).unwrap();
                p.x = x;
                p.y = y;
                p.direction = *right;
            } else if  voc.contains(down){
                let x = voc.iter().position(|&i| i == *down).unwrap();
                p.x = x;
                p.y = y;
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

    let mut visited: Vec<String> = Vec::new();
    let mut sum = 0;
    
    loop {
        println!("x: {} y: {} direction: {:?}", p.x, p.y, p.direction);
        if p.direction == *outside {
            break
        }else{
            let x = p.x;
            let y = p.y;
            let position = format!("{x}|{y}");
            if visited.contains(&position) {
            }else{
                visited.push(position);
            }
            sum  += 1;
        }
        p.next();

    }

    // println!("{:?}", p);
    // p.next();
    // println!("{:?}", p);
    // p.next();

    // println!("{:?}", p);



    println!("{:?}, {}", sum, visited.len())
}
