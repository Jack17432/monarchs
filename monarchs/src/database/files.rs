use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_save_files);
}

fn setup_save_files() {
    let path = get_save_folder_path();
    std::fs::create_dir_all(path).unwrap()
}

fn get_save_folder_path() -> String {
    let sys_username = std::env::var("username").expect("username env not set");
    format!("C:/Users/{}/Documents/Monarchs/saves", sys_username)
}

pub fn get_saves() -> Vec<String> {
    let path = get_save_folder_path();
    let folders = std::fs::read_dir(path).unwrap();

    folders
        .map(|path| path.unwrap().file_name().to_str().unwrap().to_owned())
        .collect()
}

pub fn create_new_save_folder(name: String) -> Result<(), std::io::Error> {
    let path = get_save_folder_path();
    
    std::fs::create_dir(format!("{}/{}", path, name))
}

pub fn delete_save_folder(name: String) -> Result<(), std::io::Error> {
    let path = get_save_folder_path();
    
    info!(path = ?path, name = ?name, "deleting save folder");
    std::fs::remove_dir_all(format!("{}/{}", path, name))
}
