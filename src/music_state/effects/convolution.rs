use num::complex::Complex;

use super::fft::{fft_64, ifft_64};
use super::ring_buffer::RingBuffer;
use super::Effect;

const FRAMES_PER_BUFFER: usize = 512;

pub struct ConvolutionEffect {
    fft_filter: Vec<Vec<Complex<f64>>>,
    fft_left_sample: RingBuffer<Vec<Complex<f64>>>,
    fft_right_sample: RingBuffer<Vec<Complex<f64>>>,
    // block_size: usize,
}

impl ConvolutionEffect {
    pub fn new(filter: &Vec<f32>) -> Self {
        let mut fft_filter = vec![];
        let block_size = (filter.len() - 1) / FRAMES_PER_BUFFER + 1;
        for block_idx in 0..block_size {
            let mut fft_filter_ = vec![Complex::new(0.0, 0.0); FRAMES_PER_BUFFER * 2];

            for sample_idx in 0..FRAMES_PER_BUFFER {
                if sample_idx + block_idx * FRAMES_PER_BUFFER < filter.len() {
                    fft_filter_[sample_idx] = Complex::new(
                        filter[sample_idx + block_idx * FRAMES_PER_BUFFER] as f64,
                        0.0,
                    );
                }
            }
            fft_filter.push(fft_64(&fft_filter_));
        }

        let fft_left_sample = RingBuffer::new(
            block_size,
            vec![Complex::new(0.0, 0.0); FRAMES_PER_BUFFER * 2],
        );
        let fft_right_sample = RingBuffer::new(
            block_size,
            vec![Complex::new(0.0, 0.0); FRAMES_PER_BUFFER * 2],
        );

        Self {
            fft_filter,
            fft_left_sample,
            fft_right_sample,
            // block_size,
        }
    }
}

impl Effect for ConvolutionEffect {
    fn effect(&mut self, left_wave: &Vec<f32>, right_wave: &Vec<f32>) -> (Vec<f32>, Vec<f32>) {
        let mut fft_left_sample_ = vec![Complex::new(0.0, 0.0); FRAMES_PER_BUFFER * 2];
        let mut fft_right_sample_ = vec![Complex::new(0.0, 0.0); FRAMES_PER_BUFFER * 2];
        for sample_idx in 0..FRAMES_PER_BUFFER {
            fft_left_sample_[sample_idx] = Complex::new(left_wave[sample_idx] as f64, 0.0);
            fft_right_sample_[sample_idx] = Complex::new(right_wave[sample_idx] as f64, 0.0);
        }
        self.fft_left_sample.push(fft_64(&fft_left_sample_));
        self.fft_right_sample.push(fft_64(&fft_right_sample_));

        let mut convolued_fft_left_sample = vec![Complex::new(0.0, 0.0); FRAMES_PER_BUFFER * 2];
        for (fft_left_sample_, fft_filter_) in
            self.fft_left_sample.iter().zip(self.fft_filter.iter())
        {
            for sample_idx in 0..FRAMES_PER_BUFFER * 2 {
                convolued_fft_left_sample[sample_idx] +=
                    fft_left_sample_[sample_idx] * fft_filter_[sample_idx];
            }
        }
        let convolved_left_sample = ifft_64(&convolued_fft_left_sample);

        let mut convolued_fft_right_sample = vec![Complex::new(0.0, 0.0); FRAMES_PER_BUFFER * 2];
        for (fft_right_sample_, fft_filter_) in
            self.fft_right_sample.iter().zip(self.fft_filter.iter())
        {
            for sample_idx in 0..FRAMES_PER_BUFFER * 2 {
                convolued_fft_right_sample[sample_idx] +=
                    fft_right_sample_[sample_idx] * fft_filter_[sample_idx];
            }
        }
        let convolved_right_sample = ifft_64(&convolued_fft_right_sample);

        let mut new_left_wave = vec![];
        let mut new_right_wave = vec![];
        for sample_idx in 0..FRAMES_PER_BUFFER {
            new_left_wave.push(convolved_left_sample[sample_idx].re as f32);
            new_right_wave.push(convolved_right_sample[sample_idx].re as f32);
        }

        (new_left_wave, new_right_wave)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convolution() {
        let mut filter = vec![];
        for i in 0..700 {
            filter.push((i + 1) as f32);
        }
        let mut conv_effect = ConvolutionEffect::new(&filter);

        let mut input1 = vec![];
        let mut input2 = vec![];
        let mut input3 = vec![];
        let mut input4 = vec![];
        for i in 0..512 {
            input1.push((i + 1) as f32);
            input2.push((i + 512 + 1) as f32);
            input3.push(0.0);
            input4.push(0.0);
        }

        let (output1, _) = conv_effect.effect(&input1, &input1);
        let (output2, _) = conv_effect.effect(&input2, &input2);
        let (output3, _) = conv_effect.effect(&input3, &input3);
        let (output4, _) = conv_effect.effect(&input4, &input4);

        // println!("{:?}", output1);
        // println!("{:?}", output2);
        println!("{:?}", output3);
        println!("{:?}", output4);
    }
}
