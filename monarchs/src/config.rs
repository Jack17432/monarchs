use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UserGamepadConfig>();
    }
}

#[derive(Resource)]
pub struct UserGamepadConfig {
    pub left_stick_deadzone: Vec2,
    pub left_stick_sensitivity: f32,
    pub right_stick_deadzone: Vec2,
    pub right_stick_sensitivity: f32,
}

impl Default for UserGamepadConfig {
    fn default() -> Self {
        Self {
            left_stick_deadzone: Vec2::new(0.1, 0.1),
            left_stick_sensitivity: 1.0,
            right_stick_deadzone: Vec2::new(0.1, 0.1),
            right_stick_sensitivity: 2.0,
        }
    }
}

impl UserGamepadConfig {
    pub fn apply_left_stick_config(&self, mut input: Vec2) -> Vec2 {
        if input.x.abs() < self.left_stick_deadzone.x {
            input.x = 0.0;
        }
        if input.y.abs() < self.left_stick_deadzone.y {
            input.y = 0.0;
        }

        input * self.left_stick_sensitivity
    }

    pub fn apply_right_stick_config(&self, mut input: Vec2) -> Vec2 {
        if input.x.abs() < self.right_stick_deadzone.x {
            input.x = 0.0;
        }
        if input.y.abs() < self.right_stick_deadzone.y {
            input.y = 0.0;
        }

        input * self.right_stick_sensitivity
    }
}
