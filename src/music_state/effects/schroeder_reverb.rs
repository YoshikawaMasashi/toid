// "サウンドエフェクトのプログラミング" p177-p188
// https://www.amazon.co.jp/dp/B01IGW5BCU

use rand::prelude::*;

use super::ring_buffer::RingBuffer;
use super::Effect;

const FRAMES_PER_BUFFER: usize = 512;

pub struct SchroederRebervEffect {
    multitap_delay: MultitapDelay,
    comb_filters: Vec<CombFilter>,
    allpass_filters: Vec<AllpassFilter>,
}

impl SchroederRebervEffect {
    fn new() -> SchroederRebervEffect {
        let mut rng = rand::thread_rng();

        let mut multitap_delay: Vec<usize> = vec![];
        let mut multitap_amp: Vec<f32> = vec![];
        for i in 0..10 {
            let fluctdelay: f32 = 0.002 * rng.gen::<f32>();
            let fluctamp: f32 = 0.1 * rng.gen::<f32>();
            let delay_sec = 0.020 + (0.008 + fluctdelay) * (i as f32);
            let delay = (delay_sec * 44100.0 + 0.5) as usize;
            let amp = 0.6 + fluctamp + (-0.3) * (i as f32) / 10.0;
            multitap_delay.push(delay);
            multitap_amp.push(amp);
        }
        let multitap_delay = MultitapDelay::new(multitap_delay, multitap_amp);

        let comb_filters = vec![
            CombFilter::new((0.03985 * 44100.0 + 0.5) as usize, 0.871402),
            CombFilter::new((0.03610 * 44100.0 + 0.5) as usize, 0.882762),
            CombFilter::new((0.03327 * 44100.0 + 0.5) as usize, 0.891443),
            CombFilter::new((0.03015 * 44100.0 + 0.5) as usize, 0.901117),
        ];

        let allpass_filters = vec![
            AllpassFilter::new((0.005 * 44100.0 + 0.5) as usize, 0.7),
            AllpassFilter::new((0.0017 * 44100.0 + 0.5) as usize, 0.7),
        ];

        SchroederRebervEffect {
            multitap_delay,
            comb_filters,
            allpass_filters,
        }
    }
}

impl Effect for SchroederRebervEffect {
    fn effect(&mut self, left_wave: &Vec<f32>, right_wave: &Vec<f32>) -> (Vec<f32>, Vec<f32>) {
        let mut new_left_wave = vec![];
        let mut new_right_wave = vec![];
        for i in 0..FRAMES_PER_BUFFER {
            let mut new_left = 0.0;
            let mut new_right = 0.0;

            let (l, r) = self.multitap_delay.filter(left_wave[i], right_wave[i]);
            new_left += l;
            new_right += r;

            let mut rear_left = 0.0;
            let mut rear_right = 0.0;
            for filter in &mut self.comb_filters {
                let (l, r) = filter.filter(left_wave[i], right_wave[i]);
                rear_left += l;
                rear_right += r;
            }

            for filter in &mut self.allpass_filters {
                let (l, r) = filter.filter(rear_left, rear_right);
                rear_left = l;
                rear_right = r;
            }

            new_left_wave.push(new_left);
            new_right_wave.push(new_right);
        }

        (new_left_wave, new_right_wave)
    }
}

pub struct MultitapDelay {
    size: usize,
    delay: Vec<usize>,
    amp: Vec<f32>,
    left_buffer: RingBuffer<f32>,
    right_buffer: RingBuffer<f32>,
}

impl MultitapDelay {
    fn new(delay: Vec<usize>, amp: Vec<f32>) -> MultitapDelay {
        let max_delay = *delay.iter().max().unwrap();
        MultitapDelay {
            size: delay.len(),
            delay,
            amp,
            left_buffer: RingBuffer::new(max_delay, 0.0),
            right_buffer: RingBuffer::new(max_delay, 0.0),
        }
    }
    fn filter(&mut self, left: f32, right: f32) -> (f32, f32) {
        let mut out_left: f32 = 0.0;
        let mut out_right: f32 = 0.0;
        for i in 0..self.size {
            out_left += self.amp[i] * self.left_buffer.get(self.delay[i] - 1).unwrap();
            out_right += self.amp[i] * self.right_buffer.get(self.delay[i] - 1).unwrap();
        }
        self.left_buffer.push(left);
        self.right_buffer.push(right);
        (out_left, out_right)
    }
}

pub struct CombFilter {
    delay: usize,
    amp: f32,
    left_buffer: RingBuffer<f32>,
    right_buffer: RingBuffer<f32>,
}

impl CombFilter {
    fn new(delay: usize, amp: f32) -> CombFilter {
        CombFilter {
            delay,
            amp,
            left_buffer: RingBuffer::new(delay, 0.0),
            right_buffer: RingBuffer::new(delay, 0.0),
        }
    }
    fn filter(&mut self, left: f32, right: f32) -> (f32, f32) {
        let out_left: f32 = *self.left_buffer.get(self.delay - 1).unwrap();
        let out_right: f32 = *self.right_buffer.get(self.delay - 1).unwrap();
        self.left_buffer.push(out_left + self.amp * left);
        self.right_buffer.push(out_right + self.amp * right);
        (out_left, out_right)
    }
}

pub struct AllpassFilter {
    delay: usize,
    amp: f32,
    left_buffer: RingBuffer<f32>,
    right_buffer: RingBuffer<f32>,
}

impl AllpassFilter {
    fn new(delay: usize, amp: f32) -> AllpassFilter {
        AllpassFilter {
            delay,
            amp,
            left_buffer: RingBuffer::new(delay, 0.0),
            right_buffer: RingBuffer::new(delay, 0.0),
        }
    }
    fn filter(&mut self, left: f32, right: f32) -> (f32, f32) {
        let left_before_z: f32 = left + self.amp * self.left_buffer.get(self.delay - 1).unwrap();
        let right_before_z: f32 = right + self.amp * self.right_buffer.get(self.delay - 1).unwrap();
        let out_left = -self.amp * left_before_z + self.left_buffer.get(self.delay - 1).unwrap();
        let out_right = -self.amp * right_before_z + self.right_buffer.get(self.delay - 1).unwrap();
        self.left_buffer.push(left_before_z);
        self.right_buffer.push(right_before_z);
        (out_left, out_right)
    }
}
