mod input;
mod toggle_ui;

use crate::editor::input::EditorActions;
use crate::editor::toggle_ui::toggle;
use avian3d::prelude::{Physics, PhysicsGizmos, PhysicsTime};
use bevy::asset::{ReflectAsset, UntypedAssetId};
use bevy::prelude::*;
use bevy::reflect::TypeRegistry;
use bevy::render::camera::Viewport;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::{Ui, WidgetText};
use bevy_egui::{EguiContext, EguiContextPass, EguiContextSettings, egui};
use bevy_enhanced_input::actions::Actions;
use bevy_inspector_egui::bevy_inspector::hierarchy::{Hierarchy, SelectedEntities};
use bevy_inspector_egui::bevy_inspector::{
    Filter, ui_for_entities_shared_components, ui_for_entity_with_children,
};
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use std::any::TypeId;
use std::cmp::PartialEq;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(input::plugin);

    app.insert_resource(EditorUiState::new());

    app.insert_state(EditorState::Open);

    app.add_systems(Startup, setup_editor)
        .add_systems(
            EguiContextPass,
            show_editor_ui_system.run_if(in_state(EditorState::Open)),
        )
        .add_systems(
            PostUpdate,
            set_camera_viewport_system
                .after(show_editor_ui_system)
                .run_if(in_state(EditorState::Open)),
        )
        .add_systems(
            PostUpdate,
            hide_editor_ui_system.run_if(in_state(EditorState::Closed)),
        );
}

#[derive(States, Debug, Copy, Clone, PartialEq, Hash, Eq)]
enum EditorState {
    Closed,
    Open,
}

fn setup_editor(mut commands: Commands) {
    info!("Setting up editor");
    commands.spawn(Actions::<EditorActions>::default());
}

fn hide_editor_ui_system(mut cameras: Query<&mut Camera>) {
    for mut cam in cameras.iter_mut() {
        cam.viewport = None;
    }
}

fn show_editor_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
    else {
        return;
    };

    let mut egui_context = egui_context.clone();

    world.resource_scope::<EditorUiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut());
    });
}

fn set_camera_viewport_system(
    ui_state: Res<EditorUiState>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Single<&EguiContextSettings>,
    mut cameras: Query<&mut Camera>,
) {
    let Ok(window) = primary_window.single() else {
        warn_once!("Trying to use editor with multiple primary windows");
        return;
    };

    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos = ui_state.viewport_rect.left_top().to_vec2() * scale_factor;
    let viewport_size = ui_state.viewport_rect.size() * scale_factor;

    let physical_position = UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32);
    let physical_size = UVec2::new(viewport_size.x as u32, viewport_size.y as u32);

    let rect = physical_position + physical_size;

    let window_size = window.physical_size();

    if rect.x <= window_size.x && rect.y <= window_size.y {
        for mut cam in cameras.iter_mut() {
            cam.viewport = Some(Viewport {
                physical_position,
                physical_size,
                depth: 0.0..1.0,
            });
        }
    }
}

#[derive(Debug, PartialEq)]
enum EditorWindowTabs {
    GameView,
    Entities,
    Children,
    Resources,
    Assets,
    Inspector,
    Physics,
}

#[derive(Eq, PartialEq)]
enum InspectorSelection {
    Entities,
    Resource(TypeId, String),
    Asset(TypeId, String, UntypedAssetId),
}

#[derive(Resource)]
struct EditorUiState {
    state: DockState<EditorWindowTabs>,
    viewport_rect: egui::Rect,
    selected_entities: SelectedEntities,
    selection: InspectorSelection,
}

impl EditorUiState {
    fn new() -> Self {
        let mut state = DockState::new(vec![EditorWindowTabs::GameView]);
        let tree = state.main_surface_mut();

        let [game, inspector] =
            tree.split_right(NodeIndex::root(), 0.75, vec![EditorWindowTabs::Inspector]);
        let [_physics, _inspector] =
            tree.split_above(inspector, 0.2, vec![EditorWindowTabs::Physics]);
        let [game, hierarchy] = tree.split_left(
            game,
            0.2,
            vec![EditorWindowTabs::Entities, EditorWindowTabs::Children],
        );
        let [_hierarchy, _resources] =
            tree.split_below(hierarchy, 0.5, vec![EditorWindowTabs::Resources]);
        let [_game, _bottom] = tree.split_below(game, 0.8, vec![EditorWindowTabs::Assets]);

        Self {
            state,
            selected_entities: SelectedEntities::default(),
            selection: InspectorSelection::Entities,
            viewport_rect: egui::Rect::NOTHING,
        }
    }

    fn ui(&mut self, world: &mut World, egui_context: &mut egui::Context) {
        let mut tab_view = EditorTabViewer {
            world,
            viewport_rect: &mut self.viewport_rect,
            selected_entities: &mut self.selected_entities,
            selection: &mut self.selection,
        };

        DockArea::new(&mut self.state)
            .style(Style::from_egui(egui_context.style().as_ref()))
            .show(egui_context, &mut tab_view);
    }
}

struct EditorTabViewer<'a> {
    world: &'a mut World,
    selected_entities: &'a mut SelectedEntities,
    selection: &'a mut InspectorSelection,
    viewport_rect: &'a mut egui::Rect,
}

impl egui_dock::TabViewer for EditorTabViewer<'_> {
    type Tab = EditorWindowTabs;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        format!("{tab:?}").into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        let type_registry = type_registry.read();

        match tab {
            EditorWindowTabs::GameView => {
                *self.viewport_rect = ui.clip_rect();
            }
            EditorWindowTabs::Entities => {
                let filter: Filter =
                    Filter::from_ui_fuzzy(ui, egui::Id::new("default_world_entities_filter"));

                let selected = Hierarchy {
                    world: self.world,
                    type_registry: &type_registry,
                    selected: self.selected_entities,
                    context_menu: None,
                    shortcircuit_entity: None,
                    extra_state: &mut (),
                }
                .show_with_filter::<Without<Observer>, _>(ui, filter);
                if selected {
                    *self.selection = InspectorSelection::Entities;
                }
            }
            EditorWindowTabs::Resources => select_resource(ui, &type_registry, self.selection),
            EditorWindowTabs::Assets => {
                select_asset(ui, &type_registry, self.world, self.selection)
            }
            EditorWindowTabs::Inspector => match *self.selection {
                InspectorSelection::Entities => match self.selected_entities.as_slice() {
                    &[entity] => ui_for_entity_with_children(self.world, entity, ui),
                    entities => ui_for_entities_shared_components(self.world, entities, ui),
                },
                InspectorSelection::Resource(type_id, ref name) => {
                    ui.label(name);
                    bevy_inspector_egui::bevy_inspector::by_type_id::ui_for_resource(
                        self.world,
                        type_id,
                        ui,
                        name,
                        &type_registry,
                    );
                }
                InspectorSelection::Asset(type_id, ref name, handle) => {
                    ui.label(name);
                    bevy_inspector_egui::bevy_inspector::by_type_id::ui_for_asset(
                        self.world,
                        type_id,
                        handle,
                        ui,
                        &type_registry,
                    );
                }
            },
            EditorWindowTabs::Children => {
                bevy_inspector_egui::bevy_inspector::ui_for_entities_with_relationship::<Children>(
                    self.world, ui, true,
                );
            }
            EditorWindowTabs::Physics => physics_ui(ui, &mut self.world),
        }
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        *tab != EditorWindowTabs::GameView
    }
}

fn select_resource(ui: &mut Ui, type_registry: &TypeRegistry, selection: &mut InspectorSelection) {
    let mut resources: Vec<_> = type_registry
        .iter()
        .filter(|registration| registration.data::<ReflectResource>().is_some())
        .map(|registration| {
            (
                registration.type_info().type_path_table().short_path(),
                registration.type_id(),
            )
        })
        .collect();
    resources.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));

    for (resource_name, type_id) in resources {
        let selected = match *selection {
            InspectorSelection::Resource(selected, _) => selected == type_id,
            _ => false,
        };

        if ui.selectable_label(selected, resource_name).clicked() {
            *selection = InspectorSelection::Resource(type_id, resource_name.to_string());
        }
    }
}

fn physics_ui(ui: &mut Ui, world: &mut World) {
    ui.heading("Physics World Config");

    egui::Grid::new("physics control grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            let mut physics_time = world.get_resource_mut::<Time<Physics>>().unwrap();

            ui.label("Pause");
            let mut state = physics_time.is_paused();
            ui.add(toggle(&mut state));
            if state != physics_time.is_paused() {
                match state {
                    true => physics_time.pause(),
                    false => physics_time.unpause(),
                }
            }
            ui.end_row();

            ui.label("Timestamp");
            ui.label(format!("{:?}", physics_time.elapsed()));
            ui.end_row();

            ui.label("Relative speed");
            let mut rel_time = physics_time.relative_speed();
            ui.add(egui::DragValue::new(&mut rel_time).speed(0.01));
            physics_time.set_relative_speed(rel_time);
            ui.end_row();

            let (gizmos_config, physics_gizmos) = world
                .get_resource_mut::<GizmoConfigStore>()
                .unwrap()
                .into_inner()
                .config_mut::<PhysicsGizmos>();

            ui.label("Hide mesh");
            ui.add(toggle(&mut physics_gizmos.hide_meshes));
            ui.end_row();

            ui.label("Show gizmos");
            ui.add(toggle(&mut gizmos_config.enabled));
            ui.end_row();
        });
}

fn select_asset(
    ui: &mut Ui,
    type_registry: &TypeRegistry,
    world: &World,
    selection: &mut InspectorSelection,
) {
    let mut assets: Vec<_> = type_registry
        .iter()
        .filter_map(|registration| {
            let reflect_asset = registration.data::<ReflectAsset>()?;
            Some((
                registration.type_info().type_path_table().short_path(),
                registration.type_info(),
                reflect_asset,
            ))
        })
        .collect();
    assets.sort_by(|(name_a, ..), (name_b, ..)| name_a.cmp(name_b));

    for (asset_name, asset_type_id, reflect_asset) in assets {
        let handles: Vec<_> = reflect_asset.ids(world).collect();

        ui.collapsing(format!("{asset_name} ({})", handles.len()), |ui| {
            for handle in handles {
                let selected = match *selection {
                    InspectorSelection::Asset(_, _, selected_id) => selected_id == handle,
                    _ => false,
                };

                if ui
                    .selectable_label(selected, format!("{handle:?}"))
                    .clicked()
                {
                    *selection = InspectorSelection::Asset(
                        asset_type_id.type_id(),
                        asset_name.to_string(),
                        handle,
                    );
                }
            }
        });
    }
}
