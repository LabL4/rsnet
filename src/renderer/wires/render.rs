use tracing::info;
use wgpu::{BindGroup, RenderPass, RenderPipeline};

use crate::scene::shared::SceneStorage;

pub fn render<'b, 'c>(
    render_pass: &mut RenderPass<'c>,
    pipeline: &'b RenderPipeline,
    scene_storage: &'b SceneStorage,
    common_uniforms_bind_group: &'b BindGroup,
    scene_storage_bind_group: &'b BindGroup,
    // chunk_data_bind_group: &'b BindGroup,
) where
    'b: 'c,
{
    render_pass.set_pipeline(pipeline);

    render_pass.set_bind_group(0, common_uniforms_bind_group, &[]);
    render_pass.set_bind_group(1, scene_storage_bind_group, &[]);
    // render_pass.set_bind_group(2, chunk_data_bind_group, &[]);

    // info!("segments: {:?}", scene_storage.wire_segments.get().len());

    let n_segments = scene_storage.wire_segments.get().len() as u32;

    render_pass.draw(0..4, 0..n_segments)
}
