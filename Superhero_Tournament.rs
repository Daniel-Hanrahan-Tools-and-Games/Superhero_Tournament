use std::io;
use std::process;
use std::thread;
use std::time::Duration;
use std::fs;
use rand::Rng;
use mlua::Lua;

fn main() {
    // License notice
    println!("Copyright (C) 2025 Daniel Hanrahan Tools and Games");
    println!("This program is free software under the GNU GPL v3 or later.");
    println!("Anything not covered by GPLv3 is licensed under CC BY-SA 4.0.");

    // Ask about mods
    let mut str_mod_input = String::new();
    println!("Would you like to use mods, yes(1) or no(2):");
    io::stdin().read_line(&mut str_mod_input).expect("Failed to read input");

    let int_mod_input: u8 = match str_mod_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number, Goodbye");
            process::exit(0);
        }
    };

    let lua = Lua::new();
    let enemy_name: Option<mlua::Function>;
    let str_mod_name_and_notices: Option<String>;

    if int_mod_input == 1 {
        let code = fs::read_to_string("Superhero_Tournament_Mod.lua")
            .expect("Failed to load Mod file.");
        lua.load(&code).exec().expect("Failed to run Mod file.");

        let globals = lua.globals();
        enemy_name = Some(globals.get("EnemyName").expect("EnemyName not found"));
        str_mod_name_and_notices = Some(globals.get("strNoticeAndName").expect("Missing notice"));
        println!("Loaded Mod:\n{}", str_mod_name_and_notices.as_ref().unwrap());
    } else {
        println!("You decided not to use mods");
        enemy_name = None;
        str_mod_name_and_notices = None;
    }

    // Main loop
    loop {
        let mut rng = rand::thread_rng();
        let mut int_enemy_action: u8 = rng.gen_range(0..=1);
        let mut int_show_description: u8 = rng.gen_range(0..=2);

        let int_max_health = 100;

        let mut int_player_extra_attack_power: u8 = 0;
        let mut int_player_attack_power: u8 = 5;
        let mut int_player_damage: u8 = 0;
        let mut int_player_health: i8 = int_max_health;

        let int_player_type: u8 = rng.gen_range(1..=4);

        let mut int_enemy_extra_attack_power: u8 = 0;
        let mut int_enemy_attack_power: u8 = 5;
        let mut int_enemy_damage: u8 = 0;
        let mut int_enemy_health: i8 = int_max_health;

        let int_enemy_type: u8 = rng.gen_range(1..=4);

        println!("--- New Battle ---");

        // --- Type matchup system ---
        if int_player_type == 1 && int_enemy_type == 3 {
            println!("You are closest to a Tool User, strong against Wizard!");
            int_player_attack_power = 10;
            int_enemy_attack_power = 5;
        } else if int_player_type == 2 && int_enemy_type == 1 {
            println!("You are closest to SuperPowered, strong against Tool User!");
            int_player_attack_power = 10;
            int_enemy_attack_power = 5;
        } else if int_player_type == 3 && int_enemy_type == 2 {
            println!("You are closest to a Wizard, strong against SuperPowered!");
            int_player_attack_power = 10;
            int_enemy_attack_power = 5;
        } else if int_player_type == 2 && int_enemy_type == 3 {
            println!("You are closest to SuperPowered, weak against Wizard!");
            int_player_attack_power = 5;
            int_enemy_attack_power = 10;
        } else if int_player_type == 1 && int_enemy_type == 2 {
            println!("You are a closest to Tool User, weak against SuperPowered!");
            int_player_attack_power = 5;
            int_enemy_attack_power = 10;
        } else if int_player_type == 3 && int_enemy_type == 1 {
            println!("You are closest to a Wizard, weak against Tool User!");
            int_player_attack_power = 5;
            int_enemy_attack_power = 10;
        } else if int_player_type == int_enemy_type {
            println!("You and your enemy are the same type.");
            int_player_attack_power = 7;
            int_enemy_attack_power = 7;
        } else if int_enemy_type == 4 {
            println!("Enemy is closest to a Gamer!");
            int_player_attack_power = 7;
            int_enemy_attack_power = 7;
        } else if int_player_type == 4 {
            println!("You are closest to a Gamer!");
            int_player_attack_power = 7;
            int_enemy_attack_power = 7;
        }

        // Print modded enemy name if available
        if let Some(enemy_func) = &enemy_name {
            let result: String = enemy_func
                .call(int_enemy_type)
                .expect("EnemyName function failed");
            println!("Enemy mod name: {}", result);
        }

        // --- Battle loop ---
        loop {
            let mut str_input = String::new();
            println!("What would you like to do, attack(1) or increase attack power(2):");
            io::stdin().read_line(&mut str_input).expect("Failed to read input");

            let int_input: u8 = match str_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Not a number, Goodbye");
                    process::exit(0);
                }
            };

            if int_input == 1 {
                if int_show_description == 2 {
                    thread::sleep(Duration::from_secs(3));
                    println!("Player jumps in the air.");
                    thread::sleep(Duration::from_secs(3));
                    println!("Player approaches enemy.");
                    thread::sleep(Duration::from_secs(3));
                    println!("Player attacks enemy!");
                    thread::sleep(Duration::from_secs(3));
                }
                int_enemy_damage += int_player_attack_power;
                int_enemy_health = int_max_health as i8 - int_enemy_damage as i8;
            } else if int_input == 2 {
                int_player_extra_attack_power += 5;
                int_player_attack_power += 5;
                println!("You increased your attack power!");
            } else {
                println!("Not an option, Goodbye");
                process::exit(0);
            }

            // enemy turn
            if int_enemy_action == 0 {
                if int_show_description == 2 {
                    thread::sleep(Duration::from_secs(3));
                    println!("Enemy jumps in the air.");
                    thread::sleep(Duration::from_secs(3));
                    println!("Enemy approaches player.");
                    thread::sleep(Duration::from_secs(3));
                    println!("Enemy attacks you!");
                    thread::sleep(Duration::from_secs(3));
                }
                println!("Enemy attacks player");
                int_player_damage += int_enemy_attack_power;
                int_player_health = int_max_health as i8 - int_player_damage as i8;
            } else {
                int_enemy_extra_attack_power += 5;
                int_enemy_attack_power += 5;
                println!("Enemy powers up!");
            }

            // check for win/lose
            if int_player_health <= 0 {
                println!("Enemy wins, Game Over.");
                process::exit(0);
            }
            if int_enemy_health <= 0 {
                println!("Player wins, You go on to next round.");
                break;
            }
        }
    }
}

