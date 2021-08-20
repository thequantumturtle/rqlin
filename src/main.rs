mod complex_number {
    use std::fmt;

    #[derive(Debug, PartialEq)]
    pub struct ComplexNumber{
        pub real: f32,
        pub imaginary: f32,
    }

    impl fmt::Display for ComplexNumber{
        fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result{
            if self.imaginary < 0.0
            {
                write!(f, "{}{}i", self.real, self.imaginary)
            }else{
                write!(f, "{}+{}i", self.real, self.imaginary)
            }
        }
    }

    impl ComplexNumber{
        pub fn new(r:f32, i:f32) -> ComplexNumber{
            ComplexNumber{
                real:r, 
                imaginary:i,
            }
        }
    
        pub fn add(&self, b:&ComplexNumber) -> ComplexNumber{
            let real = self.real + b.real;
            let imaginary = self.imaginary + b.imaginary;
            ComplexNumber::new(real, imaginary)
        }
    
        pub fn sub(&self, b:&ComplexNumber) -> ComplexNumber{
            let neg_b = ComplexNumber::new(-b.real, -b.imaginary);
            ComplexNumber::add(&self, &neg_b)
        }
    
        pub fn mul(&self, b:&ComplexNumber) -> ComplexNumber{
            let real = self.real * b.real + self.imaginary * b.imaginary * -1.0;
            let imaginary = self.real * b.imaginary + self.imaginary * b.real;
            ComplexNumber::new(real, imaginary)
        }
    
        pub fn div(&self, b:&ComplexNumber) -> ComplexNumber{
            let divisor = b.modulus().powf(2.0);
            let real = (self.real * b.real + self.imaginary * b.imaginary)/divisor;
            let imaginary = (b.real * self.imaginary - self.real * b.imaginary)/divisor;
            ComplexNumber::new(real, imaginary)
        }
    
        pub fn modulus(&self) -> f32{
            (self.real.powf(2.0) + self.imaginary.powf(2.0)).sqrt()
        }
    
        pub fn conjugate(&self) -> ComplexNumber{
            ComplexNumber::new(self.real, -self.imaginary)
        }
    }

    
}
fn main() {
    let a = complex_number::ComplexNumber::new(-2.0, 1.0);
    println!("real: {}, imaginary: {}", a.real, a.imaginary);
    println!("{}", &a);
    let b = complex_number::ComplexNumber::new(3.0, -3.0);
    println!("{}", &b);
    println!("{:?}", &b);
    println!("{}", b.to_string());

}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn create_complex_number(){
        let c_num = complex_number::ComplexNumber::new(2.0, 3.0);
        assert_eq!(c_num.real, 2.0);
        assert_eq!(c_num.imaginary, 3.0);
        
    }

    #[test]
    fn formatter_display_complex_number(){
        let c_num = complex_number::ComplexNumber::new(0.0, 0.0);
        assert_eq!(c_num.to_string(), "0+0i");
       
        let c_num = complex_number::ComplexNumber::new(1.0, 1.0);
        assert_eq!(c_num.to_string(), "1+1i");
       
        let c_num = complex_number::ComplexNumber::new(1.0, -1.0);
        assert_eq!(c_num.to_string(), "1-1i");
       
        let c_num = complex_number::ComplexNumber::new(-1.0, -1.0);
        assert_eq!(c_num.to_string(), "-1-1i");

        let c_num = complex_number::ComplexNumber::new(-0.0, -0.0);
        assert_eq!(c_num.to_string(), "0+0i");
       
        let c_num = complex_number::ComplexNumber::new(0.12, -0.34);
        assert_eq!(c_num.to_string(), "0.12-0.34i");
    }

    #[test]
    fn formatter_debug_complex_number(){
        let c_num = complex_number::ComplexNumber::new(0.0, 0.0);
        assert_eq!(format!("{:?}", c_num), "ComplexNumber { real: 0.0, imaginary: 0.0 }");

        let c_num = complex_number::ComplexNumber::new(1.0, 1.0);
        assert_eq!(format!("{:?}", c_num), "ComplexNumber { real: 1.0, imaginary: 1.0 }");
    
        let c_num = complex_number::ComplexNumber::new(1.0, -1.0);
        assert_eq!(format!("{:?}", c_num), "ComplexNumber { real: 1.0, imaginary: -1.0 }");

        let c_num = complex_number::ComplexNumber::new(-1.0, -1.0);
        assert_eq!(format!("{:?}", c_num), "ComplexNumber { real: -1.0, imaginary: -1.0 }");

        let c_num = complex_number::ComplexNumber::new(-0.0, -0.0);
        assert_eq!(format!("{:?}", c_num), "ComplexNumber { real: -0.0, imaginary: -0.0 }");

        let c_num = complex_number::ComplexNumber::new(0.12, -0.34);
        assert_eq!(format!("{:?}", c_num), "ComplexNumber { real: 0.12, imaginary: -0.34 }");
    }

    #[test]
    fn add_complex_numbers(){
        let a = complex_number::ComplexNumber::new(2.0, 3.0);
        let b = complex_number::ComplexNumber::new(4.0, 6.0);
        let sum = complex_number::ComplexNumber::new(6.0, 9.0);

        assert_eq!(a.add(&b), sum);
    }

    #[test]
    fn subtract_complex_numbers(){
        let a = complex_number::ComplexNumber::new(2.0, 3.0);
        let b = complex_number::ComplexNumber::new(4.0, 6.0);
        let difference = complex_number::ComplexNumber::new(-2.0, -3.0);

        assert_eq!(a.sub(&b), difference);
    }

    #[test]
    fn multiply_complex_numbers(){
        let a = complex_number::ComplexNumber::new(2.0, 3.0);
        let b = complex_number::ComplexNumber::new(4.0, 6.0);
        let product = complex_number::ComplexNumber::new(-10.0, 24.0);

        assert_eq!(a.mul(&b), product);
    }

    #[test]
    fn modulus_complex_numbers(){
        let a = complex_number::ComplexNumber::new(3.0, -4.0);
        assert_eq!(a.modulus(), 5.0);
    }

    #[test]
    fn division_complex_numbers(){
        let a = complex_number::ComplexNumber::new(-2.0, 1.0);
        let b = complex_number::ComplexNumber::new(1.0, 2.0);
        let quotient = complex_number::ComplexNumber::new(0.0, 1.0);

        assert_eq!(a.div(&b), quotient);
    }

    #[test]
    fn conjugate_complex_numbers(){
        let a = complex_number::ComplexNumber::new(-2.0, 1.0);
        let conj = complex_number::ComplexNumber::new(-2.0, -1.0);
 
        assert_eq!(a.conjugate(), conj);

    }
}