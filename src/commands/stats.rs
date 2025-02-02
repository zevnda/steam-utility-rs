use crate::commands::init_steam_client;
use std::time::Duration;

// Update a stat for a game
pub fn update_stat(app_id: u32, stat_name: &str, value: i32) {
    // Initialize the Steam client
    let (client, _single) = init_steam_client(app_id).expect("Failed to initialize Steam client");

    let user_stats = client.user_stats();
    user_stats.request_current_stats();

    // Wait for the stats to be requested
    std::thread::sleep(Duration::from_millis(500));

    // Update the stat
    if let Ok(_) = user_stats.set_stat_i32(stat_name, value) {
        println!("Updated stat: {} = {}", stat_name, value);
    } else {
        println!("Failed to update stat: {} = {}", stat_name, value);
    }

    // Store the updated stats
    if let Ok(_) = user_stats.store_stats() {
        println!("Stored stats");
    } else {
        println!("Failed to store stats");
    }
}

// Reset all stats for a game
pub fn reset_all_stats(app_id: u32) {
    // Initialize the Steam client
    let (client, _single) = init_steam_client(app_id).expect("Failed to initialize Steam client");

    let user_stats = client.user_stats();

    // Reset all stats
    if let Ok(_) = user_stats.reset_all_stats(false) {
        println!("All stats reset");
    } else {
        println!("Failed to reset all stats");
    }

    // Store the updated stats
    if let Ok(_) = user_stats.store_stats() {
        println!("Stored stats");
    } else {
        println!("Failed to store stats");
    }
}
