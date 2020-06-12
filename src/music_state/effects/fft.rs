use num::complex::Complex;

pub fn fft(input: &Vec<Complex<f32>>) -> Vec<Complex<f32>> {
    let PI = std::f32::consts::PI;

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
    let PI = std::f32::consts::PI;

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

pub fn fft_64(input: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let PI = std::f64::consts::PI;

    let size = input.len();
    let mut butterfly_idx: Vec<usize> = vec![];
    butterfly_idx.push(0);
    let mut butterfly_size = size / 2;
    while butterfly_size >= 1 {
        let new_butterfly_idx = butterfly_idx.iter().map(|x| x + butterfly_size).collect();
        butterfly_idx = [butterfly_idx, new_butterfly_idx].concat();
        butterfly_size = butterfly_size / 2;
    }

    let mut output: Vec<Complex<f64>> = butterfly_idx.iter().map(|&i| input[i]).collect();

    let mut butterfly_step = 2;
    while butterfly_step <= size {
        let butterfly_neighbor = butterfly_step / 2;
        for i in 0..size / 2 {
            let left_idx = i / butterfly_neighbor * butterfly_step + i % butterfly_neighbor;
            let right_idx = left_idx + butterfly_neighbor;
            let left_exponents = left_idx % butterfly_step;
            let right_exponents = right_idx % butterfly_step;

            let left_w =
                (-2.0 * PI * Complex::i() * left_exponents as f64 / butterfly_step as f64).exp();
            let right_w =
                (-2.0 * PI * Complex::i() * right_exponents as f64 / butterfly_step as f64).exp();

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

pub fn ifft_64(input: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let PI = std::f64::consts::PI;

    let size = input.len();
    let mut butterfly_idx: Vec<usize> = vec![];
    butterfly_idx.push(0);
    let mut butterfly_size = size / 2;
    while butterfly_size >= 1 {
        let new_butterfly_idx = butterfly_idx.iter().map(|x| x + butterfly_size).collect();
        butterfly_idx = [butterfly_idx, new_butterfly_idx].concat();
        butterfly_size = butterfly_size / 2;
    }

    let mut output: Vec<Complex<f64>> = butterfly_idx.iter().map(|&i| input[i]).collect();

    let mut butterfly_step = 2;
    while butterfly_step <= size {
        let butterfly_neighbor = butterfly_step / 2;
        for i in 0..size / 2 {
            let left_idx = i / butterfly_neighbor * butterfly_step + i % butterfly_neighbor;
            let right_idx = left_idx + butterfly_neighbor;
            let left_exponents = left_idx % butterfly_step;
            let right_exponents = right_idx % butterfly_step;

            let left_w =
                (2.0 * PI * Complex::i() * left_exponents as f64 / butterfly_step as f64).exp();
            let right_w =
                (2.0 * PI * Complex::i() * right_exponents as f64 / butterfly_step as f64).exp();

            let left = output[left_idx];
            let right = output[right_idx];
            let new_left = left + right * left_w;
            let new_right = left + right * right_w;
            output[left_idx] = new_left;
            output[right_idx] = new_right;
        }

        butterfly_step *= 2;
    }

    output = output.iter().map(|x| x / size as f64).collect();

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
        // println!("{:?}", output);

        // array([28.+0.j        , -4.+9.65685425j, -4.+4.j        , -4.+1.65685425j,
        // -4.+0.j        , -4.-1.65685425j, -4.-4.j        , -4.-9.65685425j])
    }

    #[test]
    fn test_ifft() {
        let mut input = vec![];
        for i in 0..1024 {
            input.push(Complex::new(i as f32, 0.0));
        }
        let fft_output = fft(&input);
        let ifft_output = ifft(&fft_output);
        // println!("{:?}", ifft_output.iter().map(|x| x.re).collect::<Vec<f32>>());
    }

    #[test]
    fn test_conv() {
        let size = 512;

        let mut x1 = vec![];
        let mut x2 = vec![];
        for i in 0..size {
            x1.push(Complex::new((i + 1) as f64, 0.0));
            x2.push(Complex::new((i + 1) as f64, 0.0));
        }
        for i in 0..size {
            x1.push(Complex::new(0.0, 0.0));
            x2.push(Complex::new(0.0, 0.0));
        }

        let x1 = fft_64(&x1);
        let x2 = fft_64(&x2);
        // println!("{:?}", x1.split_at(16).0);
        // println!("{:?}", x1.split_at(1024 - 16).1);

        let ix1 = ifft_64(&x1);
        // println!("{:?}", ix1.split_at(16).0);
        // println!("{:?}", ix1.split_at(1024 - 16).1);

        let mut x3 = vec![];
        for i in 0..size * 2 {
            x3.push(x1[i] * x2[i]);
        }
        let x3 = ifft_64(&x3);

        // println!("{:?}", x3.iter().map(|x| x.re).collect::<Vec<f64>>());
    }
}
