// "サウンドエフェクトのプログラミング" p177-p188
// https://www.amazon.co.jp/dp/B01IGW5BCU
use super::ring_buffer::RingBuffer;
use super::Effect;

const FRAMES_PER_BUFFER: usize = 512;


pub struct SchroederRebervEffect {
    multitap_delay: MultitapDelay,
    comb_filters: Vec<CombFilter>,
    allpass_filters: Vec<AllpassFilter>,
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
    fn filter(&mut self, left: f32, right: f32) -> (f32, f32) {
        let mut out_left: f32 = 0.0;
        let mut out_right: f32 = 0.0;
        for i in 0..self.size {
            out_left += self.amp[i] * self.left_buffer.get(self.delay[i]).unwrap();
            out_right += self.amp[i] * self.right_buffer.get(self.delay[i]).unwrap();
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
    fn filter(&mut self, left: f32, right: f32) -> (f32, f32) {
        let out_left: f32 = *self.left_buffer.get(self.delay).unwrap();
        let out_right: f32 = *self.right_buffer.get(self.delay).unwrap();
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
    fn filter(&mut self, left: f32, right: f32) -> (f32, f32) {
        let left_before_z: f32 = left + self.amp * self.left_buffer.get(self.delay).unwrap();
        let right_before_z: f32 = right + self.amp * self.right_buffer.get(self.delay).unwrap();
        let out_left = -self.amp * left_before_z + self.left_buffer.get(self.delay).unwrap();
        let out_right = -self.amp * right_before_z + self.right_buffer.get(self.delay).unwrap();
        self.left_buffer.push(left_before_z);
        self.right_buffer.push(right_before_z);
        (out_left, out_right)
    }
}