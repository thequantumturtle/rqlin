
use std::fmt;
use crate::complex::Complex;



#[derive(Debug, PartialEq)]
pub struct CVec2{
    pub vector: Vec<Complex>,
}


impl fmt::Display for CVec2{
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result{
            write!(f, "{:?}", self.vector)
    }
}

impl CVec2{

    pub fn new(v:Vec<Complex>) -> CVec2{
        CVec2{
            vector: v,
        }
    }

    pub fn add(self, b:&CVec2) -> CVec2 {
        CVec2::new( 
            self.vector.iter().zip(b.vector.iter()).map(|(a, b)| a.add(&b)).collect()
        )
    }
    
    pub fn inverse(&self) -> CVec2 {
        CVec2::new(
            self.vector.iter().map(|a| a.mul(&Complex::new(-1.0, 0.0))).collect()
        )
    }
    
    pub fn scalar_mul(&self, scalar: &Complex) -> CVec2 {
        CVec2::new(
            self.vector.iter().map(|v| scalar.mul(&v)).collect()
        )
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_add(){
        let a =  CVec2::new(vec![Complex::new(1.0, 1.0), Complex::new(-2.0, -2.0), Complex::new(3.0, 4.0)]);
        let b = CVec2::new(vec![Complex::new(1.0, 1.0), Complex::new(-5.0, 4.0), Complex::new(2.0, 7.0)]);
        let sum = CVec2::new(vec![Complex::new(2.0, 2.0), Complex::new(-7.0, 2.0), Complex::new(5.0, 11.0)]);
        
        assert_eq!(sum.vector, a.add(&b).vector);
    }

    #[test]
    fn test_inverse(){
        let a = CVec2::new(vec![Complex::new(1.0, 1.0), Complex::new(-2.0, -2.0), Complex::new(3.0, 4.0)]);
        let inverse_a = a.inverse();
        let zero_vec = CVec2::new(vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)]);
        assert_eq!(zero_vec, a.add(&inverse_a));
    }

    #[test]
    fn test_scalar_mul(){
        let a = CVec2::new(vec![Complex::new(1.0, 1.0), Complex::new(-2.0, -2.0), Complex::new(3.0, 4.0)]);
        let scalar = Complex::new(1.0, 0.0);
        let product = CVec2::new(vec![Complex::new(1.0, 1.0), Complex::new(-2.0, -2.0), Complex::new(3.0, 4.0)]);
        assert_eq!(product, a.scalar_mul(&scalar));

        let scalar = Complex::new(2.0, 2.0);
        let product = CVec2::new(vec![Complex::new(0.0, 4.0), Complex::new(0.0, -8.0), Complex::new(-2.0, 14.0)]);
        assert_eq!(product, a.scalar_mul(&scalar));
    }
}