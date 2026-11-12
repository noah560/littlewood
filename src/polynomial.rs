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
    let mut normalized: [Complex; N + 1] = [
        Complex::from_f32(0.0, 0.0); N + 1
    ];
    for i in 0..(N + 1) {
        normalized[i] = coefficients[i] / coefficients[N];
    }
    let mut guesses: [Complex; N] = [
        Complex::from_f32(1.0, 0.0); N
    ];
    let multiplier = Complex::from_f32(0.4, 0.9);
    for i in 1..N {
        guesses[i] = multiplier * guesses[i - 1];
    }
    for _ in 0..iterations {
        let mut new_guesses: [Complex; N] = [
            Complex::from_f32(0.0, 0.0); N
        ];
        for i in 0..N {
            let mut p: Complex = Complex::from_f32(0.0, 0.0);
            for c in &normalized {
                p = p * guesses[i] + *c;
            }
            let mut d: Complex = Complex::from_f32(1.0, 0.0);
            for j in 0..N {
                if j != i {
                    d = d * (guesses[i] - guesses[j]);
                }
            }
            new_guesses[i] = guesses[i] - p / d;
        }
        let mut max_diff: f32 = (new_guesses[0] - guesses[0]).abs();
        for i in 1..N {
            let d = (new_guesses[i] - guesses[i]).abs();
            if d > max_diff {
                max_diff = d;
            }
        }
        guesses = new_guesses;
        if max_diff < 0.000001 {
            return guesses;
        }
    }
    guesses
}
