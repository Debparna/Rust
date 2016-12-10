
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
}


pub enum Token {
    Operator(Operator),
    Operand(isize), 
}

/* 
  CONDITIONS
- if operator > operand 
- if token is empty
- if stack length is 0 
- if operator is before operand 
- if there are only operand on the stack
- if there are only operators on the stack
- if three operands and one operator 
*/

            
pub fn eval(tokens: &[Token]) -> Option<isize> {
    let mut stack: Vec<isize> = Vec::new();
    let mut stack = Vec::new(); 
    let length = stack.len(); 

    //CONDITIONS - NUMBER OF OPERANDS AND NUMBER OF OPERATORS 

    let mut operand = 0;
    for i in tokens { 
        let mut z: isize = 0;  
        match *i { 
            Token::Operand(isize) => operand = operand + 1,
            Token::Operator(Operator::Add) => z = 0, 
            Token::Operator(Operator::Mul) => z = 0, 
            Token::Operator(Operator::Sub) => z = 0, 
        }
    }
   //println!("Operand = {}", operand);
   
    let mut operator = 0;
    for i in tokens {
        let mut s: isize = 0;
        match *i {
            Token::Operand(isize) => s = 0,
            Token::Operator(Operator::Add) => operator = operator + 1,
            Token::Operator(Operator::Mul) => operator = operator + 1,
            Token::Operator(Operator::Sub) => operator = operator + 1,
        }
    }
   //println!("Operator = {}", operator);

    if( operand < operator ) {
        return None;
    }

    if( operand == operator ) { //WHAT???
        return None;
    }

    //if(operator == 0) {
      //  return None;
    //}

   if(operand == 0) {
        return None;
    }

    if ( operator != operand - 1) {
        return None;
    }
    

   //-----------------------------------------------------//
   if !((*tokens).is_empty()) {

   for i in tokens {
        let mut x = 0;

        match *i {
            Token::Operand(isize) => stack.push(isize), 
            Token::Operator(Operator::Add) => {
                let length = stack.len(); 
                if (length < 2) {
                    return None;
                }
                else {
                 x = stack.pop().unwrap() + stack.pop().unwrap();  
                }
            }, 

            Token::Operator(Operator::Mul) => {
                let length = stack.len(); 
                if (length < 2) {
                    return None;
                }
                else {
                x = stack.pop().unwrap() * stack.pop().unwrap();  
                }          
            }, 

            Token::Operator(Operator::Sub) => {
            let length = stack.len(); 
                if (length < 2) {
                    return None;
                }
                else {
                 x = -(stack.pop().unwrap() - stack.pop().unwrap());
                }
            },
         }


       if x > 0 { 
            stack.push(x); 
        }
        else if x < 0 {
            stack.push(x);
        }
        let length = stack.len();
          if length == 0 { 
            if x == 0 { 
                stack.push(x); 
            } 
        }
    }
}


let length = stack.len(); 
   if (length > 1) {  
      return None;
    }


  //-----------------------------------------------------//
    //CONDITIONS

    //Is empty
    if ((*tokens).is_empty()){
     return None;
    }


    //-----------------------------------------------------//
        //EXECUTION
        let x = stack.pop().unwrap();

        println!("{}", x); 
        return Some(x);
    }


pub fn main() { 
    let tokens_0 =  [Token::Operand(1)]; //Token::Operand(1), Token::Operand(1), Token::Operator(Operator::Sub)
    let samples: &[Token] = &tokens_0; //1--1-1
    eval(samples);
}


