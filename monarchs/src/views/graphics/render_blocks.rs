use crate::core::cubic;
use bevy::core_pipeline::core_3d::Opaque3d;
use bevy::ecs::query::ROQueryItem;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParamItem;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::render_phase::{
    AddRenderCommand, PhaseItem, RenderCommand, RenderCommandResult, SetItemPipeline,
    TrackedRenderPass,
};
use bevy::render::render_resource::{BufferUsages, FragmentState, MultisampleState, PrimitiveState, RawBufferVec, RenderPipelineDescriptor, ShaderType, SpecializedRenderPipeline, SpecializedRenderPipelines, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::{Render, RenderApp, RenderSet};

const BLOCK_SHADER: &str = "shaders/block.wgsl";

#[derive(Debug, Default)]
pub struct RenderBlocksPlugin;

impl Plugin for RenderBlocksPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(ExtractResourcePlugin::<cubic::Chunk>::default());

        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .init_resource::<RenderBlocksPipeline>()
            .init_resource::<SpecializedRenderPipelines<RenderBlocksPipeline>>()
            .add_render_command::<Opaque3d, DrawBlocksPhaseItemCommands>()
            .add_systems(
                Render,
                prepare_phase_item_buffers.in_set(RenderSet::Prepare),
            )
            .add_systems(Render, queue_);
    }
}

#[derive(Debug, Resource)]
struct RenderBlocksPipeline {
    shader: Handle<Shader>,
}

impl FromWorld for RenderBlocksPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            shader: asset_server.load(BLOCK_SHADER).into(),
        }
    }
}

impl SpecializedRenderPipeline for RenderBlocksPipeline {
    type Key = Msaa;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some("Block render pipeline".into()),
            layout: vec![],
            push_constant_ranges: vec![],
            vertex: VertexState {
                shader: self.shader.clone(),
                shader_defs: vec![],
                entry_point: "vertex".into(),
                buffers: vec![VertexBufferLayout {
                    array_stride: size_of::<Vec3>() as u64,
                    step_mode: VertexStepMode::Vertex,
                    attributes: vec![VertexAttribute {
                        format: VertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 0,
                    }],
                }],
            },
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                shader_defs: vec![],
                entry_point: "fragment".into(),
                targets: vec![],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState {
                count: key.samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            zero_initialize_workgroup_memory: false,
        }
    }
}

type DrawBlocksPhaseItemCommands = (SetItemPipeline, DrawBlocksPhaseItem);

struct DrawBlocksPhaseItem;

impl<P> RenderCommand<P> for DrawBlocksPhaseItem
where
    P: PhaseItem,
{
    type Param = SRes<BlockPhaseItemBuffer>;
    type ViewQuery = ();
    type ItemQuery = ();

    fn render<'w>(
        item: &P,
        view: ROQueryItem<'w, Self::ViewQuery>,
        entity: Option<ROQueryItem<'w, Self::ItemQuery>>,
        param: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        RenderCommandResult::Success
    }
}

#[derive(Resource)]
struct BlockPhaseItemBuffer {
    vbo: RawBufferVec<Vec3>,
    ibo: RawBufferVec<u32>,
}

impl FromWorld for BlockPhaseItemBuffer {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let render_queue = world.resource::<RenderQueue>();

        let mut vbo = RawBufferVec::new(BufferUsages::VERTEX);
        let mut ibo = RawBufferVec::new(BufferUsages::INDEX);

        let vertices = [vec3(0.0, 1.0, 0.0), vec3(-1.0, 0.0, 1.0), vec3(1.0, 0.0, 1.0)];
        for idx in 0..vertices.len() {
            vbo.push(vertices[idx]);
            ibo.push(idx as u32);
        }

        vbo.write_buffer(render_device, render_queue);
        ibo.write_buffer(render_device, render_queue);

        BlockPhaseItemBuffer { vbo, ibo }
    }
}

fn prepare_phase_item_buffers(mut commands: Commands) {
    commands.init_resource::<BlockPhaseItemBuffer>();
}

fn queue_phase_item(

)
