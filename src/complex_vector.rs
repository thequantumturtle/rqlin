
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