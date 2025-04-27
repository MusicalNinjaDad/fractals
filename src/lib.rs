mod lim;

use num::complex::{Complex64, ComplexFloat};
use lim::unbounded;

pub fn mandelbrot(c: Complex64) -> bool {
    match unbounded(100, |z: Complex64| {z.powu(2) + c}, |z| {z.abs() >= 2.0}) {
        Some(_) => false,
        None => true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert!(mandelbrot(0.0.into()))
    }

    #[test]
    fn point4() {
        assert!(!mandelbrot(0.4.into()))
    }

    #[test]
    fn top_bulb() {
        assert!(mandelbrot(Complex64::new(-0.13, 0.77)))
    }

    #[test]
    fn outside() {
        assert!(!mandelbrot(Complex64::new(-1., 0.55)))
    }
}
