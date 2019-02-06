// Source adopted from
// https://github.com/tildeio/helix-website/blob/master/crates/word_count/src/lib.rs

extern crate portaudio_sys;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use portaudio_sys as portaudio;

/// Represents a file that can be searched
#[pyclass]
struct WordCounter {
    path: PathBuf,
}

#[pymethods]
impl WordCounter {
    #[new]
    fn __new__(obj: &PyRawObject, path: String) -> PyResult<()> {
        obj.init(|| WordCounter {
            path: PathBuf::from(path),
        })
    }

    /// Searches for the word, parallelized by rayon
    fn search(&self, py: Python<'_>, search: String) -> PyResult<usize> {
        let contents = fs::read_to_string(&self.path)?;

        let count = py.allow_threads(move || {
            contents
                .par_lines()
                .map(|line| count_line(line, &search))
                .sum()
        });
        Ok(count)
    }

    /// Searches for a word in a classic sequential fashion
    fn search_sequential(&self, needle: String) -> PyResult<usize> {
        let contents = fs::read_to_string(&self.path)?;

        let result = contents.lines().map(|line| count_line(line, &needle)).sum();

        Ok(result)
    }
}

fn matches(word: &str, needle: &str) -> bool {
    let mut needle = needle.chars();
    for ch in word.chars().skip_while(|ch| !ch.is_alphabetic()) {
        match needle.next() {
            None => {
                return !ch.is_alphabetic();
            }
            Some(expect) => {
                if ch.to_lowercase().next() != Some(expect) {
                    return false;
                }
            }
        }
    }
    return needle.next().is_none();
}

/// Count the occurences of needle in line, case insensitive
#[pyfunction]
fn count_line(line: &str, needle: &str) -> usize {
    let mut total = 0;
    for word in line.split(' ') {
        if matches(word, needle) {
            total += 1;
        }
    }
    total
}

struct PaTestData {
    sine: [f64; 200],
    left_phase: i32,
    right_phase: i32,
    message: [char; 20],
}

#[pyfunction]
fn portaudio_test() -> i32 {
    let mut outputParameters: portaudio::PaStreamParameters;
    let mut stream: *mut portaudio::PaStream;
    let mut err: portaudio::PaError;
    let mut data: PaTestData;

    let NUM_SECONDS: i32 = 5;
    let SAMPLE_RATE: i32 = 44100;
    let FRAMES_PER_BUFFER: i32 = 64;
    let M_PI: f64 = 3.14159265;
    let TABLE_SIZE: i32 = 200;

    println!("PortAudio Test: output sine wave. SR = {}, BufSize = {}\n",
      SAMPLE_RATE, FRAMES_PER_BUFFER);
    
    1
}

#[pymodule]
fn rustoid(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(count_line))?;
    m.add_wrapped(wrap_pyfunction!(portaudio_test))?;
    m.add_class::<WordCounter>()?;

    Ok(())
}
