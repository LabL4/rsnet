use rsnet_net_parser::{get_nn_class, parse_nn};

use pyo3::prelude::*;
use tracing::{error, info};
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();

    let python_src = include_str!("./test.py");

    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let nn_class = get_nn_class(py, python_src);

        if let Err(e) = nn_class {
            info!("error: {:#?}", e);
            return;
        }
        let nn_class = nn_class.unwrap();

        let nn_instance = nn_class.call0();
        if let Err(e) = nn_instance {
            info!("error creating nn instance: {:#?}", e);
            return;
        }
        let nn_instance = nn_instance.unwrap();

        let nn_res = parse_nn(nn_instance);
        if let Err(e) = nn_res {
            error!("error parsing nn instance: {:#?}", e);
            return;
        }
        info!("parsed_nn: {:#?}", nn_res.unwrap());
    });
}
