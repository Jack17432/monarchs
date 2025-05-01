use bevy::prelude::*;
use monarchs_descent::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
