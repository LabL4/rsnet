use std::collections::HashMap;
use std::num::TryFromIntError;

use super::component;
use super::scene;

use component::{Component, DefaultComponentTypes};
use egui::emath::Numeric;
use egui::output;
use nalgebra::Vector;
use scene::{Scene, SceneError};
use tracing_subscriber::layer;

use crate::types::Id;
use crate::types::NodeId;

use rsnet_net_parser::types::{Layer, LinearLayer, Nn};

use nalgebra::Vector2;
use petgraph::graph::UnGraph;
use thiserror::Error;

// Generate a new scene from a neural network
pub fn gen_from_nn(nn: Nn) -> Scene {
    let mut scene = Scene::new_empty();

    let mut offset = Vector2::new(0.0, 0.0);
    let spacing = 2.0;
    let mut last_id = 0u32;
    // Add crossbar
    for layer in nn.layers {
        match layer {
            Layer::Activation(activation_ty) => {
                // Add activation
            }
            Layer::Linear(linear_layer) => {
                last_id =
                    add_linear(&mut scene, &linear_layer, offset, spacing, last_id).unwrap() + 1;
                offset += Vector2::new(
                    linear_layer.output_size as f32 * spacing + spacing * 10.0,
                    0.0,
                );
            }
        }
    }

    scene
}

fn add_linear(
    scene: &mut Scene,
    layer: &LinearLayer,
    offset: Vector2<f32>,
    spacing: f32,
    start_id: u32,
) -> Result<Id, SceneError> {
    let n_rows = layer.input_size;
    let n_cols = layer.output_size;

    let total_spacing_y = n_rows as f32 * spacing;

    for row in 0..n_rows {
        for col in 0..n_cols {
            let id: u32 = ((row + col * n_rows) as u32) + start_id;

            // if col == 0 {
            //     scene.add_wire(
            //         0,
            //         Wire::new(
            //             id as u32,
            //             Vector2::new(
            //                 col as f32 * spacing,
            //                 total_spacing_y - row as f32 * spacing) + offset,
            //             Vector2::new(
            //                 col as f32 * spacing,
            //                 total_spacing_y - row as f32 * spacing) + offset,
            //             Vector2::new(1.0, 0.0),
            //             Vector2::new(1.0, 1.0),
            //             0u32,
            //         ),
            //     );
            // }

            scene.add_component(
                0,
                Component::new(
                    id as u32,
                    0u32,
                    Vector2::new(col as f32 * spacing, total_spacing_y - row as f32 * spacing)
                        + offset,
                    // 45.0f32.to_radians(),
                    45.0f32.to_radians(),
                    0u32, // id as u32 % 5u32,
                ),
            );
        }
    }

    Ok(((n_rows - 1) + (n_cols - 1) * n_rows) as u32 + start_id + 100)
}

pub struct Crossbar {
    /// The index of the layer that the crossbar belongs to (a layer in a MLP consists only of one crossbar).
    layer_idx: usize,

    /// ```components_id_range``` should be used to store the component ids, it should be consistent with the components
    /// meaning that no the total component count should be components_id_range.1 - components_id_range.0 + 1.
    // components: Vec<Id>,

    /// The number of (input) rows of the crossbar.
    rows: u32,
    /// The number of (output) columns of the crossbar.
    cols: u32,

    spacing: f32,
    center: Vector2<f32>,    

    /// The input nodes of the crossbar, should be provided on creation, when calling the add_to_scene method,
    /// new nodes (inner to the crossbar) will be created (if needed) and connected to the input and output nodes.
    input_nodes: Vec<String>,
    /// The output nodes of the crossbar, should be provided on creation. Refer to input_nodes for more information.
    output_nodes: Vec<String>,

    /// The range of components Ids (inside the) that belong to the crossbar.
    components_id_range: (Id, Id),
}

impl Crossbar {
    pub fn new(
        layer_idx: usize,
        input_nodes: Vec<String>,
        output_nodes: Vec<String>,
        rows: u32,
        cols: u32,
        start_component_id: Id,
        spacing: f32,
        center: Vector2<f32>,
    ) -> Crossbar {
        Crossbar {
            layer_idx,
            input_nodes,
            output_nodes,
            rows: rows,
            cols: cols,
            spacing,
            center,
            components_id_range: (
                start_component_id,
                (rows - 1 + (cols - 1) * rows) + start_component_id as Id,
            ),
        }
    }
}

impl Construct for Crossbar {
    fn components_id_range(&self) -> (Id, Id) {
        self.components_id_range
    }

    fn add_to_scene(&self, scene_manager: &mut SceneManager) -> Result<(), SceneManagerError> {

        let scene = scene_manager.scene_mut();

        let n_rows = self.rows;
        let n_cols = self.cols;

        let total_spacing_y = n_rows as f32 * self.spacing;

        for row in 0..n_rows {
            for col in 0..n_cols {
                let id: u32 = row + col * n_rows + self.components_id_range.0;

                let col_f32: Option<f32> = num::cast(col);
                let row_f32: Option<f32> = num::cast(row);

                if col_f32.is_none() {
                    return Err(
                        SceneManagerError::FloatIntConversionError(format!("u32 {} -> f32 at Crossbar", col)),
                    );
                }

                if row_f32.is_none() {
                    return Err(
                        SceneManagerError::FloatIntConversionError(format!("u32 {} -> f32 at Crossbar", row)),
                    );
                }

                let col_f32 = col_f32.unwrap();
                let row_f32 = row_f32.unwrap();

                // if col == 0 {
                //     scene.add_wire(
                //         0,
                //         Wire::new(
                //             id as u32,
                //             Vector2::new(
                //                 col as f32 * spacing,
                //                 total_spacing_y - row as f32 * spacing) + offset,
                //             Vector2::new(
                //                 col as f32 * spacing,
                //                 total_spacing_y - row as f32 * spacing) + offset,
                //             Vector2::new(1.0, 0.0),
                //             Vector2::new(1.0, 1.0),
                //             0u32,
                //         ),
                //     );
                // }



                scene.add_component(
                    0,
                    Component::new(
                        id,
                        0u32,
                        Vector2::new(
                            col_f32 * self.spacing,
                            total_spacing_y - row_f32 * self.spacing,
                        ) + self.center,
                        45.0f32.to_radians(),
                        DefaultComponentTypes::Memristor.into(),
                    ),
                );
            }
        }

        Ok(())
    }
}

pub trait Construct {
    /// Return the range of components ids (in the scene) that are part of the constructs
    fn components_id_range(&self) -> (Id, Id);
    /// Adds the construct to the scene using components
    fn add_to_scene(&self, scene_manager: &mut SceneManager) -> Result<(), SceneManagerError>;
}

#[derive(Error, Debug)]
pub enum SceneManagerError {
    #[error("There was an error during scene manipulation: {0:#?}")]
    SceneError(SceneError),
    #[error("There was an error while converting between int types (likely from usize to u32) {0}")]
    IntConversionError(TryFromIntError),
    #[error("Error while converting between float and int types: {0}")]
    FloatIntConversionError(String),
}

impl From<SceneError> for SceneManagerError {
    fn from(value: SceneError) -> Self {
        Self::SceneError(value)
    }
}

impl From<TryFromIntError> for SceneManagerError {
    fn from(value: TryFromIntError) -> Self {
        Self::IntConversionError(value)
    }
}

pub struct SceneManager {
    scene: Scene,

    /// Just store the connections, the components are stored in the scene. Nodes are ```usize```, edges do not have any data.
    netlist: UnGraph<usize, ()>,

    /// Map from node name to id
    node_name_map: HashMap<String, usize>,

    last_component_id: Option<Id>,
    last_node_id: Option<NodeId>,

    /// A construct is a group of components such that they define a higher level circuital entity.
    /// For example, a crossbar is a construct, an activation function is a construct, etc.
    /// This is useful for grouping components. For example, if we want to move a crossbar, we can
    /// move all the components that are part of the crossbar.
    constructs: Vec<Box<dyn Construct>>,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            scene: Scene::new_empty(),
            netlist: UnGraph::new_undirected(),
            node_name_map: HashMap::new(),
            last_component_id: None,
            last_node_id: None,
            constructs: Vec::new(),
        }
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn add_node(&mut self, name: String) {
        
    }

    pub fn load_nn(&mut self, nn: Nn) -> Result<(), SceneManagerError> {
        let mut scene = Scene::new_empty();

        let mut offset = Vector2::new(0.0, 0.0);
        let crossbar_spacing = 2.0;

        // let mut last_nodes = None;

        // Add crossbar
        for (layer_idx, layer) in nn.layers.iter().enumerate() {
            match layer {
                Layer::Activation(activation_ty) => {
                    // Add activation
                }
                Layer::Linear(linear_layer) => {

                    // layer_idx: usize,
                    // input_nodes: Vec<String>,
                    // output_nodes: Vec<String>,
                    // rows: u32,
                    // cols: u32,
                    // start_component_id: Id,
                    // spacing: f32,
                    // center: Vector2<f32>,

                    let crossbar = Crossbar::new(
                        layer_idx,
                        vec!["".to_string(); linear_layer.input_size as usize],
                        vec!["".to_string(); linear_layer.output_size as usize],
                        linear_layer.input_size.try_into()?,
                        linear_layer.output_size as u32,
                        self.last_component_id.map_or(0, |id| id + 1),
                        crossbar_spacing,
                        offset,
                    );
                    crossbar.add_to_scene(self)?;
                    self.last_component_id = Some(crossbar.components_id_range().1);
                }
            }
        }

        Ok(())
    }
}
