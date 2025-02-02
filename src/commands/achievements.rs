use crate::commands::init_steam_client;
use std::time::Duration;

// Unlock an achievement for a game
pub fn unlock_achievement(app_id: u32, achievement_id: &str) {
    // Initialize the Steam client
    if let Ok((client, _single)) = init_steam_client(app_id) {
        let user_stats = client.user_stats();
        user_stats.request_current_stats();

        // Wait for the stats to be requested
        std::thread::sleep(Duration::from_millis(500));

        let achievement = user_stats.achievement(achievement_id);

        let mut success = false;

        // Unlock the achievement
        if achievement.set().is_ok() {
            success = true;
        }

        // Store the updated stats
        if user_stats.store_stats().is_ok() && success {
            println!("{{\"success\":\"Successfully unlocked achievement\"}}");
        } else {
            println!("{{\"error\":\"Failed to unlock achievement\"}}");
        }
    } else {
        println!("{{\"fail\":\"Failed to initialize Steam client\"}}");
    }
}

// Lock an achievement for a game
pub fn lock_achievement(app_id: u32, achievement_id: &str) {
    // Initialize the Steam client
    if let Ok((client, _single)) = init_steam_client(app_id) {
        let user_stats = client.user_stats();
        user_stats.request_current_stats();

        // Wait for the stats to be requested
        std::thread::sleep(Duration::from_millis(500));

        let achievement = user_stats.achievement(achievement_id);

        let mut success = false;

        // Lock the achievement
        if achievement.clear().is_ok() {
            success = true;
        }

        // Store the updated stats
        if user_stats.store_stats().is_ok() && success {
            println!("{{\"success\":\"Successfully locked achievement\"}}");
        } else {
            println!("{{\"error\":\"Failed to lock achievement\"}}");
        }
    } else {
        println!("{{\"fail\":\"Failed to initialize Steam client\"}}");
    }
}

pub fn toggle_achievement(app_id: u32, achievement_id: &str) {
    // Initialize the Steam client
    if let Ok((client, _single)) = init_steam_client(app_id) {
        let user_stats = client.user_stats();
        user_stats.request_current_stats();

        // Wait for the stats to be requested
        std::thread::sleep(Duration::from_millis(500));

        let achievement = user_stats.achievement(achievement_id);
        let achieved = achievement.get().unwrap_or(false);

        let mut success = false;

        // Toggle the achievement
        if !achieved {
            if achievement.set().is_ok() {
                success = true;
            }
        } else {
            if achievement.clear().is_ok() {
                success = true;
            }
        }

        // Store the updated stats
        if user_stats.store_stats().is_ok() && success {
            println!("{{\"success\":\"Successfully toggled achievement\"}}");
        } else {
            println!("{{\"error\":\"Failed to toggle achievement\"}}");
        }
    } else {
        println!("{{\"fail\":\"Failed to initialize Steam client\"}}");
    }
}

// Lock all achievements for a game
pub fn lock_all_achievements(app_id: u32) {
    // Initialize the Steam client
    if let Ok((client, _single)) = init_steam_client(app_id) {
        let user_stats = client.user_stats();

        let mut success = false;

        // Lock all achievements
        if user_stats.reset_all_stats(true).is_ok() {
            success = true;
        }

        // Store the updated stats
        if user_stats.store_stats().is_ok() && success {
            println!("{{\"success\":\"Successfully locked all achievements\"}}");
        } else {
            println!("{{\"error\":\"Failed to lock all achievements\"}}");
        }
    } else {
        println!("{{\"fail\":\"Failed to initialize Steam client\"}}");
    }
}
