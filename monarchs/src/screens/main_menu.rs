use crate::database::files;
use crate::database::saves::{CreateNewSave, LoadSave};
use crate::screens::Screen;
use bevy::prelude::*;
use bevy_egui::egui::Pos2;
use bevy_egui::{egui, EguiContexts};
use std::cmp::PartialEq;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        main_menu_screen_egui_system.run_if(in_state(Screen::MainMenu)),
    );
}

#[derive(Debug, Clone)]
struct NewGameMenu {
    name: String,
    error: Option<NewGameErrors>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum NewGameErrors {
    SaveName,
}

impl Default for NewGameMenu {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            error: None,
        }
    }
}

#[derive(Debug, Clone)]
struct MainMenuScreen {
    new_game_open: bool,
    saves_open: bool,
    new_game_menu: NewGameMenu,
}

impl Default for MainMenuScreen {
    fn default() -> Self {
        Self {
            new_game_open: false,
            saves_open: false,
            new_game_menu: NewGameMenu::default(),
        }
    }
}

fn main_menu_screen_egui_system(
    mut contexts: EguiContexts,
    window: Single<&Window>,
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut main_menu: Local<MainMenuScreen>,
    mut commands: Commands,
) {
    egui::Window::new("Monarchs")
        .default_pos(Pos2::new(
            window.width() / 2.0 - 100.0,
            window.height() / 2.0 - 50.0,
        ))
        .show(contexts.ctx_mut(), |ui| {
            if ui.button("New Game").clicked() {
                main_menu.new_game_open = true;
            }

            if ui.button("Saves").clicked() {
                main_menu.saves_open = true;
            }

            if ui.button("Exit").clicked() {
                app_exit_events.send(AppExit::Success);
            }
        });

    if main_menu.new_game_open {
        egui::Window::new("New Game")
            .default_pos(Pos2::new(
                window.width() / 2.0 - 100.0,
                window.height() / 2.0 - 50.0,
            ))
            .show(contexts.ctx_mut(), |ui| {
                ui.label("Name");
                ui.add(egui::TextEdit::singleline(
                    &mut main_menu.new_game_menu.name,
                ));
                ui.end_row();

                if main_menu
                    .new_game_menu
                    .error
                    .is_some_and(|val| val == NewGameErrors::SaveName)
                {
                    ui.label("Another save has that name!");
                }

                ui.separator();

                if ui.button("Start new game").clicked() {
                    let res = files::create_new_save_folder(main_menu.new_game_menu.name.clone());
                    if res.is_err() {
                        main_menu.new_game_menu.error = Some(NewGameErrors::SaveName);
                    } else {
                        commands.trigger(CreateNewSave {
                            name: main_menu.new_game_menu.name.clone(),
                        });
                    }
                };
            });
    }

    if main_menu.saves_open {
        egui::Window::new("Saves")
            .default_pos(Pos2::new(
                window.width() / 2.0 - 100.0,
                window.height() / 2.0 - 50.0,
            ))
            .show(contexts.ctx_mut(), |ui| {
                let saves = files::get_saves();

                if saves.len() == 0 {
                    ui.heading("No saves");
                    return;
                }

                for save_name in saves {
                    if ui.button(&save_name).clicked() {
                        info!(save_name = ?save_name, "Loading save file");
                        commands.trigger(LoadSave {
                            name: save_name.clone(),
                        })
                    }
                    
                    if ui.button("delete").clicked() {
                        info!(save_name = ?save_name, "Deleting save file");
                        let _ = files::delete_save_folder(save_name.clone());
                    }
                    ui.end_row();
                    ui.separator();
                }
            });
    }
}
