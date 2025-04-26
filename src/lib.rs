use num::complex::{Complex, Complex64, ComplexFloat};

pub fn mandelbrot(c: Complex64) -> bool {
    let mut z: Complex64 = Complex::ZERO;
    let mut i: u8 = 0;
    while z.abs() < 2.0 && i < 100 {
        z = z * z + c;
        i += 1;
    }
    i == 100
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
