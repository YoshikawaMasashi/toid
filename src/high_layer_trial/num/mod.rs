use noise::{NoiseFn, Perlin};
use rand;

use super::super::data::music_info::Pitch;

pub fn parlin_noise_seq(size: usize, degree: f32, seed: Option<f32>) -> Vec<f32> {
    let mut seed = match seed {
        Some(seed) => seed,
        None => rand::random::<f32>(),
    };

    let mut noise = vec![];
    let perlin = Perlin::new();

    for _i in 0..size {
        seed += degree;
        noise.push(perlin.get([seed as f64, 0.0]) as f32);
    }

    noise
}

pub fn max_min_normalize(vec: &Vec<f32>) -> Vec<f32> {
    let mut new_vec = vec![];

    let max = vec.iter().fold(0.0 as f32, |max, &i| max.max(i));
    let min = vec.iter().fold(0.0 as f32, |min, &i| min.min(i));
    let diff = max - min;

    for &v in vec.iter() {
        new_vec.push((v - min) / diff);
    }

    new_vec
}

pub fn change_max_min(vec: &Vec<f32>, new_max: f32, new_min: f32) -> Vec<f32> {
    let mut new_vec = vec![];

    let max = vec.iter().fold(0.0 as f32, |max, &i| max.max(i));
    let min = vec.iter().fold(0.0 as f32, |min, &i| min.min(i));
    let diff = max - min;

    let new_diff = new_max - new_min;

    for &v in vec.iter() {
        new_vec.push((v - min) / diff * new_diff + new_min);
    }

    new_vec
}

pub fn f32_vec_to_pitch_vec(vec: &Vec<f32>) -> Vec<Pitch> {
    let mut new_vec = vec![];

    for &v in vec.iter() {
        new_vec.push(Pitch::from(v));
    }

    new_vec
}
