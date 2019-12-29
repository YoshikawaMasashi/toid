use std::collections::{BTreeMap, HashSet};
use std::ops::Bound::Included;
use std::sync::Arc;

use super::generator::InstrumentGenerator;

pub struct Instrument {
    name: String,
    generators: Vec<Arc<InstrumentGenerator>>,

    min_key_range_of_gen: Option<Arc<BTreeMap<u8, HashSet<usize>>>>,
    max_key_range_of_gen: Option<Arc<BTreeMap<u8, HashSet<usize>>>>,
    min_vel_range_of_gen: Option<Arc<BTreeMap<u8, HashSet<usize>>>>,
    max_vel_range_of_gen: Option<Arc<BTreeMap<u8, HashSet<usize>>>>,
}

impl Instrument {
    pub fn new() -> Self {
        Instrument {
            name: String::from(""),
            generators: Vec::new(),

            min_key_range_of_gen: None,
            max_key_range_of_gen: None,
            min_vel_range_of_gen: None,
            max_vel_range_of_gen: None,
        }
    }

    pub fn add_generator(&mut self, generator: Arc<InstrumentGenerator>) {
        self.generators.push(generator);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn prepare_gen_range(&mut self) {
        self.prepare_min_key_range_of_gen();
        self.prepare_max_key_range_of_gen();
        self.prepare_min_vel_range_of_gen();
        self.prepare_max_vel_range_of_gen();
    }

    pub fn get_sample(&self, start: usize, end: usize) -> Vec<i16> {
        let mut sample = Vec::new();
        sample.resize(end - start, 0);

        let gen_set = self.get_generator_from_key_vel(64, 64);
        for gen in gen_set.iter() {
            if let Some(sample_obj) = &gen.sample {
                let sample_ = sample_obj.get_sample(start, end);

                for i in 0..end - start {
                    sample[i] += sample_[i];
                }
            }
        }

        sample
    }

    fn prepare_min_key_range_of_gen(&mut self) {
        let mut min_key_range_of_gen = BTreeMap::new();
        for (gen_idx, gen) in self.generators.iter().enumerate() {
            if !min_key_range_of_gen.contains_key(&gen.generator.key_range.min) {
                min_key_range_of_gen.insert(gen.generator.key_range.min, HashSet::new());
            }
            min_key_range_of_gen
                .get_mut(&gen.generator.key_range.min)
                .unwrap()
                .insert(gen_idx);
        }

        self.min_key_range_of_gen = Some(Arc::new(min_key_range_of_gen));
    }

    fn prepare_max_key_range_of_gen(&mut self) {
        let mut max_key_range_of_gen = BTreeMap::new();
        for (gen_idx, gen) in self.generators.iter().enumerate() {
            if !max_key_range_of_gen.contains_key(&gen.generator.key_range.max) {
                max_key_range_of_gen.insert(gen.generator.key_range.max, HashSet::new());
            }
            max_key_range_of_gen
                .get_mut(&gen.generator.key_range.max)
                .unwrap()
                .insert(gen_idx);
        }

        self.max_key_range_of_gen = Some(Arc::new(max_key_range_of_gen));
    }

    fn prepare_min_vel_range_of_gen(&mut self) {
        let mut min_vel_range_of_gen = BTreeMap::new();
        for (gen_idx, gen) in self.generators.iter().enumerate() {
            if !min_vel_range_of_gen.contains_key(&gen.generator.vel_range.min) {
                min_vel_range_of_gen.insert(gen.generator.vel_range.min, HashSet::new());
            }
            min_vel_range_of_gen
                .get_mut(&gen.generator.vel_range.min)
                .unwrap()
                .insert(gen_idx);
        }

        self.min_vel_range_of_gen = Some(Arc::new(min_vel_range_of_gen));
    }

    fn prepare_max_vel_range_of_gen(&mut self) {
        let mut max_vel_range_of_gen = BTreeMap::new();
        for (gen_idx, gen) in self.generators.iter().enumerate() {
            if !max_vel_range_of_gen.contains_key(&gen.generator.vel_range.max) {
                max_vel_range_of_gen.insert(gen.generator.vel_range.max, HashSet::new());
            }
            max_vel_range_of_gen
                .get_mut(&gen.generator.vel_range.max)
                .unwrap()
                .insert(gen_idx);
        }

        self.max_vel_range_of_gen = Some(Arc::new(max_vel_range_of_gen));
    }

    pub fn get_generator_from_key_vel(&self, key: u8, vel: u8) -> Vec<Arc<InstrumentGenerator>> {
        let mut gen_idx_set_for_min_key = HashSet::new();
        for (_, value) in Arc::clone(&self.min_key_range_of_gen.as_ref().unwrap())
            .range((Included(&0), Included(&key)))
        {
            gen_idx_set_for_min_key = gen_idx_set_for_min_key.union(value).cloned().collect();
        }

        let mut gen_idx_set_for_max_key = HashSet::new();
        for (_, value) in Arc::clone(&self.max_key_range_of_gen.as_ref().unwrap())
            .range((Included(&key), Included(&127)))
        {
            gen_idx_set_for_max_key = gen_idx_set_for_max_key.union(value).cloned().collect();
        }

        let mut gen_idx_set_for_min_vel = HashSet::new();
        for (_, value) in Arc::clone(&self.min_vel_range_of_gen.as_ref().unwrap())
            .range((Included(&0), Included(&vel)))
        {
            gen_idx_set_for_min_vel = gen_idx_set_for_min_vel.union(value).cloned().collect();
        }

        let mut gen_idx_set_for_max_vel = HashSet::new();
        for (_, value) in Arc::clone(&self.max_vel_range_of_gen.as_ref().unwrap())
            .range((Included(&vel), Included(&127)))
        {
            gen_idx_set_for_max_vel = gen_idx_set_for_max_vel.union(value).cloned().collect();
        }

        let gen_idx_set = gen_idx_set_for_min_key
            .intersection(&gen_idx_set_for_max_key)
            .cloned()
            .collect::<HashSet<usize>>()
            .intersection(&gen_idx_set_for_min_vel)
            .cloned()
            .collect::<HashSet<usize>>()
            .intersection(&gen_idx_set_for_max_vel)
            .cloned()
            .collect::<HashSet<usize>>();

        let mut gen_set = Vec::new();
        for &gen_idx in gen_idx_set.iter() {
            gen_set.push(Arc::clone(self.generators.get(gen_idx).unwrap()));
        }
        gen_set
    }
}
