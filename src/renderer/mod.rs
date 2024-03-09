pub mod effects;
pub mod primitives;
pub mod shared;
pub mod utils;

use primitives::{
    common::*,
    pipeline::create_primitive_pipeline,
    shared::{FragmentsDataUniform, FragmentsStorage},
    utils::{attach_empty_fragment_storage, create_fragment_storage_bind_group},
};
use shared::*;
use utils::*;

use crate::{
    app::camera::{Camera, CameraController},
    renderer::primitives::shared::FragmentsData,
    scene::{
        shared::{ComponentBufferEntry, SceneStorage},
        utils::{chunk_id_from_position, ChunkId, ChunkRange},
        Scene,
    },
    timed,
    utils::{insert_ordered_at, wgpu::context::Context, WindowSize},
};

use egui::ahash::HashSet;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, Mutex},
};
use tracing::{debug, info};
use wgpu::{
    CommandEncoder, Device, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    SurfaceConfiguration, TextureView,
};

use self::primitives::utils::attach_fragment_data_uniform;

pub struct Shared<'a> {
    pub common_uniforms: CommonUniforms,
    pub scene_storage: SceneStorage,
    pub fragments_storage: FragmentsStorage,
    // one for each type of component
    pub fragments_data_uniform_map: HashMap<u32, FragmentsDataUniform>,
    phantom: PhantomData<&'a ()>,
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
    pub chunk_range: ChunkRange,
    // Maps a componet type to an array of indices in fragments storage buffer,
    // each index in the array corresponds to a Level of detail, being 0 the highest
    pub fragments_index_map: HashMap<u32, Vec<(u32, f32 /*This is the maximum camera distance*/)>>,
    pub last_lod_for_type: HashMap<u32, u32>,
}

pub struct Renderer<'a> {
    depth_texture: Texture,
    pub shared: Shared<'a>,
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

        let fragments_data_uniform = attach_fragment_data_uniform(&device);

        // let vertex_buffer = attach_vertex_buffer(&device, None);

        fragments_storage.fragments.set(vec![
            MEMRISTOR_PRIMITIVES_L0.to_fragments(),
            MEMRISTOR_PRIMITIVES_L1.to_fragments(),
            OMP_AMP_PRIMITIVES_L1.to_fragments(),
        ]);

        let primitive_pipeline = create_primitive_pipeline(
            &config,
            &device,
            &[
                &common_uniforms.bind_group_layout,
                &fragments_storage.bind_group_layout,
                &scene_storage.bind_group_layout,
                &fragments_data_uniform.bind_group_layout,
            ],
        );

        let grid_effect_pipeline = effects::grid::pipeline::create_pipeline(config, device);

        let pipelines = Pipelines {
            primitive: primitive_pipeline,
            grid_effect: grid_effect_pipeline,
        };

        let mut fragments_data_uniform_map = HashMap::new();
        fragments_data_uniform_map.insert(0, fragments_data_uniform);

        let shared = Shared {
            common_uniforms,
            scene_storage,
            fragments_storage,
            fragments_data_uniform_map,
            // vertex_buffer,
            phantom: PhantomData,
        };

        let mut cache = Cache::default();
        cache
            .fragments_index_map
            .insert(0, vec![(0, 400.0), (1, 1200.0)]);
        cache.fragments_index_map.insert(1, vec![(2, 400.0)]);

        cache.fragments_index_map.iter().for_each(|(ty, _)| {
            cache.last_lod_for_type.insert(*ty, 0);
        });

        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

        Self {
            depth_texture,
            shared,
            pipelines,
            cache: cache,
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

        // self.render_effects(&mut render_pass);

        Self::render_effects(
            &mut render_pass,
            &self.pipelines.grid_effect,
            &self.shared.common_uniforms.bind_group,
        );

        Self::render_primitives(
            &mut render_pass,
            &self.pipelines.primitive,
            &context,
            camera_controller,
            &mut self.cache,
            &self.shared.fragments_storage,
            &mut self.shared.fragments_data_uniform_map,
            &self.shared.scene_storage.bind_group,
            &self.shared.common_uniforms.bind_group,
        );
    }

    fn render_primitives<'b, 'c>(
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

        let fragments_type_vec = Self::check_and_update_fragments_data_uniforms(
            &context.device,
            &context.queue,
            cache,
            fragments_data_uniform_map,
            camera_controller,
        );

        // Draw each batch
        let mut n_rendered = 0;
        // for (ty, n_components) in cache.n_components_by_type.iter() {
        for (fragments_idx, ty) in fragments_type_vec.iter() {
            let fragments = &fragments_storage.fragments.get()[*fragments_idx as usize];
            let n_fragments = fragments.n_circles + fragments.n_lines + fragments.n_rectangles;

            let n_components = cache.n_components_by_type.get(ty).unwrap();

            // info!("Rendering {} components of type {}", n_components, ty);
            // render_pass.set_vertex_buffer(0, self.shared.vertex_buffer.buffer.slice(..));

            // if *ty == 0 {
            //     info!("Fragments idx for 0 is {}", fragments_idx);
            // }

            // info!("Rendering {} components of type {}", n_components, ty);

            render_pass.set_bind_group(
                3,
                &fragments_data_uniform_map.get(&ty).unwrap().bind_group,
                &[],
            );

            render_pass.draw(
                0..(n_fragments * 6),
                n_rendered..(*n_components as u32 + n_rendered),
            );
            n_rendered += *n_components as u32;
        }
    }

    // Returns <fragments_idx, ty>
    pub fn check_and_update_fragments_data_uniforms(
        device: &Device,
        queue: &Queue,
        cache: &Cache,
        fragments_data_uniform_map: &mut HashMap<u32, FragmentsDataUniform>,
        camera_controller: &CameraController,
    ) -> Vec<(u32, u32)> {
        let mut fragments_type_vec = Vec::new();
        for (ty, _n_components) in cache.n_components_by_type.iter() {
            let mut fragments_idx = match cache
                .fragments_index_map
                .get(ty)
                .unwrap()
                .binary_search_by_key(&(camera_controller.radius() as usize), |v| v.1 as usize)
            {
                Ok(i) => i,
                Err(i) => i,
            };

            if fragments_idx >= cache.fragments_index_map.get(ty).unwrap().len() {
                // fragment_idx = cache.fragments_index_map.get(ty).unwrap().len() - 1;
                continue;
            }

            fragments_idx = cache.fragments_index_map.get(ty).unwrap()[fragments_idx].0 as usize;

            if *ty == 0 {
                // info!("Fragments idx for 0 is {}", fragments_idx);
            }

            match fragments_data_uniform_map.get_mut(ty) {
                Some(fragments_data_uniform) => {
                    let prev_fragment_idx =
                        fragments_data_uniform.fragments_data.uniform.fragments_idx;

                    if prev_fragment_idx != fragments_idx as u32 {
                        // info!("Prev was {}, new is {}", prev_fragment_idx, fragments_idx);
                        let fragments_data = FragmentsData {
                            fragments_idx: fragments_idx as u32,
                        };
                        fragments_data_uniform.fragments_data.set(fragments_data);
                        // .write(&fragments_data)
                        // .unwrap();
                        queue.write_buffer(
                            &fragments_data_uniform.fragments_data.buffer,
                            0,
                            fragments_data_uniform.fragments_data.encase_buffer.as_ref(),
                        );
                    }
                }
                None => {
                    let mut fragments_data_uniform = attach_fragment_data_uniform(device);
                    let fragments_data = FragmentsData {
                        fragments_idx: fragments_idx as u32,
                    };

                    fragments_data_uniform.fragments_data.set(fragments_data);
                    queue.write_buffer(
                        &fragments_data_uniform.fragments_data.buffer,
                        0,
                        fragments_data_uniform.fragments_data.encase_buffer.as_ref(),
                    );

                    fragments_data_uniform_map.insert(*ty, fragments_data_uniform);
                }
            }

            fragments_type_vec.push((fragments_idx as u32, *ty));
        }

        fragments_type_vec
    }

    #[inline]
    fn render_effects<'b, 'c>(
        render_pass: &mut wgpu::RenderPass<'c>,
        pipeline: &'b wgpu::RenderPipeline,
        common_uniforms_bind_group: &'b wgpu::BindGroup,
    ) where
        'b: 'c,
    {
        render_pass.set_pipeline(pipeline);

        render_pass.set_bind_group(0, common_uniforms_bind_group, &[]);

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
        _camera_controller: &CameraController,
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

            let chunk_range = &mut self.cache.chunk_range;

            let actual_chunk_range = ChunkRange {
                min_chunk,
                max_chunk,
            };

            let (in_self_not_other, in_other_not_self) = timed! {
                chunk_range.diff(&actual_chunk_range),
                "Diffing"
            };

            let compute_start_positions = |n_components_by_type: &HashMap<u32, usize>| {
                let mut acc = 0;
                let start_positions = n_components_by_type
                    .iter()
                    .map(|(ty, n)| {
                        let start_p = acc;
                        acc += n;
                        (*ty, start_p)
                    })
                    .collect::<HashMap<u32, usize>>();
                start_positions
            };

            let (mut n_aditions, mut n_deletions) = (0, 0);

            let start_positions = compute_start_positions(n_components_by_type);
            let to_remove = Arc::new(Mutex::new(Vec::<(u32, u32)>::new()));
            timed!(
                {
                    // To delete
                    in_self_not_other.par_iter().for_each(|chunk_id| {
                        if components.len() == 0 {
                            return;
                        }

                        if let Some(chunk) = scene.components().get(&chunk_id) {
                            for component in chunk {
                                let start_p = start_positions.get(&component.ty()).unwrap_or(&0);
                                let n_components =
                                    n_components_by_type.get(&component.ty()).unwrap_or(&1);

                                // get the slice for the type
                                let c = &components[*start_p..(*start_p + n_components)];

                                // insert ordered by id
                                if let Ok(_idx) = c.binary_search_by(|a| a.id.cmp(&component.id()))
                                {
                                    to_remove
                                        .lock()
                                        .unwrap()
                                        .push((component.id(), component.ty()));
                                    // to_remove.push((component.id(), component.ty()));
                                }
                            }
                        }
                    });
                    let mut to_remove = to_remove.lock().unwrap().clone();

                    to_remove.iter().for_each(|(_id, ty)| {
                        n_components_by_type.get_mut(ty).map(|n| {
                            *n -= 1;
                            n_deletions += 1;
                        });
                    });

                    to_remove.par_sort();

                    components
                        .retain(|c| !to_remove.binary_search_by_key(&c.id, |(id, _)| *id).is_ok());
                },
                "Deleting components"
            );

            // info!("components after delete: {:#?}", components);

            // Check if components is sorted by type
            let mut prev_ty = 0;
            for comp in components.iter() {
                if comp.ty < prev_ty {
                    info!("components: {:#?}", components);
                    panic!("Not sorted by type after removing components");
                }
                prev_ty = comp.ty;
            }

            // Where to insert and the components to insert
            let to_insert = Arc::new(Mutex::new(Some(
                HashMap::<usize, Vec<ComponentBufferEntry>>::new(),
            )));
            timed!(
                {
                    in_other_not_self.par_iter().for_each(|chunk_id| {
                        if let Some(chunk) = scene.components().get(&chunk_id) {
                            for component in chunk {
                                // insert ordered by type chained with id
                                if let Err(i) = components.binary_search_by(|a| {
                                    a.ty()
                                        .cmp(&component.ty())
                                        .then(a.id().cmp(&component.id()))
                                }) {
                                    let mut to_insert = to_insert.lock().unwrap();

                                    // to_insert.entry(i).or_insert(Vec::new());
                                    // let entry = to_insert.get_mut(&i).unwrap();
                                    let entry =
                                        to_insert.as_mut().unwrap().entry(i).or_insert(Vec::new());

                                    // Insert in entry ordered by type chained with id
                                    if let Err(j) = entry.binary_search_by(|a| {
                                        a.ty()
                                            .cmp(&component.ty())
                                            .then(a.id().cmp(&component.id()))
                                    }) {
                                        entry.insert(
                                            j,
                                            ComponentBufferEntry::from_component(component),
                                        );
                                    }
                                }
                            }
                        }
                    });

                    let to_insert = to_insert.lock().unwrap().take().unwrap();

                    to_insert.iter().for_each(|(_idx, comps)| {
                        comps.iter().for_each(|c| {
                            n_components_by_type
                                .entry(c.ty)
                                .and_modify(|n| *n += 1)
                                .or_insert(1);
                            n_aditions += 1;
                        });
                    });

                    insert_ordered_at(&mut components, to_insert);
                },
                "Adding components"
            );

            timed!(
                if n_aditions + n_deletions > 0 {
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
                    if prev_size
                        != self
                            .shared
                            .scene_storage
                            .components
                            .get_scratch()
                            .as_ref()
                            .len()
                    {
                        //prev_size != self.shared.scene_storage.components.get_scratch().as_ref().len() {
                        self.shared.scene_storage.bind_group = create_scene_storage_bind_group(
                            device,
                            &self.shared.scene_storage.bind_group_layout,
                            self.shared.scene_storage.components.buffer().unwrap(),
                        );
                    }
                },
                "Writing components to buffer"
            );
        }
    }
}
