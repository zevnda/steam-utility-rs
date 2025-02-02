mod achievements;
mod idle;
mod stats;
mod users;

pub use achievements::{
    lock_achievement, lock_all_achievements, toggle_achievement, unlock_achievement,
};

pub use idle::idle;
pub use stats::{reset_all_stats, update_stat};
pub use users::get_users;

use steamworks::{AppId, Client};

// Public function to initialize the Steam client
pub fn init_steam_client(app_id: u32) -> Result<(Client, steamworks::SingleClient), String> {
    Client::init_app(AppId(app_id)).map_err(|e| e.to_string())
}
