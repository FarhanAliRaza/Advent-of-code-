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

    let mut sum = 0;

    println!("{}", sum)
}
