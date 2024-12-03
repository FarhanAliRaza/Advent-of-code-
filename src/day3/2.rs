use std::fs;





#[derive(Debug)]
struct Node<'a> {
    //type of node
    t: &'a str,
    value: i32,
    idx: usize,
    num1: i32, 
    num2 : i32


}


struct Parser<'a> {

    s : &'a str,
    idx : usize,
    next : char,
    start: usize,
}

impl<'a> Parser<'a> {

    fn next(&mut self){
        //xyz
        self.idx += 1;
        let n = self.s.chars().nth(self.idx);
        if n.is_none() {
            if self.is_eof(){

            }else{

                self.next();
            }
        }else{
            self.next = n.unwrap();
        }
    }

    fn is_eof(&self) -> bool {
        self.idx >= self.s.len()
    }
    fn start(&mut self){
        //xyz
        self.start = self.idx;
    }

   
    
    
}





fn main() {

    //mul(427,266)

    



    let contents = fs::read_to_string("/home/farhan/code/advent_of_code/day_1/src/input.txt")
        .expect("File not found");

    // let contents: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let mut  sum = 0;

    let mut nodes = Vec::new();
    
    let mut parser = Parser {
        s: &contents,
        idx: 0,
        next: contents.chars().nth(0).unwrap(),
        start:0
    };
    //implement the prse loop here
    while !parser.is_eof() {
        if parser.next.is_alphabetic() {
            // println!("next: {:?}, peek: {:?}", parser.next, parser.peek());

            if parser.next == 'm' {
                //parse mul
                parser.next();
                if parser.next == 'u' {
                    parser.next();
                    if parser.next == 'l' {
                        parser.next();
                        if parser.next == '(' {
                                
                                // 
                                parser.next();
                                if parser.next.is_digit(10) {
                                    let mut num1 = String::new();
                                    while parser.next.is_digit(10) {
                                        num1.push(parser.next);
                                        parser.next();
                                    }
                                    
                                    // Skip comma
                                    if parser.next == ',' {
                                        parser.next();
                                        
                                        // Parse second number
                                        let mut num2 = String::new();
                                        while parser.next.is_digit(10) {
                                            num2.push(parser.next);
                                            parser.next();
                                        }
                                        
                                        // Check closing parenthesis
                                        if parser.next == ')' {
                                            // Convert strings to numbers and multiply
                                            if let (Ok(n1), Ok(n2)) = (num1.parse::<i32>(), num2.parse::<i32>()) {
                                                let result = n1 * n2;
                                                let node = Node {
                                                    value: result,
                                                    t:"mul",
                                                    idx:parser.idx,
                                                    num1: n1,
                                                    num2: n2,

                                                };
                                                nodes.push(node);

                                            }
                                            
                                        }
                                    }
                                }

                                

                            }




                        }
                    }
                }else if parser.next == 'd' {
                    let mut s = String::new();
                    s.push(parser.next);
                    parser.next();
                    if parser.next == 'o' {
                        s.push(parser.next);
                        parser.next();
                        
                        // Handle "don't"
                        if parser.next == 'n' {
                            s.push(parser.next);
                            parser.next();
                            if parser.next == '\''{
                                s.push(parser.next);
                                parser.next();
                                if parser.next == 't' {
                                    s.push(parser.next);
                                    parser.next();
                                }
                            }
                        }
                        
                        // Skip parentheses
                        if parser.next == '(' {
                            s.push(parser.next);
                            parser.next();
                            
                            // while parser.next != ')' && !parser.is_eof() {
                            //     parser.next();
                            // }
                            if parser.next == ')' {
                                s.push(parser.next);
    
                                if s == "do()"{
                                    let node = Node{
                                        t:"do",
                                        value: 0,
                                        idx:parser.idx,
                                        num1:0,
                                        num2:0


                                    };
                                    nodes.push(node);
                                }else if s == "don't()" {
                                    let node = Node{
                                        t:"don't",
                                        value: 0,
                                        idx:parser.idx,
                                        num1:0,
                                        num2:0

                                    };
                                    nodes.push(node);
    
    
                                }
    
                            }
                        }
                    }
                }


            }         

        parser.next();
    }
    let mut dont = false;
    let mut dos = 0;
    let mut donts = 0;
    for node in nodes{
        println!("{:?}", node);
        if node.t == "don't"{
            donts += 1;
            dont = true;
        }else if node.t == "do"{
            dos +=1;
            dont = false
        }
        if node.t == "mul"{
            if dont == false {
                sum = sum + node.value
            }
        }

    }
    println!("{:?}", sum);
    println!("Do: {:?}", dos);
    println!("Donts: {:?}", donts);


       
    }
        
    // println!("{:?}", nodes);
 


    

   
    


    



    

