pub mod effects;
pub mod primitives;
pub mod shader;
pub mod shared;
pub mod utils;
pub mod wires;

use md5::digest::consts::True;
use primitives::{
    common::*,
    pipeline::create_primitive_pipeline,
    shared::{FragmentsDataUniform, FragmentsStorage},
};
use shared::*;
use utils::*;

use crate::{
    app::camera::{Camera, CameraController},
    scene::{
        self,
        shared::{
            create_scene_storage_bind_group, ComponentBufferEntry, SceneStorage, WireBufferEntry,
        },
        utils::{chunk_id_from_position, ChunkId, ChunkRange, ChunkSize},
        Scene,
    },
    timed,
    utils::{insert_ordered_at, wgpu::context::Context, Id, WindowSize},
};

use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    primitive,
    sync::{Arc, Mutex},
};
use tracing::{debug, info, warn};
use wgpu::{
    CommandEncoder, Device, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    SurfaceConfiguration, TextureView,
};

pub struct Shared<'a> {
    pub chunk_data_uniform: ChunkDataUniform,
    pub time_uniform: TimeUniform,
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
    wires: wgpu::RenderPipeline,
}

#[derive(Debug, Default)]
pub struct Cache {
    /// This is a cache of the number of components in the scene, batched by type.
    pub n_components_by_type: HashMap<u32, usize>,
    pub chunk_range: Option<ChunkRange>,
    /// Maps a componet type to an array of indices in fragments storage buffer,
    /// each index in the array corresponds to a Level of detail, 0 being the highest
    pub compty_fragments_index_map:
        HashMap<u32, Vec<(u32, f32 /*This is the maximum camera distance*/)>>,
    pub scene_chunk_step_idx: u32,
}

pub struct Renderer<'a> {
    depth_texture: Texture,
    pub shared: Shared<'a>,
    pub pipelines: Pipelines,
    pub cache: Cache,
    pub msaa_count: u32,
    time: u32,
    last_rendered: std::time::Instant,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Renderer<'a> {
    pub fn new(config: &SurfaceConfiguration, device: &Device) -> Self {
        let chunk_data_uniform = ChunkDataUniform::attach(
            device,
            ChunkData {
                prev_chunk_size: 1.0,
                chunk_size: 1.0,
                last_chunk_size_update: 0,
            },
        );

        let time_uniform = TimeUniform::attach(device, 0);

        let common_uniforms = CommonUniforms::attach(device);

        let msaa_count = 1;

        // This is the scene cache on GPU
        let scene_storage = SceneStorage::attach_empty(&device);

        let fragments_storage = FragmentsStorage::attach_from_primitives(
            &device,
            vec![
                // &MEMRISTOR_PRIMITIVES_L0,
                // &MEMRISTOR_PRIMITIVES_L1,
                // &OMP_AMP_PRIMITIVES_L0,
                // &NMOS_PRIMITIVES_L0,
            ],
        );

        // info!("fragments_storage: {:#?}", fragments_storage.component_ty_fragments.get());

        let fragments_data_uniform = FragmentsDataUniform::attach(device);

        // let vertex_buffer = attach_vertex_buffer(&device, None);

        let primitive_pipeline = create_primitive_pipeline(
            config,
            device,
            1,
            &common_uniforms.bind_group_layout,
            &fragments_storage.bind_group_layout,
            &scene_storage.bind_group_layout,
            &fragments_data_uniform.bind_group_layout,
        );

        let grid_effect_pipeline =
            effects::grid::pipeline::create_pipeline(config, device, msaa_count);

        let wires_pipeline = wires::pipeline::create_pipeline(
            config,
            device,
            msaa_count,
            &common_uniforms.bind_group_layout,
            &scene_storage.bind_group_layout,
        );

        let pipelines = Pipelines {
            primitive: primitive_pipeline,
            wires: wires_pipeline,
            grid_effect: grid_effect_pipeline,
        };

        let mut fragments_data_uniform_map = HashMap::new();
        fragments_data_uniform_map.insert(0, fragments_data_uniform);

        let shared = Shared {
            chunk_data_uniform,
            time_uniform,
            common_uniforms,
            scene_storage,
            fragments_storage,
            fragments_data_uniform_map,
            // vertex_buffer,
            phantom: PhantomData,
        };

        let mut cache = Cache::default();

        // cache
        //     .fragments_comptype_index_map
        //     .insert(0, vec![(0, 400.0), (1, 1200.0)]);
        // cache
        //     .fragments_comptype_index_map
        //     .insert(1, vec![(2, 400.0)]);
        // cache
        //     .fragments_comptype_index_map
        //     .insert(2, vec![(0, 400.0)]);

        // info!("fragments_comptype_index_map: {:#?}", cache.fragments_comptype_index_map);

        // cache
        //     .fragments_comptype_index_map
        //     .iter()
        //     .for_each(|(ty, _)| {
        //         cache.last_lod_for_type.insert(*ty, 0);
        //     });

        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

        Self {
            depth_texture,
            shared,
            pipelines,
            cache: cache,
            msaa_count,
            time: 0,
            last_rendered: std::time::Instant::now(),
            phantom: PhantomData,
        }
    }

    pub fn set_msaa_count(&mut self, count: u32) {
        self.msaa_count = count;
    }

    pub fn rebuild_pipelines(&mut self, config: &SurfaceConfiguration, device: &Device) {
        self.pipelines.primitive = create_primitive_pipeline(
            config,
            device,
            self.msaa_count,
            &self.shared.common_uniforms.bind_group_layout,
            &self.shared.fragments_storage.bind_group_layout,
            &self.shared.scene_storage.bind_group_layout,
            &self.shared.fragments_data_uniform_map[&0].bind_group_layout,
        );

        self.pipelines.grid_effect =
            effects::grid::pipeline::create_pipeline(config, device, self.msaa_count);

        self.pipelines.wires = wires::pipeline::create_pipeline(
            config,
            device,
            self.msaa_count,
            &self.shared.common_uniforms.bind_group_layout,
            &self.shared.scene_storage.bind_group_layout,
        );
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
        ms_view: Option<&TextureView>,
        view: &TextureView,
        context: &Context,
        encoder: &mut CommandEncoder,
        camera_controller: &mut CameraController,
        scene: &Scene,
    ) {
        let t = std::time::Instant::now();
        let elapsed = t.duration_since(self.last_rendered);

        self.time += (elapsed.as_millis() as u32).clamp(0, 20);

        self.check_and_update_time_uniform(&context.queue);

        self.check_and_update_chunk_data_uniform(&context.queue, camera_controller);

        self.check_and_update_common_uniforms(&context.queue, camera_controller);

        self.check_and_update_scene_storage(
            &context.device,
            &context.queue,
            camera_controller,
            scene,
        );

        self.check_and_update_fragments_storage(&context.device, &context.queue, scene);

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: if ms_view.is_some() {
                    &ms_view.unwrap()
                } else {
                    view
                },
                resolve_target: if ms_view.is_some() { Some(view) } else { None },
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

        effects::render::render(
            &mut render_pass,
            &self.pipelines.grid_effect,
            &self.shared.common_uniforms.bind_group,
            &self.shared.time_uniform.bind_group,
            &self.shared.chunk_data_uniform.bind_group,
        );

        primitives::render::render(
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

        wires::render::render(
            &mut render_pass,
            &self.pipelines.wires,
            &self.shared.scene_storage,
            &self.shared.common_uniforms.bind_group,
            &self.shared.scene_storage.bind_group,
        );

        self.last_rendered = t;
    }

    fn check_and_update_chunk_data_uniform(
        &mut self,
        queue: &Queue,
        camera_controller: &CameraController,
    ) {
        let actual_chunk_size = camera_controller.chunk_size;
        let chunk_size = self
            .shared
            .chunk_data_uniform
            .uniform_buffer_data
            .get()
            .chunk_size;
        // info!("Actual chunk size {:#?}, shared: {:#?}", actual_chunk_size, chunk_size);

        // panic!();

        if chunk_size != actual_chunk_size {
            self.shared
                .chunk_data_uniform
                .uniform_buffer_data
                .set(ChunkData {
                    prev_chunk_size: chunk_size,
                    chunk_size: actual_chunk_size,
                    last_chunk_size_update: self.time,
                });

            queue.write_buffer(
                &self.shared.chunk_data_uniform.uniform_buffer_data.buffer(),
                0,
                self.shared
                    .chunk_data_uniform
                    .uniform_buffer_data
                    .encase_buffer
                    .as_ref(),
            );
            // panic!("Chunk size updated");
        } else {
            // panic!("Chunk size is the same");
        }
    }

    fn check_and_update_time_uniform(&mut self, queue: &Queue) {
        let actual_time = self.time;
        let time = self.shared.time_uniform.uniform_buffer_data.get().time;

        if time != actual_time {
            self.shared
                .time_uniform
                .uniform_buffer_data
                .set(TimeData { time: actual_time });

            queue.write_buffer(
                &self.shared.time_uniform.uniform_buffer_data.buffer(),
                0,
                self.shared
                    .time_uniform
                    .uniform_buffer_data
                    .encase_buffer
                    .as_ref(),
            );
        }
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
        scene: &Scene,
    ) {
        let fragments_storage = &mut self.shared.fragments_storage;
        let mut write = false;

        // Add primitives to the fragments storage
        for (compty, _) in self.cache.n_components_by_type.iter() {
            if self.cache.compty_fragments_index_map.get(compty).is_none() {
                match scene.primitives().get(compty) {
                    Some(primitives) => {
                        write = true;
                        for (primitive, max_dist) in primitives {
                            fragments_storage.add_primitives(primitive);

                            let mut idx_entry = self
                                .cache
                                .compty_fragments_index_map
                                .entry(*compty)
                                .or_insert(vec![]);
                            idx_entry.push((
                                (fragments_storage.component_ty_fragments.get().len() - 1) as u32,
                                *max_dist,
                            ));
                        }
                    }
                    None => {
                        // TODO: Add an unknown primitive for rendering in this situation
                    }
                };
            }
        }

        // Remove primitives from the fragments storage
        let mut to_remove_idx = Vec::new();
        self.cache.compty_fragments_index_map.retain(|ty, indices| {
            if self.cache.n_components_by_type.get(ty).is_none() {
                write = true;

                to_remove_idx.append(
                    &mut indices
                        .iter()
                        .map(|(idx, _)| *idx as usize)
                        .collect::<Vec<usize>>(),
                );

                return false;
            }

            true
        });

        to_remove_idx.iter().for_each(|remove_idx| {
            self.cache
                .compty_fragments_index_map
                .iter_mut()
                .for_each(|(ty, indices)| {
                    indices
                        .iter_mut()
                        .for_each(|(idx_in_fragments, _max_dist)| {
                            if (*remove_idx as u32) < *idx_in_fragments {
                                *idx_in_fragments = *idx_in_fragments - 1;
                            }
                        });
                });
        });

        fragments_storage.remove_primitives(to_remove_idx);

        if write {
            fragments_storage.write(device, queue);
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
        let chunk_size = camera_controller.chunk_size;
        // let half_chunk_size = scene.chunk_size() / 2.0;

        // debug!("Camera AABB: {:?}", camera_controller.screen_world_aabb);
        // debug!("Half chunk size: {}", half_chunk_size);
        let min_chunk =
            chunk_id_from_position(&camera_controller.screen_world_aabb.min, chunk_size);
        let max_chunk =
            chunk_id_from_position(&camera_controller.screen_world_aabb.max, chunk_size);

        let actual_chunk_range = ChunkRange {
            min_chunk,
            max_chunk,
        };

        if self.cache.scene_chunk_step_idx != camera_controller.chunk_step_idx as u32 {
            self.cache.scene_chunk_step_idx = camera_controller.chunk_step_idx as u32;
            self.clear_scene_storage(&device, &queue);
        }

        if self.cache.chunk_range.is_none()
            || self.cache.chunk_range.as_ref().unwrap() != &actual_chunk_range
        {
            let chunk_step_idx = self.cache.scene_chunk_step_idx;

            // debug!("Visible chunks changed, updating components, ({}, {}), ({}, {})", min_chunk.0, min_chunk.1, max_chunk.0, max_chunk.1);
            let scene_components = scene.components().get(&(chunk_step_idx as u32));
            let scene_wires = scene.wire_segments().get(&(chunk_step_idx as u32));

            let mut components = self.shared.scene_storage.components.get_mut();
            let mut wire_buffer = self.shared.scene_storage.wires.get_mut();
            let n_components_by_type = &mut self.cache.n_components_by_type;

            let (in_self_not_other, in_other_not_self) =
                if let Some(chunk_range) = self.cache.chunk_range.as_ref() {
                    chunk_range.diff(&actual_chunk_range)
                } else {
                    (Vec::new(), actual_chunk_range.clone().into_iter().collect())
                };

            let compute_start_positions = |n_components_by_type: &HashMap<u32, usize>| {
                let mut acc = 0;
                let mut start_positions = HashMap::<u32, usize>::new();
                let mut n_comps = 0;
                let mut i = 0;
                while n_comps < components.len() {
                    match n_components_by_type.get(&i) {
                        Some(n) => {
                            start_positions.insert(i, acc);
                            acc += n;
                            n_comps += n;
                        }
                        None => (),
                    }
                    i += 1;
                }

                start_positions
            };

            let (mut n_aditions, mut n_deletions) = (0, 0);

            let start_positions = compute_start_positions(n_components_by_type);

            // to_remove is a vec of (id, type)
            // let to_remove = Arc::new(Mutex::new(Vec::<(u32, u32)>::new()));
            let components_to_remove = Arc::new(Mutex::new(HashMap::<u32, u32>::new()));
            let wires_to_remove = Arc::new(Mutex::new(HashSet::<u32>::new()));
            {
                // To delete
                in_self_not_other.iter().for_each(|chunk_id| {
                    if components.len() > 0 && scene_components.is_some() {
                        let scene_components = scene_components.unwrap();

                        if let Some(chunk) = scene_components.get(&chunk_id) {
                            chunk.par_iter().for_each(|component| {
                                let start_p = start_positions.get(&component.ty()).unwrap_or(&0);
                                let n_components =
                                    n_components_by_type.get(&component.ty()).unwrap_or(&0);

                                if *n_components == 0 {
                                    return;
                                }

                                // get the slice for the type
                                let comps_slice = &components[*start_p..(*start_p + n_components)];

                                // insert ordered by id
                                if let Ok(_idx) = comps_slice.binary_search_by(|c| {
                                    c.ty()
                                        .cmp(&component.ty())
                                        .then(c.id().cmp(&component.id()))
                                }) {
                                    components_to_remove
                                        .lock()
                                        .unwrap()
                                        .insert(component.id(), component.ty());
                                }
                            });
                        }
                    }

                    if wire_buffer.len() > 0 && scene_wires.is_some() {
                        let scene_wires = scene_wires.unwrap();
                        if let Some(chunk) = scene_wires.get(&chunk_id) {
                            chunk.par_iter().for_each(|wire_id| {
                                let wire = scene.wires().get(wire_id).unwrap();

                                let bottom_left = chunk_id_from_position(&wire.start(), chunk_size);
                                let top_right = chunk_id_from_position(&wire.end(), chunk_size);

                                let wire_range = ChunkRange {
                                    min_chunk: bottom_left,
                                    max_chunk: top_right,
                                };

                                if actual_chunk_range.overlaps(&wire_range) {
                                    return;
                                }

                                if let Ok(_idx) =
                                    wire_buffer.binary_search_by(|w| w.id().cmp(&wire_id))
                                {
                                    wires_to_remove.lock().unwrap().insert(*wire_id);
                                }
                            });
                        }
                    }

                    // if let Some(chunk) =
                });
                let components_to_remove = components_to_remove.lock().unwrap().clone();

                components_to_remove.iter().for_each(|(_id, ty)| {
                    n_components_by_type.get_mut(ty).map(|n| {
                        *n -= 1;
                        n_deletions += 1;
                    });
                });

                components.retain(|comp| !components_to_remove.contains_key(&comp.id()));

                let wires_to_remove = wires_to_remove.lock().unwrap().clone();
                wire_buffer.retain(|wire| !wires_to_remove.contains(&wire.id()));
            }

            // Where to insert and the components to insert
            let components_to_insert = Arc::new(Mutex::new(Some(HashMap::<
                usize,
                Vec<ComponentBufferEntry>,
            >::new())));
            let mut wires_to_insert = Arc::new(Mutex::new(Some(HashMap::<
                usize,
                Vec<WireBufferEntry>,
            >::new())));
            {
                in_other_not_self.iter().for_each(|chunk_id| {
                    if scene_components.is_some() {
                        let scene_components = scene_components.unwrap();

                        if let Some(chunk) = scene_components.get(&chunk_id) {
                            chunk.par_iter().for_each(|component| {
                                // insert ordered by type chained with id
                                if let Err(i) = components.binary_search_by(|a| {
                                    a.ty()
                                        .cmp(&component.ty())
                                        .then(a.id().cmp(&component.id()))
                                }) {
                                    let mut to_insert = components_to_insert.lock().unwrap();

                                    // to_insert.entry(i).or_insert(Vec::new());
                                    // let entry = to_insert.get_mut(&i).unwrap();
                                    let entry =
                                        to_insert.as_mut().unwrap().entry(i).or_insert(Vec::new());

                                    // Insert in entry ordered by type chained with id
                                    if let Err(j) = entry.binary_search_by(|c| {
                                        c.ty()
                                            .cmp(&component.ty())
                                            .then(c.id().cmp(&component.id()))
                                    }) {
                                        entry.insert(
                                            j,
                                            ComponentBufferEntry::from_component(component),
                                        );
                                    }
                                }
                            });
                        }
                    }

                    if scene_wires.is_some() {
                        let scene_wires_ids = scene_wires.unwrap();

                        if let Some(chunk) = scene_wires_ids.get(&chunk_id) {
                            chunk.par_iter().for_each(|wire_id| {
                                // insert ordered by id
                                if let Err(i) =
                                    wire_buffer.binary_search_by(|w| w.id().cmp(&wire_id))
                                {
                                    let mut to_insert = wires_to_insert.lock().unwrap();

                                    let entry =
                                        to_insert.as_mut().unwrap().entry(i).or_insert(Vec::new());

                                    // Insert in entry ordered by type chained with id
                                    if let Err(j) = entry.binary_search_by(|w| w.id().cmp(&wire_id))
                                    {
                                        entry.insert(
                                            j,
                                            WireBufferEntry::from_wire(
                                                scene.wires().get(wire_id).unwrap(),
                                            ),
                                        );
                                    }
                                }
                            });
                        }
                    }
                });

                let to_insert = components_to_insert.lock().unwrap().take().unwrap();

                to_insert.iter().for_each(|(_idx, comps)| {
                    comps.iter().for_each(|c| {
                        n_components_by_type
                            .entry(c.ty)
                            .and_modify(|n| *n += 1)
                            .or_insert(1);
                        n_aditions += 1;
                    });
                });
                timed!(
                    insert_ordered_at(&mut components, to_insert),
                    "insert_ordered_at"
                );

                let to_insert = wires_to_insert.lock().unwrap().take().unwrap();
                insert_ordered_at(&mut wire_buffer, to_insert);
            }

            self.cache.chunk_range = Some(actual_chunk_range);

            self.cache.n_components_by_type.retain(|_, n| *n > 0);

            if n_aditions + n_deletions > 0 {
                self.shared.scene_storage.write(device, queue);
            }
        }
    }

    fn clear_scene_storage(&mut self, device: &Device, queue: &Queue) {
        self.cache.chunk_range = None;
        self.cache.n_components_by_type.clear();

        self.shared.scene_storage.components.set(Vec::new());
        self.shared.scene_storage.wires.set(Vec::new());

        self.shared.scene_storage.write(device, queue);
    }
}
