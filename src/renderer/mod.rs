pub mod effects;
pub mod primitives;
pub mod shared;
pub mod utils;

use shared::*;
use utils::*;
use primitives::{
    common::MEMRISTOR_PRIMITIVES,
    pipeline::{create_primitive_pipeline, attach_vertex_buffer, Vertex, VertexBuffer},
    shared::FragmentsStorage,
    utils::{attach_empty_fragment_storage, create_fragment_storage_bind_group}
};

use crate::{
    app::camera::{self, Camera, CameraController}, scene::{
        shared::{ComponentBufferEntry, SceneStorage},
        utils::{chunk_id_from_position, iter_chunks_in_range, ChunkId, ChunkRange},
        Scene,
    }, timed, utils::{wgpu::context::Context, WindowSize}
};

use wgpu::{
    util::DeviceExt, Buffer, CommandEncoder, Device, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, SurfaceConfiguration, TextureView,
};
use std::{borrow::BorrowMut, collections::HashMap, marker::PhantomData};
use egui::ahash::HashSet;
use tracing::{debug, info};
use rayon::prelude::*;


pub struct Shared {
    pub common_uniforms: CommonUniforms,
    pub scene_storage: SceneStorage,
    pub fragments_storage: FragmentsStorage,
    // pub vertex_buffer: VertexBuffer<'a>,
}

pub struct Pipelines {
    primitive: wgpu::RenderPipeline,
    grid_effect: wgpu::RenderPipeline,
}

#[derive(Debug, Default)]
pub struct Cache {
    /// This is a cache of the number of components in the scene, batched by type.
    pub n_components_by_type: HashMap<u32, usize>,
    pub visible_chunks: HashSet<ChunkId>,
    pub chunk_range: ChunkRange
}

pub struct Renderer<'a> {
    pub shared: Shared,
    pub pipelines: Pipelines,
    pub cache: Cache,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Renderer<'a> {
    pub fn new(config: &SurfaceConfiguration, device: &Device) -> Self {
        let common_uniforms = attach_common_uniforms(
            &device,
            CameraUniform::new(),
            MouseUniform::default(),
            WindowUniform::default(),
        );

        // This is the scene cache on GPU
        let scene_storage = attach_empty_scene_storage(&device);

        let mut fragments_storage = attach_empty_fragment_storage(&device);

        // let vertex_buffer = attach_vertex_buffer(&device, None);

        fragments_storage
            .fragments
            .set(vec![MEMRISTOR_PRIMITIVES.to_fragments()]);

        let primitive_pipeline = create_primitive_pipeline(
            &config,
            &device,
            &[
                &common_uniforms.bind_group_layout,
                &fragments_storage.bind_group_layout,
                &scene_storage.bind_group_layout,
            ],
        );

        let grid_effect_pipeline = effects::grid::pipeline::create_pipeline(config, device);

        let pipelines = Pipelines {
            primitive: primitive_pipeline,
            grid_effect: grid_effect_pipeline,
        };

        let shared = Shared {
            common_uniforms,
            scene_storage,
            fragments_storage,
            // vertex_buffer
        };

        Self {
            shared,
            pipelines,
            cache: Cache::default(),
            phantom: PhantomData,
        }
    }

    pub fn update_camera(&mut self, camera: &Camera, queue: &Queue) {
        let camera_uniform = &mut self.shared.common_uniforms.camera.uniform;
        camera_uniform.update_view_projection(camera);
        self.shared
            .common_uniforms
            .camera
            .encase_buffer
            .write(&camera_uniform)
            .unwrap();
        queue.write_buffer(
            &self.shared.common_uniforms.camera.buffer,
            0,
            self.shared.common_uniforms.camera.encase_buffer.as_ref(),
        );
    }

    pub fn update_window(&mut self, size: WindowSize, queue: &Queue) {
        let window_uniform = &mut self.shared.common_uniforms.window.uniform;

        window_uniform.size.x = size.width;
        window_uniform.size.y = size.height;
        window_uniform.compute_aspect();

        self.shared
            .common_uniforms
            .window
            .encase_buffer
            .write(&window_uniform)
            .unwrap();
        queue.write_buffer(
            &self.shared.common_uniforms.window.buffer,
            0,
            self.shared.common_uniforms.window.encase_buffer.as_ref(),
        );
    }

    pub fn update_mouse(&mut self, pos: (f32, f32), queue: &Queue) {
        let mouse_uniform = &mut self.shared.common_uniforms.mouse.uniform;

        mouse_uniform.pos.x = pos.0;
        mouse_uniform.pos.y = pos.1;

        self.shared
            .common_uniforms
            .mouse
            .encase_buffer
            .write(&mouse_uniform)
            .unwrap();
        queue.write_buffer(
            &self.shared.common_uniforms.mouse.buffer,
            0,
            self.shared.common_uniforms.mouse.encase_buffer.as_ref(),
        );
    }

    pub fn render(
        &mut self,
        view: &TextureView,
        context: &Context,
        encoder: &mut CommandEncoder,
        camera_controller: &mut CameraController,
        scene: &Scene,
    ) {
        self.check_and_update_common_uniforms(&context.queue, camera_controller);

        self.check_and_update_fragments_storage(&context.device, &context.queue, camera_controller);

        
        // timed!(self.check_and_update_scene_storage(
        //     &context.device,
        //     &context.queue,
        //     camera_controller,
        //     scene,
        // ), "Scene storage update");
        self.check_and_update_scene_storage(
            &context.device,
            &context.queue,
            camera_controller,
            scene,
        );
        

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.7,
                        g: 0.7,
                        b: 0.7,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        self.render_effects(&mut render_pass);

        self.render_primitives(&mut render_pass);
    }

    fn render_primitives<'b, 'c>(&'b self, render_pass: &mut wgpu::RenderPass<'c>)
    where
        'b: 'c,
    {
        render_pass.set_pipeline(&self.pipelines.primitive);

        render_pass.set_bind_group(0, &self.shared.common_uniforms.bind_group, &[]);
        render_pass.set_bind_group(1, &self.shared.fragments_storage.bind_group, &[]);
        render_pass.set_bind_group(2, &self.shared.scene_storage.bind_group, &[]);

        // Draw each batch
        let mut n_rendered = 0;
        for (ty, n_components) in self.cache.n_components_by_type.iter() {
            let fragments = &self.shared.fragments_storage.fragments.get()[*ty as usize];
            let n_fragments = fragments.n_circles + fragments.n_lines + fragments.n_rectangles;

            // info!("Rendering {} components of type {}", n_components, ty);

            // render_pass.set_vertex_buffer(0, self.cache.vertex_buffer_batches.get(ty).unwrap().slice(..));
            render_pass.draw(0..(n_fragments * 4), n_rendered..(*n_components as u32));
            n_rendered += *n_components as u32;
        }
    }

    fn render_effects<'b, 'c>(&'b self, render_pass: &mut wgpu::RenderPass<'c>)
    where
        'b: 'c,
    {
        render_pass.set_pipeline(&self.pipelines.grid_effect);

        render_pass.set_bind_group(0, &self.shared.common_uniforms.bind_group, &[]);

        render_pass.draw(0..4, 0..1);
    }

    /// This function checks if the common uniforms have changed, and updates the GPU buffer if they have.
    fn check_and_update_common_uniforms(
        &mut self,
        queue: &Queue,
        camera_controller: &mut CameraController,
    ) {
        if camera_controller.is_dirty {
            debug!("Updating camera");
            self.update_camera(&camera_controller.get_camera(), &queue);
            camera_controller.is_dirty = false;
        }
    }

    fn check_and_update_fragments_storage(
        &mut self,
        device: &Device,
        queue: &Queue,
        camera_controller: &CameraController,
    ) {
        if self.shared.fragments_storage.fragments.is_dirty() {
            // println!("n_circles: {}", self.shared.fragments_storage.fragments.get()[0].n_circles);
            let prev_size = self
                .shared
                .fragments_storage
                .fragments
                .get_scratch()
                .as_ref()
                .len();
            self.shared
                .fragments_storage
                .fragments
                .write_buffer(device, queue);

            if prev_size
                != self
                    .shared
                    .fragments_storage
                    .fragments
                    .get_scratch()
                    .as_ref()
                    .len()
            {
                self.shared.fragments_storage.bind_group = create_fragment_storage_bind_group(
                    device,
                    &self.shared.fragments_storage.bind_group_layout,
                    self.shared.fragments_storage.fragments.buffer().unwrap(),
                );
            }
        }
    }

    /// This function checks if the scene has changed, and updates the GPU buffer if it has.
    fn check_and_update_scene_storage(
        &mut self,
        device: &Device,
        queue: &Queue,
        camera_controller: &CameraController,
        scene: &Scene,
    ) {
        // let half_chunk_size = scene.chunk_size() / 2.0;

        // debug!("Camera AABB: {:?}", camera_controller.screen_world_aabb);
        // debug!("Half chunk size: {}", half_chunk_size);
        let min_chunk =
            chunk_id_from_position(&camera_controller.screen_world_aabb.min, scene.chunk_size());
        let max_chunk =
            chunk_id_from_position(&camera_controller.screen_world_aabb.max, scene.chunk_size());

        if self.shared.scene_storage.chunks != Some((min_chunk, max_chunk)) {
            self.shared.scene_storage.chunks = Some((min_chunk, max_chunk));

            // debug!("Visible chunks changed, updating components, ({}, {}), ({}, {})", min_chunk.0, min_chunk.1, max_chunk.0, max_chunk.1);

            let mut components = self.shared.scene_storage.components.get_mut();
            let n_components_by_type = &mut self.cache.n_components_by_type;

            let visible_chunks = &mut self.cache.visible_chunks;
            let chunk_range = &mut self.cache.chunk_range;

            let actual_chunk_range = ChunkRange {
                min_chunk,
                max_chunk
            };

            let (in_self_not_other, in_other_not_self) = chunk_range.diff(&actual_chunk_range);

            

            // info!("prev_chunk_range: {:#?}", chunk_range);
            // info!("actual_chunk_range: {:#?}", actual_chunk_range);

            // info!("in_self_not_other: {:#?}", in_self_not_other);
            // info!("in_other_not_self: {:#?}", in_other_not_self);


            // The start positions for types in component array
            // let mut acc = 0;
            // let start_positions = n_components_by_type.iter().map(|(ty, n)| {
            //     let start_p = acc;
            //     acc += n;
            //     (*ty, start_p)
            // }).collect::<HashMap<u32, usize>>();
            
            let compute_start_positions = |n_components_by_type: &HashMap<u32, usize>| {
                let mut acc = 0;
                let start_positions = n_components_by_type.iter().map(|(ty, n)| {
                let start_p = acc;
                acc += n;
                (*ty, start_p)
                }).collect::<HashMap<u32, usize>>();
                start_positions
            };

            timed!(
            // To delete
            for chunk_id in in_self_not_other {
                if components.len() == 0 {
                    break;
                }
                // info!("Not seeing chunk {:#?}", chunk_id);
                if let Some(chunk) = scene.components().get(&chunk_id) {
                    for component in chunk {


                        let start_positions = compute_start_positions(n_components_by_type);

                        let start_p = start_positions.get(&component.ty()).unwrap_or(&0);
                        let n_components = n_components_by_type.get(&component.ty()).unwrap_or(&1);
                        // get the slice for the type
                        let c = &mut components[*start_p..(*start_p + n_components)];
                        // insert ordered by id
                        if let Ok(idx) = c.binary_search_by(|a| a.id.cmp(&component.id())) {
                            components.remove(idx + *start_p);
                            // info!("Removed component with id: {}, n_components: {}", component.id(), components.len());
                            match n_components_by_type.get_mut(&component.ty()) {
                                Some(n) => {
                                    // Delete this component
                                    *n -= 1;                                
                                }
                                None => {
                                }
                            }
                        }


                    }
                }
            },
            "Deleting components"
            );

            timed!(
            // in_other_not_self.par_iter().for_each(|chunk_id| {

            for chunk_id in in_other_not_self {
                if let Some(chunk) = scene.components().get(&chunk_id) {
                    for component in chunk {
                        if components.len() == 0 {
                            components.push(ComponentBufferEntry::from_component(component));
                            n_components_by_type.insert(component.ty(), 1);
                            continue;
                        }

                        let start_positions = compute_start_positions(&n_components_by_type);

                        let start_p = start_positions.get(&component.ty()).unwrap_or(&0);
                        // info!("start_p: {}", start_p);
                        let n_comps = n_components_by_type.get(&component.ty()).unwrap_or(&1);
                        
                        // let prev_comp_ids = components.iter().map(|c| c.id).collect::<Vec<u32>>();
                        // get the slice for the type
                        let components_slice = &mut components[*start_p..(*start_p + n_comps)];
                        // let comp_slice_prev_comp_ids = components_slice.iter().map(|c| c.id).collect::<Vec<u32>>();
                        // insert ordered by id
                        if let Err(mut i) = components_slice.binary_search_by_key(&component.id(), |c| c.id()) {
                            // info!("prev components ids: {:#?}", prev_comp_ids);
                            // info!("prev components_slice ids {}: {:#?}", n_comps, comp_slice_prev_comp_ids);
                            
                            if i == components_slice.len() {
                                components.push(ComponentBufferEntry::from_component(component));
                                i +=1 ;
                                // info!("Pushed component {} at {}", component.id(), i);
                            } else {
                                i += *start_p;
                                components.insert(i, ComponentBufferEntry::from_component(component));
                                // info!("Inserted component {} at {}", component.id(), i);
                            }
    
                            // info!("Added component with id: {}, n_components: {}", component.id(), components.len());
                            // info!("components ids: {:#?}", components.iter().map(|c| c.id).collect::<Vec<u32>>());
                            match n_components_by_type.get_mut(&component.ty()) {
                                Some(n) => {
                                    *n += 1;
                                }    
                                None => {
                                    n_components_by_type.insert(component.ty(), 1);
                                }    
                            }    
                        }

                        // components.insert(
                        //     n_components_by_type.get(&component.ty()).unwrap() - 1,
                        //     ComponentBufferEntry::from_component(component),
                        // );
                    }
                }
            },
            "Adding components"
            );

            // info!("components ids:\n{:#?}", components.iter().map(|c| c.id).collect::<Vec<u32>>());


            self.cache.chunk_range = actual_chunk_range;


            // self.shared.scene_storage.components.set(components.clone());
            let prev_size = self
                .shared
                .scene_storage
                .components
                .get_scratch()
                .as_ref()
                .len();
            self.shared
                .scene_storage
                .components
                .write_buffer(device, queue);
            if prev_size != self.shared.scene_storage.components.get_scratch().as_ref().len() {
                //prev_size != self.shared.scene_storage.components.get_scratch().as_ref().len() {
                self.shared.scene_storage.bind_group = create_scene_storage_bind_group(
                    device,
                    &self.shared.scene_storage.bind_group_layout,
                    self.shared.scene_storage.components.buffer().unwrap(),
                );
            }
        }

        // if self.shared.scene_storage.components.is_dirty() {
        //     self.shared.scene_storage.components.update(&device);
        // }
    }
}

// fn check_and_update_scene_storage(
//     &mut self,
//     device: &Device,
//     queue: &Queue,
//     camera_controller: &CameraController,
//     scene: &Scene,
// ) {
//     let half_chunk_size = scene.chunk_size() / 2.0;

//     // debug!("Camera AABB: {:?}", camera_controller.screen_world_aabb);
//     // debug!("Half chunk size: {}", half_chunk_size);
//     let min_chunk =
//         chunk_id_from_position(&camera_controller.screen_world_aabb.min, scene.chunk_size());
//     let max_chunk =
//         chunk_id_from_position(&camera_controller.screen_world_aabb.max, scene.chunk_size());

//     if self.shared.scene_storage.chunks != Some((min_chunk, max_chunk)) {
//         self.shared.scene_storage.chunks = Some((min_chunk, max_chunk));

//         // debug!("Visible chunks changed, updating components, ({}, {}), ({}, {})", min_chunk.0, min_chunk.1, max_chunk.0, max_chunk.1);

//         let mut components: Vec<ComponentBufferEntry> = Vec::new();
//         let mut n_components_by_type: HashMap<u32, usize> = HashMap::new();

//         let visible_chunks = &mut self.cache.visible_chunks;

//         for chunk_id in iter_chunks_in_range(&min_chunk, &max_chunk) {
//             if !visible_chunks.contains(&chunk_id) {
//                 if let Some(chunk) = scene.components().get(&chunk_id) {
//                     for component in chunk {
//                         match n_components_by_type.get_mut(&component.ty()) {
//                             Some(n) => {
//                                 *n += 1;
//                             }
//                             None => {
//                                 n_components_by_type.insert(component.ty(), 1);
//                             }
//                         }
//                         components.insert(
//                             n_components_by_type.get(&component.ty()).unwrap() - 1,
//                             ComponentBufferEntry::from_component(component),
//                         );
//                     }
//                 }
//                 visible_chunks.insert(chunk_id);
//             }
//         }

//         self.cache.n_components_by_type = n_components_by_type;

//         self.shared.scene_storage.components.set(components);
//         let prev_size = self
//             .shared
//             .scene_storage
//             .components
//             .get_scratch()
//             .as_ref()
//             .len();
//         self.shared
//             .scene_storage
//             .components
//             .write_buffer(device, queue);
//         if true {
//             //prev_size != self.shared.scene_storage.components.get_scratch().as_ref().len() {
//             self.shared.scene_storage.bind_group = create_scene_storage_bind_group(
//                 device,
//                 &self.shared.scene_storage.bind_group_layout,
//                 self.shared.scene_storage.components.buffer().unwrap(),
//             );
//         }
//     }

// timed!(
//     // in_other_not_self.par_iter().for_each(|chunk_id| {
// for chunk_id in in_other_not_self {
//     if let Some(chunk) = scene.components().get(&chunk_id) {
//         for component in chunk {
//             if components.len() == 0 {
//                 components.push(ComponentBufferEntry::from_component(component));
//                 n_components_by_type.insert(component.ty(), 1);
//                 continue;
//             }

//             let start_positions = compute_start_positions(&n_components_by_type);

//             let start_p = start_positions.get(&component.ty()).unwrap_or(&0);
//             // info!("start_p: {}", start_p);
//             let n_comps = n_components_by_type.get(&component.ty()).unwrap_or(&1);
            
//             // let prev_comp_ids = components.iter().map(|c| c.id).collect::<Vec<u32>>();
//             // get the slice for the type
//             let components_slice = &mut components[*start_p..(*start_p + n_comps)];
//             // let comp_slice_prev_comp_ids = components_slice.iter().map(|c| c.id).collect::<Vec<u32>>();
//             // insert ordered by id
//             if let Err(mut i) = components_slice.binary_search_by_key(&component.id(), |c| c.id()) {
//                 // info!("prev components ids: {:#?}", prev_comp_ids);
//                 // info!("prev components_slice ids {}: {:#?}", n_comps, comp_slice_prev_comp_ids);
                
//                 if i == components_slice.len() {
//                     components.push(ComponentBufferEntry::from_component(component));
//                     i +=1 ;
//                     // info!("Pushed component {} at {}", component.id(), i);
//                 } else {
//                     i += *start_p;
//                     components.insert(i, ComponentBufferEntry::from_component(component));
//                     // info!("Inserted component {} at {}", component.id(), i);
//                 }

//                 // info!("Added component with id: {}, n_components: {}", component.id(), components.len());
//                 // info!("components ids: {:#?}", components.iter().map(|c| c.id).collect::<Vec<u32>>());
//                 match n_components_by_type.get_mut(&component.ty()) {
//                     Some(n) => {
//                         *n += 1;
//                     }    
//                     None => {
//                         n_components_by_type.insert(component.ty(), 1);
//                     }    
//                 }    
//             }

//             // components.insert(
//             //     n_components_by_type.get(&component.ty()).unwrap() - 1,
//             //     ComponentBufferEntry::from_component(component),
//             // );
//         }
//     }
// },
// "Adding components"
// );