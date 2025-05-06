use crate::editor::EditorState;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<EditorActions>();

    app.add_observer(bind_editor_actions);

    app.add_observer(toggle_editor);
}

fn bind_editor_actions(
    trigger: Trigger<Binding<EditorActions>>,
    mut editor_actions: Query<&mut Actions<EditorActions>>,
) {
    let mut actions = editor_actions.get_mut(trigger.target()).unwrap();

    actions
        .bind::<ToggleEditor>()
        .to(KeyCode::Backslash)
        .with_conditions(Release::default());
}

#[derive(InputContext, Clone, Debug)]
pub struct EditorActions;

#[derive(InputAction, Clone, Debug)]
#[input_action(output = bool)]
pub struct ToggleEditor;

fn toggle_editor(
    _trigger: Trigger<Fired<ToggleEditor>>,
    state: Res<State<EditorState>>,
    mut next_state: ResMut<NextState<EditorState>>,
) {
    info!("??");

    match state.get() {
        EditorState::Closed => {
            next_state.set(EditorState::Open);
        }
        EditorState::Open => {
            next_state.set(EditorState::Closed);
        }
    }
}
