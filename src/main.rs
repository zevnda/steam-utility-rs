mod commands;

use commands::{
    get_users, idle, lock_achievement, lock_all_achievements, reset_all_stats, unlock_achievement,
    update_stat,
};

use std::env;
use std::process;

struct Command {
    name: &'static str,
    usage: &'static str,
}

// List of available commands and their usage
const COMMANDS: &[Command] = &[
    Command {
        name: "get_users",
        usage: "get_users",
    },
    Command {
        name: "idle",
        usage: "idle <app_id>",
    },
    Command {
        name: "unlock_achievement",
        usage: "unlock_achievement <app_id> <achievement_id>",
    },
    Command {
        name: "lock_achievement",
        usage: "lock_achievement <app_id> <achievement_id>",
    },
    Command {
        name: "lock_all_achievements",
        usage: "lock_all_achievements <app_id>",
    },
    Command {
        name: "update_stat",
        usage: "update_stat <app_id> <stat_name> <value>",
    },
    Command {
        name: "reset_all_stats",
        usage: "reset_all_stats <app_id>",
    },
];

// Print help information
fn print_help(show_info: bool) {
    if show_info {
        println!("Version 1.0.0 by zevnda");
    }
    println!("\nUsage:");
    println!("      SteamUtility.exe <command> [args...]");
    println!("      SteamUtility.exe [--help | -h]");
    println!("\nCommands:                         Usage:");
    for cmd in COMMANDS {
        println!("      {:<30} SteamUtility.exe {}", cmd.name, cmd.usage);
    }
}

// Wait for the user to press Enter before exiting
// Used to keep the console window open
fn wait_for_enter() {
    println!("\nPress Enter to exit...");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}

fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: SteamUtility.exe <command> [args...]");
        println!("\nPress Enter to exit...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        process::exit(1);
    }
    let command = &args[1];

    // Check for help flag and print help information
    if command == "--help" || command == "-h" {
        print_help(true);
        return;
    }

    // Execute the command
    match command.as_str() {
        "get_users" => {
            if args.len() < 2 {
                eprintln!("Usage: SteamUtility.exe get_users");
                wait_for_enter();
                process::exit(1);
            }
            get_users();
        }
        "idle" => {
            if args.len() < 3 {
                eprintln!("Usage: SteamUtility.exe idle <app_id>");
                wait_for_enter();
                process::exit(1);
            }
            let app_id = args[2].parse::<u32>().expect("Invalid app_id");
            idle(app_id);
        }
        "unlock_achievement" => {
            if args.len() < 4 {
                eprintln!("Usage: SteamUtility.exe unlock_achievements <app_id> <achievement_id>");
                wait_for_enter();
                process::exit(1);
            }
            let app_id = args[2].parse::<u32>().expect("Invalid app_id");
            let achievement_id = args[3].clone();
            unlock_achievement(app_id, &achievement_id);
        }
        "lock_achievement" => {
            if args.len() < 4 {
                eprintln!("Usage: SteamUtility.exe lock_achievements <app_id> <achievement_id>");
                wait_for_enter();
                process::exit(1);
            }
            let app_id = args[2].parse::<u32>().expect("Invalid app_id");
            let achievement_id = args[3].clone();
            lock_achievement(app_id, &achievement_id);
        }
        "lock_all_achievements" => {
            if args.len() < 3 {
                eprintln!("Usage: SteamUtility.exe lock_all_achievements <app_id>");
                wait_for_enter();
                process::exit(1);
            }
            let app_id = args[2].parse::<u32>().expect("Invalid app_id");
            lock_all_achievements(app_id);
        }
        "update_stat" => {
            if args.len() < 5 {
                eprintln!("Usage: SteamUtility.exe update_stat <app_id> <stat_name> <value>");
                wait_for_enter();
                process::exit(1);
            }
            let app_id = args[2].parse::<u32>().expect("Invalid app_id");
            let stat_name = args[3].clone();
            let value = args[4].parse::<i32>().expect("Invalid value");
            update_stat(app_id, &stat_name, value);
        }
        "reset_all_stats" => {
            if args.len() < 3 {
                eprintln!("Usage: SteamUtility.exe reset_all_stats <app_id>");
                wait_for_enter();
                process::exit(1);
            }
            let app_id = args[2].parse::<u32>().expect("Invalid app_id");
            reset_all_stats(app_id);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_help(false);
            process::exit(1);
        }
    }
}
