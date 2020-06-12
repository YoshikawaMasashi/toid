use num::complex::Complex;

use super::fft::{fft, ifft};
use super::ring_buffer::RingBuffer;
use super::Effect;

const FRAMES_PER_BUFFER: usize = 512;

pub struct ConvolutionEffect {
    fft_filter: Vec<Vec<Complex<f32>>>,
    fft_left_sample: RingBuffer<Vec<Complex<f32>>>,
    fft_right_sample: RingBuffer<Vec<Complex<f32>>>,
    // block_size: usize,
}

impl ConvolutionEffect {
    pub fn new(filter: Vec<f32>) -> Self {
        let mut fft_filter = vec![];
        let block_size = (filter.len() - 1) / FRAMES_PER_BUFFER + 1;
        for block_idx in 0..block_size {
            let mut fft_filter_ = vec![Complex::new(0.0, 0.0); FRAMES_PER_BUFFER * 2];

            for sample_idx in 0..FRAMES_PER_BUFFER {
                if sample_idx + block_idx * FRAMES_PER_BUFFER < filter.len() {
                    fft_filter_[sample_idx] =
                        Complex::new(filter[sample_idx + block_idx * FRAMES_PER_BUFFER], 0.0);
                }
            }
            fft_filter.push(fft(&fft_filter_));
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
            fft_left_sample_[sample_idx] = Complex::new(left_wave[sample_idx], 0.0);
            fft_right_sample_[sample_idx] = Complex::new(right_wave[sample_idx], 0.0);
        }
        self.fft_left_sample.push(fft(&fft_left_sample_));
        self.fft_right_sample.push(fft(&fft_right_sample_));

        let mut convolued_fft_left_sample = vec![Complex::new(0.0, 0.0); FRAMES_PER_BUFFER * 2];
        for (fft_left_sample_, fft_filter_) in self
            .fft_left_sample
            .iter()
            .rev()
            .zip(self.fft_filter.iter())
        {
            for sample_idx in 0..FRAMES_PER_BUFFER * 2 {
                convolued_fft_left_sample[sample_idx] =
                    fft_left_sample_[sample_idx] * fft_filter_[sample_idx];
            }
        }
        let convolved_left_sample = ifft(&convolued_fft_left_sample);

        let mut convolued_fft_right_sample = vec![Complex::new(0.0, 0.0); FRAMES_PER_BUFFER * 2];
        for (fft_right_sample_, fft_filter_) in self
            .fft_right_sample
            .iter()
            .rev()
            .zip(self.fft_filter.iter())
        {
            for sample_idx in 0..FRAMES_PER_BUFFER * 2 {
                convolued_fft_right_sample[sample_idx] =
                    fft_right_sample_[sample_idx] * fft_filter_[sample_idx];
            }
        }
        let convolved_right_sample = ifft(&convolued_fft_right_sample);

        let mut new_left_wave = vec![];
        let mut new_right_wave = vec![];
        for sample_idx in 0..FRAMES_PER_BUFFER {
            new_left_wave.push(convolved_left_sample[sample_idx].re);
            new_right_wave.push(convolved_right_sample[sample_idx].re);
        }

        (new_left_wave, new_right_wave)
    }
}
