use bevy::core_pipeline::core_3d::Opaque3d;
use bevy::ecs::query::ROQueryItem;
use bevy::ecs::system::SystemParamItem;
use crate::core::cubic;
use bevy::prelude::*;
use bevy::render::extract_resource::ExtractResourcePlugin;
use bevy::render::render_phase::{AddRenderCommand, RenderCommand, RenderCommandResult, SetItemPipeline, TrackedRenderPass};
use bevy::render::render_resource::{RenderPipelineDescriptor, SpecializedRenderPipeline, SpecializedRenderPipelines};
use bevy::render::RenderApp;

const BLOCK_SHADER: &str = "shaders/block.vert";

#[derive(Debug, Default)]
pub struct RenderBlocksPlugin;

impl Plugin for RenderBlocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<cubic::WorldBlocks>::default());

        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .init_resource::<RenderBlocksPipeline>()
            .init_resource::<SpecializedRenderPipelines<RenderBlocksPipeline>>()
            .add_render_command::<Opaque3d, DrawBlocksPhaseItemCommands>();
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
    type Key = ();

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        todo!()
    }
}

type DrawBlocksPhaseItemCommands = (SetItemPipeline, DrawBlocksPhaseItem);

struct DrawBlocksPhaseItem;

impl<P> RenderCommand<P> for DrawBlocksPhaseItem {
    type Param = ();
    type ViewQuery = ();
    type ItemQuery = ();

    fn render<'w>(item: &P, view: ROQueryItem<'w, Self::ViewQuery>, entity: Option<ROQueryItem<'w, Self::ItemQuery>>, param: SystemParamItem<'w, '_, Self::Param>, pass: &mut TrackedRenderPass<'w>) -> RenderCommandResult {
        todo!()
    }
}