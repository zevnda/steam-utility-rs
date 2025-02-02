use crate::commands::init_steam_client;
use std::time::Duration;

// Unlock an achievement for a game
pub fn unlock_achievement(app_id: u32, achievement_id: &str) {
    // Initialize the Steam client
    let (client, _single) = init_steam_client(app_id).expect("Failed to initialize Steam client");

    let user_stats = client.user_stats();
    user_stats.request_current_stats();

    // Wait for the stats to be requested
    std::thread::sleep(Duration::from_millis(500));

    let achievement = user_stats.achievement(achievement_id);

    // Unlock the achievement
    if let Ok(_) = achievement.set() {
        println!("Unlocked achievement: {}", achievement_id);
    } else {
        println!("Failed to unlock achievement: {}", achievement_id);
    }

    // Store the updated stats
    if let Ok(_) = user_stats.store_stats() {
        println!("Stored stats");
    } else {
        println!("Failed to store stats");
    }
}

// Lock an achievement for a game
pub fn lock_achievement(app_id: u32, achievement_id: &str) {
    // Initialize the Steam client
    let (client, _single) = init_steam_client(app_id).expect("Failed to initialize Steam client");

    let user_stats = client.user_stats();
    user_stats.request_current_stats();

    // Wait for the stats to be requested
    std::thread::sleep(Duration::from_millis(500));

    let achievement = user_stats.achievement(achievement_id);

    // Lock the achievement
    if let Ok(_) = achievement.clear() {
        println!("Locked achievement: {}", achievement_id);
    } else {
        println!("Failed to lock achievement: {}", achievement_id);
    }

    // Store the updated stats
    if let Ok(_) = user_stats.store_stats() {
        println!("Stored stats");
    } else {
        println!("Failed to store stats");
    }
}

// Lock all achievements for a game
pub fn lock_all_achievements(app_id: u32) {
    // Initialize the Steam client
    let (client, _single) = init_steam_client(app_id).expect("Failed to initialize Steam client");

    let user_stats = client.user_stats();

    // Lock all achievements
    if let Ok(_) = user_stats.reset_all_stats(true) {
        println!("All achievements locked");
    } else {
        println!("Failed to lock all achievements");
    }

    // Store the updated stats
    if let Ok(_) = user_stats.store_stats() {
        println!("Stored stats");
    } else {
        println!("Failed to store stats");
    }
}
