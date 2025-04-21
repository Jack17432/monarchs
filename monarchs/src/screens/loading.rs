use crate::gameplay::camera::PlayerCamera;
use crate::screens::Screen;
use bevy::prelude::Val::{Percent, Px};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), load_loading_screen);
    app.add_systems(
        Update,
        advance_to_gameplay_screen.run_if(in_state(Screen::Loading)),
    );
}

fn load_loading_screen(mut commands: Commands) {
    info!("triggering Loading screen");

    commands.spawn((
        Name::new("UI Root"),
        Node {
            width: Percent(100.0),
            height: Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(10.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        StateScoped(Screen::Loading),
        BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 1.0)),
    ));
}

fn advance_to_gameplay_screen(
    player_camera: Query<&PlayerCamera>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if !player_camera.is_empty() {
        next_screen.set(Screen::Gameplay);
    }
}
