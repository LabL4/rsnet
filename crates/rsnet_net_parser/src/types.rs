use std::fmt::Debug;

#[derive(Debug)]
pub enum Activation {
    ReLU,
    // Sigmoid,
    // Tanh,
    // Softmax,
}

impl Layer for Activation {}

#[derive(Debug)]
pub struct LinearLayer {
    pub input_size: usize,
    pub output_size: usize,
    pub bias: bool,
}

impl Layer for LinearLayer {}

pub trait Layer: Debug {}

#[derive(Debug)]
pub struct Nn {
    pub layers: Vec<Box<dyn Layer>>,
}
