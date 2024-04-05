use super::{
    utils::attach_fragment_data_uniform,
    shared::{FragmentsData, FragmentsDataUniform, FragmentsStorage}
};

use crate::renderer::Cache;
use crate::app::camera::CameraController;
use crate::utils::wgpu::Context;

use std::collections::{HashMap, HashSet};
use tracing::info;
use wgpu::{Device, Queue};

pub fn render<'b, 'c>(
    render_pass: &mut wgpu::RenderPass<'c>,
    pipeline: &'b wgpu::RenderPipeline,
    context: &Context,
    camera_controller: &CameraController,
    cache: &mut Cache,
    fragments_storage: &'b FragmentsStorage,
    fragments_data_uniform_map: &'b mut HashMap<u32, FragmentsDataUniform>,
    // fragments_storage_bind_group: &'b wgpu::BindGroup,
    scene_storage_bind_group: &'b wgpu::BindGroup,
    common_uniforms_bind_group: &'b wgpu::BindGroup,
) where
    'b: 'c,
{
    render_pass.set_pipeline(pipeline);

    render_pass.set_bind_group(0, &common_uniforms_bind_group, &[]);
    render_pass.set_bind_group(1, &fragments_storage.bind_group, &[]);
    render_pass.set_bind_group(2, &scene_storage_bind_group, &[]);

    let fragments_type_vec = check_and_update_fragments_data_uniforms(
        &context.device,
        &context.queue,
        cache,
        fragments_data_uniform_map,
        camera_controller,
    );

    // Draw each batch
    let mut n_rendered = 0;
    for (fragments_idx, ty) in fragments_type_vec.iter() {
        let component_ty_fragments = &fragments_storage.component_ty_fragments.get()[*fragments_idx as usize];
        let n_fragments = component_ty_fragments.n_fragments();

        let n_components = cache.n_components_by_type.get(ty).unwrap();

        render_pass.set_bind_group(
            3,
            &fragments_data_uniform_map.get(&ty).unwrap().bind_group,
            &[],
        );

        // info!("Rendering {} fragments for type {} for {} components", n_fragments, ty, n_components);

        render_pass.draw(
            0..(n_fragments * 6),
            n_rendered..(*n_components as u32 + n_rendered),
        );
        n_rendered += *n_components as u32;
    }
}

// Returns <fragments_idx, ty>
fn check_and_update_fragments_data_uniforms(
    device: &Device,
    queue: &Queue,
    cache: &Cache,
    fragments_data_uniform_map: &mut HashMap<u32, FragmentsDataUniform>,
    camera_controller: &CameraController,
) -> Vec<(u32, u32)> {
    let mut fragments_type_vec = vec![(0u32, 0u32); cache.n_components_by_type.len()];
    let mut positions = HashSet::<u32>::new();
    
    for (idx, (ty, _n_components)) in cache.n_components_by_type.iter().enumerate() {
        let mut fragments_idx = match cache
            .fragments_comptype_index_map
            .get(ty)
            .unwrap()
            .binary_search_by_key(&(camera_controller.radius() as usize), |v| v.1 as usize)
        {
            Ok(i) => i,
            Err(i) => i,
        };

        if fragments_idx >= cache.fragments_comptype_index_map.get(ty).unwrap().len() {
            // fragment_idx = cache.fragments_index_map.get(ty).unwrap().len() - 1;
            continue;
        }

        
        fragments_idx = cache.fragments_comptype_index_map.get(ty).unwrap()[fragments_idx].0 as usize;
        
        println!("fragments_idx: {}", fragments_idx);

        if *ty == 0 {
            // info!("Fragments idx for 0 is {}", fragments_idx);
        }

        match fragments_data_uniform_map.get_mut(ty) {
            Some(fragments_data_uniform) => {
                let prev_fragment_idx =
                    fragments_data_uniform.buffer.uniform.fragments_idx;

                if prev_fragment_idx != fragments_idx as u32 {
                    // info!("Prev was {}, new is {}", prev_fragment_idx, fragments_idx);
                    let fragments_data = FragmentsData {
                        fragments_idx: fragments_idx as u32,
                    };
                    fragments_data_uniform.buffer.set(fragments_data);
                    // .write(&fragments_data)
                    // .unwrap();
                    queue.write_buffer(
                        &fragments_data_uniform.buffer.buffer,
                        0,
                        fragments_data_uniform.buffer.encase_buffer.as_ref(),
                    );
                }
            }
            None => {
                let mut fragments_data_uniform = attach_fragment_data_uniform(device);
                let fragments_data = FragmentsData {
                    fragments_idx: fragments_idx as u32,
                };

                fragments_data_uniform.buffer.set(fragments_data);
                queue.write_buffer(
                    &fragments_data_uniform.buffer.buffer,
                    0,
                    fragments_data_uniform.buffer.encase_buffer.as_ref(),
                );

                fragments_data_uniform_map.insert(*ty, fragments_data_uniform);
            }
        }

        fragments_type_vec[idx] = (fragments_idx as u32, *ty);
        positions.insert(*ty as u32);

    }

    // info!("Positions: {:?}", positions);

    fragments_type_vec.retain(|(_fragments_idx, ty)| {
        positions.contains(ty)
    });

    // sort by ty
    fragments_type_vec.sort_by(|a, b| a.1.cmp(&b.1));

    fragments_type_vec
}