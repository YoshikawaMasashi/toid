pub struct ConvolutionEffect {
    filter: Vec<f64>,
}

impl ConvolutionEffect {
    pub fn new(filter: Vec<f64>) -> Self {
        Self { filter }
    }
}
