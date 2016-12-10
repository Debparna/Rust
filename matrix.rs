use std::{ops, fmt};

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}

impl<T: Copy> Matrix<T> 
{
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        
       let mut matrix = Vec::new();
            for i in values { 
                match *i {
                    isize => matrix.push(isize),
                }
               } 

        Matrix {
            col: col,
            row: row,
            data: matrix,
        }  
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
          let mut matrix = Vec::new();

        Matrix {
            col: col,
            row: row,
            data: matrix,
        }  
    }

    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        &self.data
      
    }

    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
         &mut self.data
    }

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
      let x: (usize, usize) = (self.row, self.col);
      x
    }
}


impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> 
{
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
     let mut v = Vec::new();   
     let length = self.row * self.col; 
    // if(self.row != rhs.row || self.col != rhs.col) {
    //    panic!("Not Equal");
     //}
     //else{
        for i in 0..length {
      
            v.push(self.data[i] + rhs.data[i]);

        }
     // }
      
      Matrix{
        data: v,
        row: self.row,
        col: self.col,
      }
  }

}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> 
{
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output 
    {
        let mut v = Vec::new();  
        let length = self.row * self.col;
           // if(self.row != rhs.row || self.col != rhs.col) 
            //{
           //     panic!("Not Equal");
           // }
          //  else
          //  {      
                for i in 0..length 
                {
                    v.push(self.data[i] - rhs.data[i]);
                }
            //}
Matrix{
        data: v,
        row: self.row,
        col: self.col,
      }
  
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy + Default> ops::Mul for Matrix<T> 
{
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.

    fn mul(self, rhs: Self) -> Self::Output 
    {
        let mut v = Vec::new();          
        let length = self.row * self.col; 

        for i in 0..self.row 
        { 
            for j in 0..rhs.col 
            {
                let mut sum: T = Default::default();
                for k in 0..self.row 
                {
                    sum = sum + (self.data[i*self.col+j] * rhs.data[j*rhs.col+k]);
                }
            }
        }

         
Matrix{
        data: v,
        row: self.row,
        col: self.col,
      }
   
    
  }
}


impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, 
    //except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      

        let mut s = String::new();
          /*
        let mut k = 0;
        for 0 in self.row {
            for 0 in self.col {
                push_str(&format("\n", data[k]));
            }
            push(" ");
            k = k + 1;
        }
        */
       write!(f, "{}", s)
    
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
//use Matrix;
use super::*;
    #[test]
    fn test_new() {
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        println!("{:?}", x.row);
        println!("{:?}", x.col);
        println!("{:?}", x.data);
        assert_eq!(x.row, 2 as usize);
        assert_eq!(x.col, 3 as usize);
        assert_eq!(x.data[0], -2);
    }
    #[test]
    fn test_new_empty() {
        let x:Matrix<i32> = Matrix::new_empty(2,3);
        assert_eq!(x.row, 2);
        assert_eq!(x.col, 3);
    }
/*
     #[test]
    fn test() {
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(2, 3, &[0, 0, 0, 0, 0, 0]);
    assert_eq!(x + y, x);
    assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
    }
    */

    #[test]
    fn test_data() {
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]); 
        let shared_ref = x.data();
        assert_eq!(x.data[0], shared_ref[0]);
    }
    #[test]
    fn test_mut_data() {
        let mut x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        let data = x.mut_data();
        data[0] = 5;
        assert_eq!(data[0], 5);
    }
    #[test] 
    fn test_size() {
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        let y = x.size();
        assert_eq!(y.0, 2);
        assert_eq!(y.1, 3);
    }
     #[test]
    fn test_add() {
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]);
        let z = x + y;
        assert_eq!(z.data, [-1,0,1,2,3,4]);
        //assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
    }
    #[test]
    fn test_sub() {
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]);
        let z = x - y;
        assert_eq!(z.data, [-3,-2,-1,0,1,2]);
    }
    #[test]
    #[should_panic]
    //#[ignore]
    fn test_add_panic() {
        let x = Matrix::new(1, 3, &[-2, -1, 0, 1, 2, 3]);
        let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]);
        let _z = x + y;
    }
    
    #[test]
    #[should_panic]
    //#[ignore]
    fn test_sub_panic() {
        let x = Matrix::new(1, 3, &[-2, -1, 0, 1, 2, 3]);
        let y = Matrix::new(2, 3, &[1, 1, 1, 1, 1, 1]);
        let _z = x - y;
    }
    #[test]
    fn test_fmt() {
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
    }
    #[test]
    fn test_mul() {
        let x = Matrix::new(3, 3, &[1,2,3,4,5,6,7,8,9]);
        let y = Matrix::new(3, 3, &[9,8,7,2,2,2,1,1,1]);
        let z = x * y;
        assert_eq!(z.data, [16,15,14,52,48,44,88,81,74]);
    }
}