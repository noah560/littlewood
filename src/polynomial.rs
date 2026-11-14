use crate::complex::Complex;
use rand::random_range;

pub fn value(
    coefficients: &[Complex],
    input: Complex,
) -> Complex {
    let mut sum = Complex::from((0.0, 0.0));
    let mut power = Complex::from((1.0, 0.0));
    for i in 0..coefficients.len() {
        sum = sum + power * coefficients[i];
        power = power * input;
    }
    sum
}

pub fn derivative(
    coefficients: &[Complex],
    input: Complex,
) -> Complex {
    if coefficients.len() <= 1 {
        return Complex::from((0.0, 0.0));
    }
    let mut sum = Complex::from((0.0, 0.0));
    let mut power = Complex::from((1.0, 0.0));
    for i in 1..coefficients.len() {
        sum = sum + power * coefficients[i] * Complex::from((i as f32, 0.0));
        power = power * input;
    }
    sum
}

pub fn solve(
    coefficients: &[Complex],
    iterations: i32,
) -> Complex {
    let mut guess = Complex::from((
        random_range(-2.0..2.0),
        random_range(-2.0..2.0),
    ));
    for _i in 0..iterations {
        guess = guess - value(coefficients, guess) / derivative(coefficients, guess);
    }
    guess
}

pub fn solve_all<const N: usize>(
    coefficients: &[Complex],
    iterations: i32,
) -> [Complex; N] where [(); N + 1]: {
    if coefficients.len() != (N + 1) {
        panic!("Wrong number of coefficients!");
    }
    let mut roots: [Complex; N] = [
        Complex::from_f32(1.0, 0.0); N
    ];
    let multiplier = Complex::from_f32(
        f32::cos(1.0),
        f32::sin(1.0),
    );
    for i in 1..roots.len() {
        roots[i] = roots[i - 1] * multiplier;
    }
    let one = Complex::from_f32(1.0, 0.0);
    for _iteration in 0..iterations {
        for i in 0..roots.len() {
            let newton = value(coefficients, roots[i]) / derivative(coefficients, roots[i]);
            let mut repulsion = Complex::from_f32(0.0, 0.0);
            for j in 0..roots.len() {
                if i != j {
                    repulsion = repulsion + one / (
                        roots[i] - roots[j]
                    );
                }
            }
            roots[i] = roots[i] - newton / (
                one - (newton * repulsion)
            );
        }
    }
    roots
}
