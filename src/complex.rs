
use std::fmt;


#[derive(Debug, PartialEq)]
pub struct Complex{
    pub real: f32,
    pub imaginary: f32,
}

impl fmt::Display for Complex{
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result{
        if self.imaginary < 0.0
        {
            write!(f, "{}{}i", self.real, self.imaginary)
        }else{
            write!(f, "{}+{}i", self.real, self.imaginary)
        }
    }
}

impl Complex{
    pub fn new(r:f32, i:f32) -> Complex{
        Complex{
            real:r, 
            imaginary:i,
        }
    }

    pub fn new_from_polar(ρ:f32, θ:f32) -> Complex{
        let (a, b) = Complex::polar_to_cartesian(ρ, θ);
        Complex::new(a, b)
    }

    pub fn add(&self, b:&Complex) -> Complex{
        let real = self.real + b.real;
        let imaginary = self.imaginary + b.imaginary;
        Complex::new(real, imaginary)
    }

    pub fn sub(&self, b:&Complex) -> Complex{
        let neg_b = Complex::new(-b.real, -b.imaginary);
        Complex::add(&self, &neg_b)
    }

    pub fn mul(&self, b:&Complex) -> Complex{
        let real = self.real * b.real + self.imaginary * b.imaginary * -1.0;
        let imaginary = self.real * b.imaginary + self.imaginary * b.real;
        Complex::new(real, imaginary)
    }

    pub fn div(&self, b:&Complex) -> Complex{
        let divisor = b.modulus().powf(2.0);
        let real = (self.real * b.real + self.imaginary * b.imaginary)/divisor;
        let imaginary = (b.real * self.imaginary - self.real * b.imaginary)/divisor;
        Complex::new(real, imaginary)
    }

    pub fn modulus(&self) -> f32{
        (self.real.powf(2.0) + self.imaginary.powf(2.0)).sqrt()
    }

    pub fn conjugate(&self) -> Complex{
        Complex::new(self.real, -self.imaginary)
    }

    pub fn get_cartesian(&self) -> (f32,f32){
        (self.real, self.imaginary)
    }

    pub fn get_polar(&self) -> (f32, f32){
        Complex::cartesian_to_polar(self.real, self.imaginary)
    }

    pub fn cartesian_to_polar(a:f32, b:f32) -> (f32, f32) {
        let c = Complex::new(a, b);
        (c.modulus(), c.imaginary.atan2(c.real))
    }

    pub fn polar_to_cartesian(ρ:f32, θ:f32 ) -> (f32, f32){
        (ρ*θ.cos(), ρ*θ.sin())
    }
}





#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn create_complex(){
        let c_num = Complex::new(2.0, 3.0);
        assert_eq!(c_num.real, 2.0);
        assert_eq!(c_num.imaginary, 3.0);
    }

    #[test]
    fn create_complex_from_polar(){
        // finding it difficult to find a more meaningful test due to floating point errors
        let c = Complex::new(2.0, 0.0);
        assert_eq!(c.real, 2.0);
        assert_eq!(c.imaginary, 0.0);
    }

    #[test]
    fn formatter_display_complex(){
        let c_num = Complex::new(0.0, 0.0);
        assert_eq!(c_num.to_string(), "0+0i");
        
        let c_num = Complex::new(1.0, 1.0);
        assert_eq!(c_num.to_string(), "1+1i");
        
        let c_num = Complex::new(1.0, -1.0);
        assert_eq!(c_num.to_string(), "1-1i");
        
        let c_num = Complex::new(-1.0, -1.0);
        assert_eq!(c_num.to_string(), "-1-1i");

        let c_num = Complex::new(-0.0, -0.0);
        assert_eq!(c_num.to_string(), "-0+-0i");
        
        let c_num = Complex::new(0.12, -0.34);
        assert_eq!(c_num.to_string(), "0.12-0.34i");
    }

    #[test]
    fn formatter_debug_complex(){
        let c_num = Complex::new(0.0, 0.0);
        assert_eq!(format!("{:?}", c_num), "Complex { real: 0.0, imaginary: 0.0 }");

        let c_num = Complex::new(1.0, 1.0);
        assert_eq!(format!("{:?}", c_num), "Complex { real: 1.0, imaginary: 1.0 }");

        let c_num = Complex::new(1.0, -1.0);
        assert_eq!(format!("{:?}", c_num), "Complex { real: 1.0, imaginary: -1.0 }");

        let c_num = Complex::new(-1.0, -1.0);
        assert_eq!(format!("{:?}", c_num), "Complex { real: -1.0, imaginary: -1.0 }");

        let c_num = Complex::new(-0.0, -0.0);
        assert_eq!(format!("{:?}", c_num), "Complex { real: -0.0, imaginary: -0.0 }");

        let c_num = Complex::new(0.12, -0.34);
        assert_eq!(format!("{:?}", c_num), "Complex { real: 0.12, imaginary: -0.34 }");
    }

    #[test]
    fn add_complex(){
        let a = Complex::new(2.0, 3.0);
        let b = Complex::new(4.0, 6.0);
        let sum = Complex::new(6.0, 9.0);

        assert_eq!(a.add(&b), sum);
    }

    #[test]
    fn subtract_complex(){
        let a = Complex::new(2.0, 3.0);
        let b = Complex::new(4.0, 6.0);
        let difference = Complex::new(-2.0, -3.0);

        assert_eq!(a.sub(&b), difference);
    }

    #[test]
    fn multiply_complex(){
        let a = Complex::new(2.0, 3.0);
        let b = Complex::new(4.0, 6.0);
        let product = Complex::new(-10.0, 24.0);

        assert_eq!(a.mul(&b), product);
    }

    #[test]
    fn modulus_complex(){
        let a = Complex::new(3.0, -4.0);
        assert_eq!(a.modulus(), 5.0);
    }

    #[test]
    fn division_complex(){
        let a = Complex::new(-2.0, 1.0);
        let b = Complex::new(1.0, 2.0);
        let quotient = Complex::new(0.0, 1.0);

        assert_eq!(a.div(&b), quotient);
    }

    #[test]
    fn conjugate_complex(){
        let a = Complex::new(-2.0, 1.0);
        let conj = Complex::new(-2.0, -1.0);

        assert_eq!(a.conjugate(), conj);

    }

    #[test]
    fn test_get_cartesian(){
        let c = Complex::new(2.0, 3.0);
        let (a, b) = c.get_cartesian();
        assert_eq!(a, c.real);
        assert_eq!(b, c.imaginary);
    }

    #[test]
    fn test_get_polar(){
        // this is a really bad example where the cartesian and polar coordinates look the same
        let c = Complex::new(2.0, 0.0);
        let (ρ, θ) = c.get_polar();
        assert_eq!(2.0, ρ);
        assert_eq!(0.0, θ);
    }

    #[test]
    fn test_cartesian_to_polar(){
        // this is a really bad example where the cartesian and polar coordinates look the same
        let (ρ, θ) = Complex::cartesian_to_polar(2.0, 0.0);
        assert_eq!(ρ, 2.0);
        assert_eq!(θ, 0.0);
    }

    #[test]
    fn test_polar_to_cartesian(){
        // this is a really bad example where the cartesian and polar coordinates look the same
        let (a, b) = Complex::polar_to_cartesian(2.0, 0.0);
        assert_eq!(a, 2.0);
        assert_eq!(b, 0.0);        
    }
}