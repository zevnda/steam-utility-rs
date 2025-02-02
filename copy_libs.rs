use std::fs;
use std::path::PathBuf;

fn main() {
    let exe_name = "SteamUtility.exe";

    let src = PathBuf::from("target/release").join(exe_name);
    let dst = PathBuf::from("../src-tauri/libs").join(exe_name);

    fs::create_dir_all("../src-tauri/libs").expect("Failed to create libs directory");

    fs::copy(&src, &dst).unwrap_or_else(|e| panic!("Failed to copy executable: {}", e));

    println!("Successfully copied {} to {}", src.display(), dst.display());
}
