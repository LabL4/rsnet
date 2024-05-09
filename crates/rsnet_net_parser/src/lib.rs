use pyo3::prelude::*;
use thiserror::Error;

mod types;

use types::*;

#[derive(Error, Debug)]
pub enum NnParseError {
    #[error("No suitable Nn class found (suitable classes need to inherit from torch.nn.Module)")]
    NoSuitableNnClassFound,
    #[error("The following python runtime errors were found when parsing the input {0}")]
    PythonError(String),
}

impl From<PyErr> for NnParseError {
    fn from(value: PyErr) -> Self {
        Self::PythonError(value.to_string())
    }
}

pub fn get_nn_class<'py>(py: Python<'py>, src: &str) -> Result<Bound<'py, PyAny>, NnParseError> {
    let src_module = PyModule::from_code_bound(py, src, "monmod", "monmod")?;

    // module.dir()
    let torch_module = py.import_bound("torch.nn")?;

    let module: Bound<'py, PyAny> = torch_module.getattr("Module")?;

    let mut nn_class = None;

    src_module.dir().iter().any(|attr_name| {
        let attr_str = attr_name.to_string();
        let item = src_module.getattr(attr_str.as_str()).unwrap();

        item.getattr("__bases__").is_ok_and(|attr| {
            if let Ok(class) = attr.call_method("__getitem__", (0,), None) {
                let class_str = class.to_string();
                if class_str == module.to_string() {
                    nn_class = Some(item);
                    return true;
                }
            }
            false
        })
    });

    if let Some(nn_class) = nn_class {
        Ok(nn_class)
    } else {
        Err(NnParseError::NoSuitableNnClassFound)
    }
}

pub fn parse_nn<'py>(nn_instance: Bound<'py, PyAny>) -> Result<Nn, NnParseError> {
    let mut nn = Nn { layers: vec![] };

    let layers = nn_instance.getattr("layers")?;

    for layer in layers.iter()? {
        let layer = layer?;
        let layer_class = layer.getattr("__class__")?;
        let layer_class_name = layer_class.getattr("__name__")?;

        match layer_class_name.to_string().as_str() {
            "Linear" => {
                let input_size = layer.getattr("in_features")?.extract::<usize>()?;
                let output_size = layer.getattr("out_features")?.extract::<usize>()?;
                let bias = layer.getattr("bias")?.extract::<Option<Bound<PyAny>>>()?;

                // info!("input_size: {:?}", input_size);
                // info!("output_size: {:?}", output_size);
                // info!("bias: {:?}", bias);

                nn.layers.push(Box::new(LinearLayer {
                    input_size,
                    output_size,
                    bias: bias.is_some(),
                }))
            }
            "ReLU" => {
                nn.layers.push(Box::new(Activation::ReLU));
            }
            _ => {}
        }
    }

    Ok(nn)
}

pub fn extract_nn(src: &str) -> Result<Nn, NnParseError> {
    Python::with_gil(|py| {
        let nn_class = get_nn_class(py, src)?;

        let nn_instance = nn_class.call0()?;

        parse_nn(nn_instance)
    })
}
