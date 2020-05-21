use noise::{NoiseFn, Perlin};
use rand;

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
