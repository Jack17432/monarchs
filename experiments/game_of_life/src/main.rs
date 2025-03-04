use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css::BLACK;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::PrimaryWindow;

const WIDTH: u32 = 50;
const HEIGHT: u32 = 50;
const IMG_SCALE: f32 = 13.0;

const MIN_NEIGHBORS_GAME_OF_LIFE: usize = 2;
const MAX_NEIGHBORS_GAME_OF_LIFE: usize = 3;
const RESPAWN_NEIGHBORS_GAME_OF_LIFE: usize = 3;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_img, game_state_keyboard, reset_game_state, mouse_click))
        .add_systems(FixedUpdate, step_game_of_life.run_if(in_state(AppState::Running)))
        .add_systems(OnEnter(AppState::Running), on_play)
        .add_systems(OnEnter(AppState::Paused), on_pause)
        .run();
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Paused,
    Running,
}

#[derive(Resource)]
struct GridResource {
    grid: Vec<Vec<bool>>,
    img_handle: Handle<Image>,
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct StateDisplay;

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn((Camera2d, MainCamera));

    let mut image = Image::new_fill(
        Extent3d {
            width: WIDTH,
            height: HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &(BLACK.to_u8_array()),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest();

    let grid = vec![vec![false; HEIGHT as usize + 2]; WIDTH as usize + 2];

    let img_handle = images.add(image);

    commands.spawn((
        Sprite::from_image(img_handle.clone()),
        Transform::default().with_scale(Vec3::splat(IMG_SCALE)),
    ));

    commands.insert_resource(GridResource {
        grid,
        img_handle,
    });

    commands.spawn((
        Text::new(format!("\
This is conways game of life, There are 4 rules\n\
- Cells need > {:?} or < {:?} neighbors\n\
- If {:?} neighbors, turn on\n\
- Else stay the same", 
        MIN_NEIGHBORS_GAME_OF_LIFE - 1, MAX_NEIGHBORS_GAME_OF_LIFE + 1, RESPAWN_NEIGHBORS_GAME_OF_LIFE)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        })
    );

    commands.spawn((
        Text::new("Space - Play / Pause\nR - Resets and spawns glider\nC - Clears the screen\nLeft mouse - add cell\nRight mouse - remove cell"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        })
    );

    commands.spawn((
        Text::new("Paused"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            right: Val::Px(12.0),
            ..default()
        },
        StateDisplay,
    ));
}

fn game_state_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(match current_state.get() {
            AppState::Running => AppState::Paused,
            AppState::Paused => AppState::Running,
        });
    }
}

fn on_play(
    text: Option<Single<Entity, (With<StateDisplay>, With<Text>)>>,
    mut writer: TextUiWriter,
) {
    if let Some(text) = text {
        *writer.text(*text, 0) = "Playing".to_string();
    }
}

fn on_pause(
    text: Option<Single<Entity, (With<StateDisplay>, With<Text>)>>,
    mut writer: TextUiWriter,
) {
    if let Some(text) = text {
        *writer.text(*text, 0) = "Paused".to_string();
    }
}

fn reset_game_state(
    mut grid: ResMut<GridResource>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        grid.grid = vec![vec![false; HEIGHT as usize + 2]; WIDTH as usize + 2];

        next_state.set(AppState::Paused);
        info!("Clearing state");
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        grid.grid = vec![vec![false; HEIGHT as usize + 2]; WIDTH as usize + 2];

        grid.grid[25 + 1][25 - 1] = true;
        grid.grid[25 + 1][25] = true;
        grid.grid[25 + 1][25 + 1] = true;

        grid.grid[25 - 1][25] = true;
        grid.grid[25][25 + 1] = true;

        next_state.set(AppState::Paused);
        info!("Resetting state");
    }
}

fn mouse_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut grid: ResMut<GridResource>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if mouse_button_input.any_pressed([MouseButton::Left, MouseButton::Right]) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .map(|ray| ray.origin.truncate())
        {
            let position: Vec2 = world_position;

            let x = (((position.x / IMG_SCALE) + (HEIGHT / 2) as f32).floor() + 1.0) as usize;
            let y = (((position.y / IMG_SCALE) + (WIDTH / 2) as f32).floor() + 1.0) as usize;

            if (x < HEIGHT as usize && x > 0) && (y < WIDTH as usize && y > 0) {
                if mouse_button_input.pressed(MouseButton::Left) {
                    grid.grid[x][WIDTH as usize - y + 1] = true;
                } else if mouse_button_input.pressed(MouseButton::Right) {
                    grid.grid[x][WIDTH as usize - y + 1] = false;
                }
            }
        }
    }
}

fn update_img(
    grid: Res<GridResource>,
    mut images: ResMut<Assets<Image>>,
) {
    let image = images.get_mut(&grid.img_handle).expect("Image not found");

    for x in 1..grid.grid.len() - 1 {
        let row = &grid.grid[x];
        for y in 1..row.len() - 1 {
            if row[y] {
                image.set_color_at(x as u32 - 1, y as u32 - 1, Color::from(Color::WHITE)).unwrap()
            } else {
                image.set_color_at(x as u32 - 1, y as u32 - 1, Color::from(Color::BLACK)).unwrap()
            }
        }
    }
}

fn step_game_of_life(mut grid: ResMut<GridResource>) {
    let mut new_grid = vec![vec![false; HEIGHT as usize + 2]; WIDTH as usize + 2];

    for x in 1..grid.grid.len() - 1 {
        for y in 1..grid.grid[x].len() - 1 {
            let neighbor_count = grid.grid[x - 1][y - 1] as usize +
                grid.grid[x - 1][y] as usize +
                grid.grid[x - 1][y + 1] as usize +
                grid.grid[x][y - 1] as usize +
                grid.grid[x][y + 1] as usize +
                grid.grid[x + 1][y - 1] as usize +
                grid.grid[x + 1][y] as usize +
                grid.grid[x + 1][y + 1] as usize;

            if neighbor_count < MIN_NEIGHBORS_GAME_OF_LIFE {
                new_grid[x][y] = false;
            } else if neighbor_count > MAX_NEIGHBORS_GAME_OF_LIFE {
                new_grid[x][y] = false;
            } else if neighbor_count == RESPAWN_NEIGHBORS_GAME_OF_LIFE {
                new_grid[x][y] = true;
            } else {
                new_grid[x][y] = grid.grid[x][y];
            }
        }
    }

    grid.grid = new_grid.clone();
}