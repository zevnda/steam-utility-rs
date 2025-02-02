use regex::Regex;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

// Get users from Steam's loginusers.vdf file
pub fn get_users() {
    // Find the Steam directory
    match find_steam_directory() {
        Ok(config_path) => match parse_login_users(&config_path) {
            Ok(users) => {
                if !users.is_empty() {
                    // Create a list of users with personaName and steamId
                    let user_list: Vec<HashMap<&str, &str>> = users
                        .iter()
                        .map(|(key, value)| {
                            let mut user = HashMap::new();
                            user.insert("personaName", value.as_str());
                            user.insert("steamId", key.as_str());
                            user
                        })
                        .collect();

                    // Convert the user list to JSON and print it
                    let users_json = json!(user_list).to_string();
                    println!("steamUsers {}", users_json);
                }
            }
            Err(e) => eprintln!("Error parsing loginusers file: {}", e),
        },
        Err(e) => eprintln!("Error finding Steam directory: {}", e),
    }
}

// Find the Steam directory
fn find_steam_directory() -> Result<PathBuf, String> {
    match get_steam_path_from_registry() {
        Some(steam_path) => {
            let config_path = steam_path.join("config").join("loginusers.vdf");
            if config_path.exists() {
                Ok(config_path)
            } else {
                Err(format!("A loginusers.vdf file was not found in the Steam config directory.\nSteam Path: {:?}", steam_path))
            }
        }
        None => Err("Steam directory not found.".to_string()),
    }
}

// Get the Steam path from the Windows registry
fn get_steam_path_from_registry() -> Option<PathBuf> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(steam_key) = hkcu.open_subkey("Software\\Valve\\Steam") {
        if let Ok(steam_path) = steam_key.get_value::<String, _>("SteamPath") {
            return Some(PathBuf::from(steam_path));
        }
    }
    eprintln!("Unable to find Steam's installation directory. Is it installed?");
    None
}

// Parse the loginusers.vdf file and extract user information
fn parse_login_users(config_path: &PathBuf) -> Result<HashMap<String, String>, String> {
    let content = fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    let user_regex = Regex::new(r#""(\d{17})"\s*\{[^}]*"PersonaName"\s*"([^"]*)""#)
        .map_err(|e| e.to_string())?;
    let mut users = HashMap::new();

    for cap in user_regex.captures_iter(&content) {
        let steam_id = cap[1].to_string();
        let persona_name = cap[2].to_string();
        users.insert(steam_id, persona_name);
    }

    Ok(users)
}
