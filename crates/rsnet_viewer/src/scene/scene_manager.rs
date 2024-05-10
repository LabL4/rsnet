use super::component;
use super::component::DefaultComponentTypes;
use super::scene;

use crate::types::Id;

use component::Component;
use nalgebra::Vector2;
use scene::{Scene, SceneError};

use rsnet_net_parser::types::{Layer, LinearLayer, Nn};

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
                last_id = add_linear(&mut scene, &linear_layer, offset, spacing, last_id).unwrap() + 1;
                println!("Adding linear layer");
                offset += Vector2::new(
                    linear_layer.output_size as f32 * spacing + spacing * 10.0,
                    0.0,
                );
            }
        }
    }

    scene
}

fn add_linear(scene: &mut Scene, layer: &LinearLayer, offset: Vector2<f32>, spacing: f32, start_id: u32) -> Result<Id, SceneError> {
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
                    Vector2::new(
                        col as f32 * spacing,
                        total_spacing_y - row as f32 * spacing) + offset,
                    // 45.0f32.to_radians(),
                    45.0f32.to_radians(),
                    0u32
                    // id as u32 % 5u32,
                ),
            );
        }
    }

    Ok(((n_rows-1) + (n_cols-1) * n_rows) as u32 +  start_id)
}
