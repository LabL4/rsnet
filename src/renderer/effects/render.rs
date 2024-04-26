use wgpu::{BindGroup, RenderPass, RenderPipeline};

pub fn render<'b, 'c>(
    render_pass: &mut RenderPass<'c>,
    pipeline: &'b RenderPipeline,
    common_uniforms_bind_group: &'b BindGroup,
    time_bind_group: &'b BindGroup,
    chunk_data_bind_group: &'b BindGroup,
) where
    'b: 'c,
{
    render_pass.set_pipeline(pipeline);

    render_pass.set_bind_group(0, common_uniforms_bind_group, &[]);
    render_pass.set_bind_group(1, time_bind_group, &[]);
    render_pass.set_bind_group(2, chunk_data_bind_group, &[]);

    render_pass.draw(0..4, 0..1);
}
