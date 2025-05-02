use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_enhanced_input::prelude::*;

const DEFAULT_SENSITIVITY: f32 = 0.002;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<OnFoot>()
        .add_input_context::<Inventory>()
        .add_input_context::<Interact>();

    app.add_observer(binding_on_foot)
        .add_observer(binding_interact)
        .add_observer(binding_inventory);
}

fn binding_on_foot(
    trigger: Trigger<Binding<OnFoot>>,
    mut on_foot: Query<&mut Actions<OnFoot>>,
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
        .to((KeyCode::Tab, GamepadButton::East));

    actions
        .bind::<OpenInteract>()
        .to((KeyCode::KeyE, GamepadButton::North));
}

fn binding_inventory(
    trigger: Trigger<Binding<Inventory>>,
    mut inventory: Query<&mut Actions<Inventory>>,
    mut window: Single<&mut Window>,
) {
    let mut actions = inventory.get_mut(trigger.target()).unwrap();

    window.cursor_options.grab_mode = CursorGrabMode::None;
    window.cursor_options.visible = true;

    actions
        .bind::<CloseInventory>()
        .to((KeyCode::Tab, GamepadButton::East));
}

fn binding_interact(
    trigger: Trigger<Binding<Interact>>,
    mut interact: Query<&mut Actions<Interact>>,
    mut window: Single<&mut Window>,
) {
    let mut actions = interact.get_mut(trigger.target()).unwrap();

    window.cursor_options.grab_mode = CursorGrabMode::None;
    window.cursor_options.visible = true;

    actions
        .bind::<CloseInteract>()
        .to((KeyCode::KeyE, GamepadButton::North));
}

#[derive(InputContext, Debug)]
pub(super) struct OnFoot;

#[derive(InputContext, Debug)]
pub(super) struct Inventory;

#[derive(InputContext, Debug)]
pub(super) struct Interact;

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
