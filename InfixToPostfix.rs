
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

//---------------------------------------------------------------------------------//

pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> 
{ 

   let length = tokens.len(); 

  

    for i in 0..length { 
        if tokens[i] == InfixToken::Operator(Operator::Add) || tokens[i] == InfixToken::Operator(Operator::Sub) || tokens[i] == InfixToken::Operator(Operator::Mul) || tokens[i] == InfixToken::Operator(Operator::Div) || tokens[i] == InfixToken::RightParen {
            if i > 0 { 
                if tokens[i - 1] == InfixToken::Operator(Operator::Add) || tokens[i - 1] == InfixToken::Operator(Operator::Sub) || tokens[i - 1] == InfixToken::Operator(Operator::Mul) || tokens[i - 1] == InfixToken::Operator(Operator::Div) || tokens[i - 1] == InfixToken::LeftParen {
                    return None; 
                }
            }
        }
    }

      let mut s = &tokens[0];
        match s {
           &InfixToken::Operator(Operator::Add) => return None,
           &InfixToken::Operator(Operator::Sub) => return None,
           &InfixToken::Operator(Operator::Mul) => return None,
           &InfixToken::Operator(Operator::Div) => return None,
           &InfixToken::RightParen => return None,
          _ => {},
      }
  

    let mut s = &tokens[length - 1];
    match s {
      &InfixToken::Operator(Operator::Add) => return None,
       &InfixToken::Operator(Operator::Sub) => return None,
        &InfixToken::Operator(Operator::Mul) => return None,
         &InfixToken::Operator(Operator::Div) => return None,
      _ => {},
    }

    let mut pcount = 0;
    for i in 0..length {
        if tokens[i] == InfixToken::LeftParen { 
            pcount = pcount + 1; 
        }
    }

    for i in 0..length {
        if tokens[i] == InfixToken::RightParen {
            pcount = pcount - 1; 
        }
    }
    if pcount != 0  { 
        return None; 
    } 

        

//---------------------------------------------------------------------------------//


    let mut stack: Vec<InfixToken> = Vec::new();
    let mut result: Vec<PostfixToken> = Vec::new();
        let mut mdcount = 0;
   for i in tokens 
   {
      match i
      { 
       &InfixToken::Operator(Operator::Add) =>{
           if stack.len() == 0 { 
                stack.push(InfixToken::Operator(Operator::Add)); 
            } 
            else { 
                while stack.len() != 0 && stack[stack.len() - 1] != InfixToken::LeftParen {
                    let x = stack.pop().unwrap(); 
                    match x { 
                        InfixToken::Operand(isize) => {},
                        InfixToken::Operator(Operator::Add) => result.push(PostfixToken::Operator(Operator::Add)), 
                        InfixToken::Operator(Operator::Sub) => result.push(PostfixToken::Operator(Operator::Sub)),
                        InfixToken::Operator(Operator::Mul) => result.push(PostfixToken::Operator(Operator::Mul)),
                        InfixToken::Operator(Operator::Div) => result.push(PostfixToken::Operator(Operator::Div)),
                        InfixToken::LeftParen => {}, 
                        InfixToken::RightParen => {},  
                    }
                }
                stack.push(InfixToken::Operator(Operator::Add));
            }
       },
        &InfixToken::Operator(Operator::Sub) =>{
                if stack.len() == 0 { 
                stack.push(InfixToken::Operator(Operator::Sub)); 
            } 
            else { 
                while stack.len() != 0 && stack[stack.len() - 1] != InfixToken::LeftParen {
                    let x = stack.pop().unwrap(); 
                    match x { 
                        InfixToken::Operand(isize) => {},
                        InfixToken::Operator(Operator::Add) => result.push(PostfixToken::Operator(Operator::Add)), 
                        InfixToken::Operator(Operator::Sub) => result.push(PostfixToken::Operator(Operator::Sub)),
                        InfixToken::Operator(Operator::Mul) => result.push(PostfixToken::Operator(Operator::Mul)),
                        InfixToken::Operator(Operator::Div) => result.push(PostfixToken::Operator(Operator::Div)),
                        InfixToken::LeftParen => {}, 
                        InfixToken::RightParen => {},  
                    }
                }
                stack.push(InfixToken::Operator(Operator::Sub));
            }
        
        },
    &InfixToken::Operator(Operator::Mul) => {
        mdcount = mdcount + 1;
        if(result.len() == 0){return None;}
        if stack.len() == 0 {
          stack.push(InfixToken::Operator(Operator::Mul));
        }
        
        else
        {
          let mut AddorSub = 0; 
                while stack.len() != 0 && stack[stack.len() - 1] != InfixToken::LeftParen && AddorSub != 2
                {
                    let x = stack.pop().unwrap(); 
                    match x { 
                        InfixToken::Operand(isize) => {},
                        InfixToken::Operator(Operator::Add) => {stack.push(x); AddorSub = 2}, 
                        InfixToken::Operator(Operator::Sub) => {stack.push(x); AddorSub = 2},
                        InfixToken::Operator(Operator::Mul) => result.push(PostfixToken::Operator(Operator::Mul)),
                        InfixToken::Operator(Operator::Div) => result.push(PostfixToken::Operator(Operator::Div)),
                        InfixToken::LeftParen => {}, 
                        InfixToken::RightParen => {},  
                    }
                }
            stack.push(InfixToken::Operator(Operator::Mul));
        } 
    },
    &InfixToken::Operator(Operator::Div) => {

            if stack.len() == 0 { 
                stack.push(InfixToken::Operator(Operator::Div)); 
            } 
            else { 
                let mut AddorSub = 0; 
                while stack.len() != 0 && stack[stack.len() - 1] != InfixToken::LeftParen && AddorSub != 2{
                    let x = stack.pop().unwrap(); 
                    match x { 
                        InfixToken::Operand(isize) => {},
                        InfixToken::Operator(Operator::Add) => {stack.push(x); AddorSub = 2}, 
                        InfixToken::Operator(Operator::Sub) => {stack.push(x); AddorSub = 2},
                        InfixToken::Operator(Operator::Mul) => result.push(PostfixToken::Operator(Operator::Mul)),
                        InfixToken::Operator(Operator::Div) => result.push(PostfixToken::Operator(Operator::Div)),
                        InfixToken::LeftParen => {}, 
                        InfixToken::RightParen => {},  
                    }
                }
                stack.push(InfixToken::Operator(Operator::Div));
            }
    },
    &InfixToken::Operand(isize) => {
            result.push(PostfixToken::Operand(isize));       
    }, 
    &InfixToken::LeftParen => {
         stack.push(InfixToken::LeftParen);
    },
    &InfixToken::RightParen => {        
            while stack.len() != 0 && stack[stack.len() - 1] != InfixToken::LeftParen { 
                let x = stack.pop().unwrap(); 
                match x { 
                    InfixToken::Operand(isize) => {},
                    InfixToken::Operator(Operator::Add) => result.push(PostfixToken::Operator(Operator::Add)), 
                    InfixToken::Operator(Operator::Sub) => result.push(PostfixToken::Operator(Operator::Sub)),
                    InfixToken::Operator(Operator::Mul) => result.push(PostfixToken::Operator(Operator::Mul)),
                    InfixToken::Operator(Operator::Div) => result.push(PostfixToken::Operator(Operator::Div)),
                    InfixToken::LeftParen => {}, 
                    InfixToken::RightParen => {},  
                }
            }
            stack.pop(); 

    },
  }

}//for 

      while stack.len() != 0 { 
        let x = stack.pop().unwrap(); 
        match x { 
            InfixToken::Operand(isize) => {},
            InfixToken::Operator(Operator::Add) => result.push(PostfixToken::Operator(Operator::Add)), 
            InfixToken::Operator(Operator::Sub) => result.push(PostfixToken::Operator(Operator::Sub)),
            InfixToken::Operator(Operator::Mul) => result.push(PostfixToken::Operator(Operator::Mul)),
            InfixToken::Operator(Operator::Div) => result.push(PostfixToken::Operator(Operator::Div)),
            InfixToken::LeftParen => {}, 
            InfixToken::RightParen => {},  
        }
    }

  return Some(result);

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
//use Matrix;
use super::*;
    #[test]

fn testcase1 (){
    //INFIX: 3 * 4 + 6
    //POSTFIX: 3 4 * 6 +
        let x = &[InfixToken::Operand(3),
                  InfixToken::Operator(Operator::Mul),
                  InfixToken::Operand(4),
                  InfixToken::Operator(Operator::Add),
                  InfixToken::Operand(6),
                  //InfixToken::Operator(Operator::Add),
                  //InfixToken::Operand(4),
                  //InfixToken::Operator(Operator::Mul),
                  //InfixToken::Operand(2),
              ];
        let y = Some(vec![
                 PostfixToken::Operand(3),
                 PostfixToken::Operand(4),
                 PostfixToken::Operator(Operator::Mul),
                 PostfixToken::Operand(6),
                 PostfixToken::Operator(Operator::Add),
                 //PostfixToken::Operand(4),
                 //PostfixToken::Operand(2),
                 //PostfixToken::Operator(Operator::Mul),
                 //PostfixToken::Operator(Operator::Add)
                 ]);
        assert_eq!(y, infix_to_postfix(x));
}
  #[test]
fn testcase2(){
    /* INFIX: 3 - 4 + 6 - 4 + 6 * 10
       POSTFIX: 3 4 - 6 + 4 - 6 10 * +
    */
let x = &[InfixToken::Operand(3),
          InfixToken::Operator(Operator::Sub),
          InfixToken::Operand(4),
          InfixToken::Operator(Operator::Add),
          InfixToken::Operand(6),
          InfixToken::Operator(Operator::Sub),
          InfixToken::Operand(4),
          InfixToken::Operator(Operator::Add),
          InfixToken::Operand(6),
          InfixToken::Operator(Operator::Mul),
          InfixToken::Operand(10),
      ];
      let y = Some(vec![
         PostfixToken::Operand(3),
         PostfixToken::Operand(4),
         PostfixToken::Operator(Operator::Sub),
         PostfixToken::Operand(6),
         PostfixToken::Operator(Operator::Add),
         PostfixToken::Operand(4),
         PostfixToken::Operator(Operator::Sub),
         PostfixToken::Operand(6),
         PostfixToken::Operand(10),
         PostfixToken::Operator(Operator::Mul),
         PostfixToken::Operator(Operator::Add)]);
      assert_eq!(y, infix_to_postfix(x));
}
  #[test]
fn testcase3(){
    // INFIX: 6 + 4 + 6 / 10 * 5 * 4
    // POSTFIX: 6 4 + 6 10 / 5 * 4 * +
          let x = &[
              InfixToken::Operand(6),
              InfixToken::Operator(Operator::Add),
              InfixToken::Operand(4),
              InfixToken::Operator(Operator::Add),
              InfixToken::Operand(6),
              InfixToken::Operator(Operator::Div),
              InfixToken::Operand(10),
              InfixToken::Operator(Operator::Mul),
              InfixToken::Operand(5),
              InfixToken::Operator(Operator::Mul),
              InfixToken::Operand(4),
          ];
          let y = Some(vec![
             PostfixToken::Operand(6),
             PostfixToken::Operand(4),
             PostfixToken::Operator(Operator::Add),
             PostfixToken::Operand(6),
             PostfixToken::Operand(10),
             PostfixToken::Operator(Operator::Div),
             PostfixToken::Operand(5),
             PostfixToken::Operator(Operator::Mul),
             PostfixToken::Operand(4),
             PostfixToken::Operator(Operator::Mul),
             PostfixToken::Operator(Operator::Add)]);
          assert_eq!(y, infix_to_postfix(x));


}

#[test]
fn testcase4 (){
    // INFIX: 3 * (4 + 5)
    // POSTFIX: 3 4 5 + *
let x = &[InfixToken::Operand(3),
          InfixToken::Operator(Operator::Mul),
          InfixToken::LeftParen,
          InfixToken::Operand(4),
          InfixToken::Operator(Operator::Add),
          InfixToken::Operand(5),
          InfixToken::RightParen];
      let y = Some(vec![
         PostfixToken::Operand(3),
         PostfixToken::Operand(4),
         PostfixToken::Operand(5),
         PostfixToken::Operand(10),
         PostfixToken::Operator(Operator::Add),
         PostfixToken::Operator(Operator::Mul)]);
      assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase5(){
    //INFIX: 3 + (4) + (4)
    //POSTFIX: 3 4 + 4 +
     let x = &[InfixToken::Operand(3),
          InfixToken::Operator(Operator::Add),
          InfixToken::LeftParen,
          InfixToken::Operand(4),
          InfixToken::RightParen,
          InfixToken::Operator(Operator::Add),
          InfixToken::LeftParen,
          InfixToken::Operand(4),
          InfixToken::RightParen,
      ];
      let y = Some(vec![
         PostfixToken::Operand(3),
         PostfixToken::Operand(4),
         PostfixToken::Operator(Operator::Add),
         PostfixToken::Operand(4),
         PostfixToken::Operator(Operator::Add)]);
      assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase6 (){
    //INFIX: (5+3)-6
    //POSTFIX: 5 3 + 6 -
          let x = &[
              InfixToken::LeftParen,
              InfixToken::Operand(5),
              InfixToken::Operator(Operator::Add),
              InfixToken::Operand(3),
              InfixToken::RightParen,
              InfixToken::Operator(Operator::Sub),
              InfixToken::Operand(6),
          ];
          let y = Some(vec![
             PostfixToken::Operand(5),
             PostfixToken::Operand(3),
             PostfixToken::Operator(Operator::Add),
             PostfixToken::Operand(6),
             PostfixToken::Operator(Operator::Sub)]);
          assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase7 (){
    //INFIX: 5(5+3)-6
    //POSTFIX: ERROR
    let x = &[
              InfixToken::Operand(5),
              InfixToken::LeftParen,
              InfixToken::Operand(5),
              InfixToken::Operator(Operator::Add),
              InfixToken::Operand(3),
              InfixToken::RightParen,
              InfixToken::Operator(Operator::Sub),
              InfixToken::Operand(6),
          ];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
#[test]
fn testcase8 (){
    //INFIX: 3 5 6
    //POSTFIX: ERROR
    let x = &[
          InfixToken::Operand(3),
          InfixToken::Operand(5),
          InfixToken::Operand(6),
    ];
    let y = None;
    assert_eq!(y,infix_to_postfix(x));
}

#[test]
fn testcase9 (){
    //INFX: (3 + 5))
    //POSTFIX: ERROR
    let x= &[
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
        InfixToken::RightParen,
    ];
    let y = None;
    assert_eq!(y,infix_to_postfix(x));
}

#[test]
fn testcase10 () {
    //INFIX: ((3+5))
    //POSTFIX: 3 5 +
    let x = &[
    InfixToken::LeftParen,
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(5),
    InfixToken::RightParen,
    InfixToken::RightParen,
    ];
    let y = Some(vec![
   PostfixToken::Operand(3),
   PostfixToken::Operand(5),
   PostfixToken::Operator(Operator::Add)]);
   assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase11 (){
    //INFIX: 3 + +
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operator(Operator::Add)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase12 (){
    //INFX: ((3 + 5)
    //POSTFIX: ERROR
    let x= &[
        InfixToken::LeftParen,
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
    ];
    let y = None;
    assert_eq!(y,infix_to_postfix(x));
}

#[test]
fn testcase13 (){
    //INFIX: (3+5)(3+5)
    //POSTFIX: ERROR
    let x= &[
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
    ];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase14 (){
    //INFIX: (3+5)5
    //POSTFIX: ERROR
    let x= &[
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
        InfixToken::Operand(5),
    ];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase15 (){
    //INFIX: ) 3 + 5 (
    //POSTFIX: Error
    let x= &[
        InfixToken::RightParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::LeftParen,
    ];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase16 (){
    //INFIX: 3 4 +
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operand(3),
    InfixToken::Operand(4),
    InfixToken::Operator(Operator::Add)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase17 (){
    let x = &[];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase18 (){
    //INFIX: +3
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase19 (){
    //INFIX: 2+*3
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operator(Operator::Mul),
    InfixToken::Operand(3)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase20 (){
    //INFIX: +(3+3)
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operator(Operator::Add),
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3),
    InfixToken::RightParen];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase21 (){
    //INFIX: (3+3+)
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::RightParen];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));

}

#[test]
fn testcase22 (){
    //INFIX: (3+3+)+3
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::RightParen,
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));

}

#[test]
fn testcase23 (){
    //INFIX: (3+4)+4(3-4)
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::RightParen,
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Sub),
    InfixToken::Operand(4),
    InfixToken::RightParen
    ];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase24 (){
    //INFIX: ()
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::RightParen
    ];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase25 () {
    //INFIX: )(3+4
    //POSTFIX: ERROR
    let x = &[
    InfixToken::RightParen,
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase26 (){
    //INFIX: 3+4-
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::Operator(Operator::Sub),];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase27 (){
    //INFIX: (3+4)
    //POSTFIX: 3 4 +
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::RightParen];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

#[test]
fn testcase28(){
    //INFIX: (3+4)() //Test case 13 & 19 on gradebot :D
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::RightParen,
    InfixToken::LeftParen,
    InfixToken::RightParen];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}

}


