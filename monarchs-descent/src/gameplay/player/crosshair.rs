use crate::GameState;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CrosshairResource>();

    app.init_resource::<CrosshairResource>();

    app.add_systems(OnEnter(GameState::Playing), show_crosshair);
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
struct CrosshairResource {
    size: f32,
}

impl Default for CrosshairResource {
    fn default() -> Self {
        Self { size: 2.0 }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Crosshair;

fn show_crosshair(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    crosshair: Res<CrosshairResource>,
) {
    commands.spawn((
        Crosshair,
        StateScoped(GameState::Playing),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                width: Val::Percent(crosshair.size),
                ..default()
            },
            ImageNode::new(asset_server.load("Crosshair.png")),
            StateScoped(GameState::Playing),
        )],
    ));
}
