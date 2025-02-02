use crate::commands::init_steam_client;
use std::time::Duration;

// Update a stat for a game
pub fn update_stat(app_id: u32, stat_name: &str, value: i32) {
    // Initialize the Steam client
    if let Ok((client, _single)) = init_steam_client(app_id) {
        let user_stats = client.user_stats();
        user_stats.request_current_stats();

        // Wait for the stats to be requested
        std::thread::sleep(Duration::from_millis(500));

        let mut success = false;

        // Update the stat
        if user_stats.set_stat_i32(stat_name, value).is_ok() {
            success = true;
        }

        // Store the updated stats
        if user_stats.store_stats().is_ok() && success {
            println!("{{\"success\":\"Successfully updated stat\"}}");
        } else {
            println!("{{\"error\":\"Failed to update stat\"}}");
        }
    } else {
        println!("{{\"fail\":\"Failed to initialize Steam client\"}}");
    }
}

// Reset all stats for a game
pub fn reset_all_stats(app_id: u32) {
    // Initialize the Steam client
    if let Ok((client, _single)) = init_steam_client(app_id) {
        let user_stats = client.user_stats();

        let mut success = false;

        // Reset all stats
        if user_stats.reset_all_stats(false).is_ok() {
            success = true;
        }

        // Store the updated stats
        if user_stats.store_stats().is_ok() && success {
            println!("{{\"success\":\"Successfully reset all stats\"}}");
        } else {
            println!("{{\"error\":\"Failed to reset all stats\"}}");
        }
    } else {
        println!("{{\"fail\":\"Failed to initialize Steam client\"}}");
    }
}
