
use crate::complex_number::ComplexNumber;

pub fn add(a_vec:&Vec<ComplexNumber>, b_vec:&Vec<ComplexNumber>) -> Vec<ComplexNumber> {
    a_vec.iter().zip(b_vec.iter()).map(|(a, b)| a.add(&b)).collect()
}

pub fn inverse(a_vec:&Vec<ComplexNumber>) -> Vec<ComplexNumber> {
    a_vec.iter().map(|a| a.mul(&ComplexNumber::new(-1.0, 0.0))).collect()
}

pub fn scalar_mul(scalar: &ComplexNumber, vec:&Vec<ComplexNumber>) -> Vec<ComplexNumber> {
    vec.iter().map(|v| scalar.mul(&v)).collect()
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_add(){
        let a_vec =vec![ComplexNumber::new(1.0, 1.0), ComplexNumber::new(-2.0, -2.0), ComplexNumber::new(3.0, 4.0)];
        let b_vec =vec![ComplexNumber::new(1.0, 1.0), ComplexNumber::new(-5.0, 4.0), ComplexNumber::new(2.0, 7.0)];
        let sum_vec =vec![ComplexNumber::new(2.0, 2.0), ComplexNumber::new(-7.0, 2.0), ComplexNumber::new(5.0, 11.0)];
        
        assert_eq!(sum_vec, add(&a_vec, &b_vec));
    }

    #[test]
    fn test_inverse(){
        let a_vec = vec![ComplexNumber::new(1.0, 1.0), ComplexNumber::new(-2.0, -2.0), ComplexNumber::new(3.0, 4.0)];
        let inverse_a_vec = inverse(&a_vec);
        let zero_vec = vec![ComplexNumber::new(0.0, 0.0), ComplexNumber::new(0.0, 0.0), ComplexNumber::new(0.0, 0.0)];
        assert_eq!(zero_vec, add(&a_vec, &inverse_a_vec));
    }

    #[test]
    fn test_scalar_mul(){
        let a_vec = vec![ComplexNumber::new(1.0, 1.0), ComplexNumber::new(-2.0, -2.0), ComplexNumber::new(3.0, 4.0)];
        let scalar = ComplexNumber::new(1.0, 0.0);
        let product = vec![ComplexNumber::new(1.0, 1.0), ComplexNumber::new(-2.0, -2.0), ComplexNumber::new(3.0, 4.0)];
        assert_eq!(product, scalar_mul(&scalar, &a_vec));

        let scalar = ComplexNumber::new(2.0, 2.0);
        let product = vec![ComplexNumber::new(0.0, 4.0), ComplexNumber::new(0.0, -8.0), ComplexNumber::new(-2.0, 14.0)];
        assert_eq!(product, scalar_mul(&scalar, &a_vec));
    }
}