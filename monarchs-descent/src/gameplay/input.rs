use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_enhanced_input::prelude::*;

const DEFAULT_SENSITIVITY: f32 = 0.002;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<PlayerActions>()
        .add_input_context::<InventoryActions>();

    app.add_observer(binding_player)
        .add_observer(binding_inventory);
}

fn binding_player(
    trigger: Trigger<Binding<PlayerActions>>,
    mut on_foot: Query<&mut Actions<PlayerActions>>,
    mut window: Single<&mut Window>,
) {
    let mut actions = on_foot.get_mut(trigger.target()).unwrap();

    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;

    actions
        .bind::<Rotate>()
        .to((Input::mouse_motion(), Axial::right_stick()))
        .with_modifiers((Negate::all(), Scale::splat(DEFAULT_SENSITIVITY)));

    actions
        .bind::<Move>()
        .to((Cardinal::wasd_keys(), Axial::left_stick()))
        .with_modifiers(DeadZone::default());

    actions
        .bind::<Jump>()
        .to((KeyCode::Space, GamepadButton::South));

    actions
        .bind::<OpenInventory>()
        .to((KeyCode::KeyE, GamepadButton::East));

    actions
        .bind::<OpenInteract>()
        .to((KeyCode::KeyF, GamepadButton::North));
}

fn binding_inventory(
    trigger: Trigger<Binding<InventoryActions>>,
    mut inventory: Query<&mut Actions<InventoryActions>>,
    mut window: Single<&mut Window>,
) {
    let mut actions = inventory.get_mut(trigger.target()).unwrap();

    window.cursor_options.grab_mode = CursorGrabMode::None;
    window.cursor_options.visible = true;

    actions
        .bind::<UiMove>()
        .to((Input::mouse_motion(), Axial::right_stick()))
        .with_modifiers((Negate::all(), Scale::splat(DEFAULT_SENSITIVITY)));

    actions
        .bind::<CloseInventory>()
        .to((KeyCode::KeyE, GamepadButton::East));
}

#[derive(InputContext, Debug)]
pub(super) struct PlayerActions;

#[derive(InputContext, Debug)]
#[input_context(priority = 1)]
pub(super) struct InventoryActions;

#[derive(InputAction, Debug)]
#[input_action(output = bool)]
pub(super) struct Jump;

#[derive(InputAction, Debug)]
#[input_action(output = Vec2)]
pub(super) struct Move;

#[derive(InputAction, Debug)]
#[input_action(output = Vec2)]
pub(super) struct Rotate;

#[derive(InputAction, Debug)]
#[input_action(output = Vec2)]
pub(super) struct UiMove;

#[derive(InputAction, Debug)]
#[input_action(output = bool, require_reset = true)]
pub(super) struct OpenInventory;

#[derive(InputAction, Debug)]
#[input_action(output = bool, require_reset = true)]
pub(super) struct CloseInventory;

#[derive(InputAction, Debug)]
#[input_action(output = bool, require_reset = true)]
pub(super) struct OpenInteract;

#[derive(InputAction, Debug)]
#[input_action(output = bool, require_reset = true)]
pub(super) struct CloseInteract;
