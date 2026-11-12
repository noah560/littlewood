#![feature(generic_const_exprs)]

mod polynomial;
mod complex;

use complex::Complex;
use rayon::prelude::*;

const POLYNOMIAL_SIZE: usize = 12;
const RESOLUTION: usize = 3200;
const BATCH_SIZE: usize = 1024;

fn solve_polynomial(
    iterations_per_degree: usize,
    allowed_coefficients: &[Complex],
    iteration: usize,
) -> Option<(usize, u8, u8, u8)> {
    // Get the relevant coefficients
    let mut coefficients: [Complex; POLYNOMIAL_SIZE] = [
        Complex::from((0.0, 0.0)); POLYNOMIAL_SIZE
    ];
    let mut remaining = iteration;
    for i in 0..POLYNOMIAL_SIZE {
        let id = remaining % allowed_coefficients.len();
        coefficients[POLYNOMIAL_SIZE - i - 1] = allowed_coefficients[id];
        remaining /= allowed_coefficients.len();
    }
    // Get the color
    let color: (u8, u8, u8) = get_color(iteration / iterations_per_degree);
    // Solve the equation
    let solution: (f32, f32) = polynomial::solve(
        &coefficients,
        32
    ).into();
    if solution.0 <= -2.0 || solution.0 >= 2.0 || solution.1 <= -2.0 || solution.1 >= 2.0 {
        return None;
    }
    let pixel: (usize, usize) = (
        (solution.0 * (RESOLUTION as f32) / 4.0 + (RESOLUTION as f32) / 2.0) as usize,
        ((RESOLUTION as f32) / 2.0 - solution.1 * (RESOLUTION as f32) / 4.0) as usize,
    );
    let pixel_id: usize = pixel.1 * RESOLUTION * 4 + pixel.0 * 4;
    Some((pixel_id, color.0, color.1, color.2))
}

fn solve_polynomials(
    iterations_per_degree: usize,
    allowed_coefficients: &[Complex],
    iterations: &[usize],
    results: &mut [Option<(usize, u8, u8, u8)>],
) {
    results.par_iter_mut().zip(iterations).for_each(|(result, &iteration)| {
        *result = solve_polynomial(
            iterations_per_degree,
            allowed_coefficients,
            iteration,
        );
    });
}

fn get_color(degree: usize) -> (u8, u8, u8) {
    let d: f32 = degree as f32;
    let mut r: f32 = 0.0;
    let mut g: f32 = 0.0;
    let mut b: f32 = 0.0;
    if degree < 60 {
        r = 255.0;
        g = 4.25 * d;
    } else if degree < 120 {
        g = 255.0;
        r = 255.0 - 4.25 * (d - 60.0);
    } else if degree < 180 {
        g = 255.0;
        b = 4.25 * (d - 120.0);
    } else if degree < 240 {
        b = 255.0;
        g = 255.0 - 4.25 * (d - 180.0);
    } else if degree < 300 {
        b = 255.0;
        r = 4.25 * (d - 240.0);
    } else { // degree < 360
        r = 255.0;
        b = 255.0 - 4.25 * (d - 300.0);
    }
    (r as u8, g as u8, b as u8)
}

fn main() {
    let mut allowed_coefficients: [Complex; 3] = [
        Complex::from((-1.0, 0.0)),
        Complex::from((1.0, 0.0)),
        Complex::from((0.0, -1.0)),
    ];
    for frame in 0..60 {
        let mut framebuffer: Vec<u8> = vec![0; RESOLUTION * RESOLUTION * 4];
        // Set alpha values to 255
        for i in 0..(RESOLUTION * RESOLUTION) {
            framebuffer[i * 4 + 3] = 255;
        }
        // Run iterations
        let total_iterations: usize = allowed_coefficients.len().pow(POLYNOMIAL_SIZE as u32);
        let complete_batches: usize = total_iterations / BATCH_SIZE;
        let iterations_per_degree: usize = total_iterations / 360;
        for batch in 0..complete_batches {
            let mut iterations: [usize; BATCH_SIZE] = [0; BATCH_SIZE];
            for i in 0..BATCH_SIZE {
                iterations[i] = batch * BATCH_SIZE + i;
            }
            let mut results: [Option<(usize, u8, u8, u8)>; BATCH_SIZE] = [None; BATCH_SIZE];
            solve_polynomials(
                iterations_per_degree,
                &allowed_coefficients,
                &iterations,
                &mut results,
            );
            for &root in &results {
                match root {
                    Some((index, r, g, b)) => {
                        framebuffer[index + 0] = r;
                        framebuffer[index + 1] = g;
                        framebuffer[index + 2] = b;
                    },
                    _ => {}
                }
            }
        }
        // Save image
        image::save_buffer(
            &std::path::Path::new(&format!("frame_{:0>8}.png", frame)),
            &framebuffer,
            RESOLUTION as u32,
            RESOLUTION as u32,
            image::ExtendedColorType::Rgba8
        );
        println!("Generated frame {}!", frame);
        // Change coefficients for the animation
        allowed_coefficients[1] = allowed_coefficients[1] + Complex::from((-0.05, 0.05));
    }
}
