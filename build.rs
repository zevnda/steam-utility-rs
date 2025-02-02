// cargo build --release; rustc .\copy_libs.rs; .\copy_libs.exe

use std::fs;
use std::path::PathBuf;

#[cfg(windows)]
extern crate winres;

fn create_dir_if_not_exists(dir: &PathBuf) {
    if !dir.exists() {
        fs::create_dir_all(dir).unwrap();
    }
}

fn copy_file(src: &str, dest: &PathBuf) {
    fs::copy(src, dest).unwrap();
}

fn main() {
    // Set the icon for the built exe
    let mut res = winres::WindowsResource::new();
    res.set_icon("res/icon.ico");
    res.compile().unwrap();

    // Create libs dir in tauri directory if it doesn't exist
    let tauri_libs_dir = PathBuf::from("../src-tauri/libs");
    create_dir_if_not_exists(&tauri_libs_dir);

    // Copy res files to target and libs dir
    let libs_res_dir = tauri_libs_dir.join("res");
    create_dir_if_not_exists(&libs_res_dir);
    copy_file("res/icon.ico", &libs_res_dir.join("icon.ico"));
    copy_file("res/icon.ico", &tauri_libs_dir.join("res/icon.ico"));

    // Copy all files from libs to tauri libs dir
    for entry in fs::read_dir("libs").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            copy_file(
                path.to_str().unwrap(),
                &tauri_libs_dir.join(path.file_name().unwrap()),
            );
        }
    }
}
