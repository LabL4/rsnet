use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Activation {
    ReLU,
    // Sigmoid,
    // Tanh,
    // Softmax,
}

#[derive(Debug, Clone)]
pub struct LinearLayer {
    pub input_size: usize,
    pub output_size: usize,
    pub bias: bool,
}

#[derive(Debug, Clone)]
pub enum Layer {
    Linear(LinearLayer),
    Activation(Activation),
}

#[derive(Debug, Clone)]
pub struct Nn {
    pub layers: Vec<Layer>,
}
