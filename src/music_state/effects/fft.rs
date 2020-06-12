use std::f32::consts::PI;

use num::complex::Complex;

pub fn fft(input: &Vec<Complex<f32>>) -> Vec<Complex<f32>> {
    let size = input.len();
    let mut butterfly_idx: Vec<usize> = vec![];
    butterfly_idx.push(0);
    let mut butterfly_size = size / 2;
    while butterfly_size >= 1 {
        let new_butterfly_idx = butterfly_idx.iter().map(|x| x + butterfly_size).collect();
        butterfly_idx = [butterfly_idx, new_butterfly_idx].concat();
        butterfly_size = butterfly_size / 2;
    }

    let mut output: Vec<Complex<f32>> = butterfly_idx.iter().map(|&i| input[i]).collect();

    let mut butterfly_step = 2;
    while butterfly_step <= size {
        let butterfly_neighbor = butterfly_step / 2;
        for i in 0..size / 2 {
            let left_idx = i / butterfly_neighbor * butterfly_step + i % butterfly_neighbor;
            let right_idx = left_idx + butterfly_neighbor;
            let left_exponents = left_idx % butterfly_step;
            let right_exponents = right_idx % butterfly_step;

            let left_w =
                (-2.0 * PI * Complex::i() * left_exponents as f32 / butterfly_step as f32).exp();
            let right_w =
                (-2.0 * PI * Complex::i() * right_exponents as f32 / butterfly_step as f32).exp();

            let left = output[left_idx];
            let right = output[right_idx];
            let new_left = left + right * left_w;
            let new_right = left + right * right_w;
            output[left_idx] = new_left;
            output[right_idx] = new_right;
        }

        butterfly_step *= 2;
    }

    output
}

pub fn ifft(input: &Vec<Complex<f32>>) -> Vec<Complex<f32>> {
    let size = input.len();
    let mut butterfly_idx: Vec<usize> = vec![];
    butterfly_idx.push(0);
    let mut butterfly_size = size / 2;
    while butterfly_size >= 1 {
        let new_butterfly_idx = butterfly_idx.iter().map(|x| x + butterfly_size).collect();
        butterfly_idx = [butterfly_idx, new_butterfly_idx].concat();
        butterfly_size = butterfly_size / 2;
    }

    let mut output: Vec<Complex<f32>> = butterfly_idx.iter().map(|&i| input[i]).collect();

    let mut butterfly_step = 2;
    while butterfly_step <= size {
        let butterfly_neighbor = butterfly_step / 2;
        for i in 0..size / 2 {
            let left_idx = i / butterfly_neighbor * butterfly_step + i % butterfly_neighbor;
            let right_idx = left_idx + butterfly_neighbor;
            let left_exponents = left_idx % butterfly_step;
            let right_exponents = right_idx % butterfly_step;

            let left_w =
                (2.0 * PI * Complex::i() * left_exponents as f32 / butterfly_step as f32).exp();
            let right_w =
                (2.0 * PI * Complex::i() * right_exponents as f32 / butterfly_step as f32).exp();

            let left = output[left_idx];
            let right = output[right_idx];
            let new_left = left + right * left_w;
            let new_right = left + right * right_w;
            output[left_idx] = new_left;
            output[right_idx] = new_right;
        }

        butterfly_step *= 2;
    }

    output = output.iter().map(|x| x / size as f32).collect();

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft() {
        let mut input = vec![];
        for i in 0..8 {
            input.push(Complex::new(i as f32, 0.0));
        }
        let output = fft(&input);
        println!("{:?}", output);

        // array([28.+0.j        , -4.+9.65685425j, -4.+4.j        , -4.+1.65685425j,
        // -4.+0.j        , -4.-1.65685425j, -4.-4.j        , -4.-9.65685425j])
    }

    #[test]
    fn test_ifft() {
        let mut input = vec![];
        for i in 0..8 {
            input.push(Complex::new(i as f32, 0.0));
        }
        let fft_output = fft(&input);
        let ifft_output = ifft(&fft_output);
        println!("{:?}", ifft_output);
    }
}
