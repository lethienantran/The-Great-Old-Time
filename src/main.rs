// Import neccessary modules 
use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use rand::Rng;
use serde::{Deserialize, Serialize};
use rand::prelude::SliceRandom;
use std::thread::sleep;
use std::time::Duration;

// Declare Boss Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Boss {
    pub name: String,
    pub health: u32,
    pub attack: u32,
    pub defense: u32,
    pub level: u32,
}

// Declare Player Struct 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub max_health: u32,
    pub health: u32, 
    pub attack: u32, 
    pub defense: u32,
    pub gold: u32,
    pub level: u32,
    pub current_exp: u32,
    pub current_lv_exp: u32,
    pub boss_defeated: Vec<String>,
}

fn main() {
    // Display Welcome Message and Options 
    println!("--------------------------------");
    println!("The Great Old Time"); 
    println!("--------------------------------");
    println!("Option 1: Start Your New Journey");
    println!("Option 2: Continue Your Journey");
    println!("Option 3: How To Play?");
    println!("Option 4: Exit");
    println!("--------------------------------");

    // Read user input 
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("ERROR: Failed to read line.");

    // Parse user input as u32 
    let user_choice: u32 = user_input.trim().parse().expect("Invalid option. Please choose a valid option.");

    // Match user_choice case 
    match user_choice {
        1 => new_game(),
        2 => continue_game(),
        3 => instructions(),
        4 => {
            println!("Logging out...");
            std::process::exit(0);
        }
        _ => println!("Invalid option. Please choose a valid option."),
    }
}

// Function: display_game_intro
// Description: Presents the immersive narrative and background story of the game to the player.
//              Sets the stage for the player's journey in the mystical land of Eldoria, introducing the overarching conflict and the role of the chosen hero.
//              Utilizes the sleep function to introduce each paragraph with a slight delay, enhancing the storytelling experience.
// Parameters: None
// Returns: None
fn display_game_intro() {
    println!("In the mystical land of Eldoria, where ancient magic and mythical creatures coexist, a lone adventurer sets forth on a journey through time and challenges.");
    sleep(Duration::from_millis(200));
    println!("You, a courageous soul seeking glory and adventure, find yourself standing at the crossroads of destiny.");
    sleep(Duration::from_millis(200));
    println!("Eldoria is in turmoil, plagued by the looming presence of malevolent beings known as the Great Old Ones.");
    sleep(Duration::from_millis(200));
    println!("These monstrous entities, born from the depths of ancient darkness, threaten to plunge the world into eternal chaos.");
    sleep(Duration::from_millis(200));
    println!("As the chosen hero, it is your destiny to rise against the shadows and restore peace to Eldoria.");
    sleep(Duration::from_millis(200));
    println!("Your journey begins in the quaint village of Arcadia, a haven for the common folk who dream of a better future.");
    sleep(Duration::from_millis(200));
    println!("The air is thick with a sense of foreboding, and whispers of an impending darkness echo through the cobblestone streets.");
    sleep(Duration::from_millis(200));
    println!("The village elder, a wise old sage, reveals that only a true hero can confront the Great Old Ones and save Eldoria from impending doom.");
    sleep(Duration::from_millis(200));
    println!("Armed with a basic set of equipment and your wits, you embark on an epic quest filled with perilous encounters, mysterious dungeons, and formidable bosses.");
    sleep(Duration::from_millis(200));
    println!("The path to greatness is not without challenges. Eldoria is teeming with dangerous creatures, cunning foes, and enigmatic puzzles that guard the secrets of the Great Old Ones.");
    sleep(Duration::from_millis(200));
    println!("As you traverse the lands, you will encounter peculiar characters, some offering aid and others harboring secrets that could alter the course of your quest.");
    sleep(Duration::from_millis(200));
    println!("Your journey unfolds in a world of magic and mystery, where every decision you make, every battle you fight, and every interaction shapes the narrative.");
    sleep(Duration::from_millis(200));
    println!("Will you rise to become the legendary hero Eldoria needs, or will the darkness claim victory?");
    sleep(Duration::from_millis(200));
    println!("Prepare yourself, adventurer, for \"The Great Old Time\" awaits – a journey through time, magic, and destiny where your courage will be tested, and your choices will echo across the ages.");
    sleep(Duration::from_millis(200));
    println!("The fate of Eldoria rests in your hands.");
    sleep(Duration::from_millis(200)); 
    println!("--------------------------------");
}

// Function: save_player_data
// Description: Serializes the provided Player struct into a JSON-formatted string and writes it to the "playerStats.json" file.
// Parameters:
//    - player: A reference to the Player struct containing the player's data to be saved.
// Returns: None
fn save_player_data(player: &Player) {
    let current_player = serde_json::to_string_pretty(player).expect("Failed to serialize player data");
    // Write serialized player data to file
    match File::create("./src/playerStats.json") {
        Ok(mut file) => {
            file.write_all(current_player.as_bytes()).expect("Failed to write playerStats.json");
        }
        Err(err) => {
            eprintln!("Failed to create playerStats.json file: {}", err);
        }
    }
}

// Function: save_boss_data
// Description: Serializes the provided Boss struct into a JSON-formatted string and writes it to the "bossStats.json" file.
// Parameters:
//    - boss: A reference to the Boss struct containing the boss's data to be saved.
// Returns: None
fn save_boss_data(boss: &Boss) {
    let current_boss = serde_json::to_string_pretty(boss).expect("Failed to serialize boss data");
    
    // Write serialized boss data to file
    match File::create("./src/bossStats.json") {
        Ok(mut file) => {
            file.write_all(current_boss.as_bytes()).expect("Failed to write bossStats.json");
        }
        Err(err) => {
            eprintln!("Failed to create playerStats.json file: {}", err);
        }
    }
}


// Function: new_game
// Description: Initiates a new game, displaying the game introduction and prompting the player for their name.
//              Creates a new player profile with default stats at level 1 and a default boss for the initial challenge.
//              Saves the player and boss data to JSON files and continues the game.
// Parameters: None
// Returns: None
fn new_game() {
    // Display the story of the game
    display_game_intro();
    
    //Prompt for asking player's name
    println!("May we ask \"What is your name, adventurer?\"...");
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name).expect("Failed to read line");

    // Create Player Profile with default stats at level 1
    let new_player = Player {
        name: player_name.trim().to_string(),
        max_health: 100,
        health: 100,
        attack: 6,
        defense: 10,
        gold: 0,
        level: 1,
        current_exp: 0,
        current_lv_exp: 1,
        boss_defeated: Vec::new(),
    };

    // Create Default Boss when user create as a new player
    let new_boss = Boss {
        name: "The Crazy Guy".to_string(),
        health: 5, 
        attack: 11, 
        defense: 20,
        level: 1, 
    };

    // Save boss and player data to json file.
    save_player_data(&new_player);
    save_boss_data(&new_boss);
    continue_game();
}

// Function: continue_game
// Description: Attempts to open the playerStats.json file to read the player's data. 
//              Displays the home menu with options based on the player's progress.
//              Handles user input and directs the flow of the game accordingly.
// Parameters: None
// Returns: None
fn continue_game() {
    // Try to open the playerStats.json file to read data
    let file_result = File::open("./src/playerStats.json");

    match file_result {
        Ok(mut file) => {
            // The file exists, read its contents, and deserializable into Player struct
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to load player's data.");

            // Try to parse data from JSON
            let player: Player = serde_json::from_str(&contents).expect("Failed to load player's data into Player.");
            
            // If successful, display home menu
            println!("--------------------------------");
            println!("The Great Old Time");
            println!("--------------------------------");
            println!("Option 1: Shop & Heal");
            println!("Option 2: Fight Boss & Farm Gold");
            println!("Option 3: View My Stats");
            println!("Option 4: Back To Menu");
            println!("Option 5: Exit");
            println!("--------------------------------");

            // Read user input 
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read your input, please try again.");

            // Parse user_input as u32
            let user_choice: u32 = user_input.trim().parse().expect("Invalid option. Please choose a valid option.");

            // Match user's choice
            match user_choice {
                1 => shop(player.clone()),
                2 => fight_boss_act(player.clone()),
                3 => show_stats(player.clone()),
                4 => main(),
                5 => {
                    println!("Logging out...");
                    std::process::exit(0);
                }
                _ => println!("Invalid option. Please choose a valid option."),
            }
        } 
        Err(_) => {
            println!("We cannot found any record of your current game. Let's start the journey!");
            println!("--------------------------------");
            new_game();
        }
    }
}

fn instructions() {
    println!("--------------------------------");
    println!("The Great Old Time");
    println!("--------------------------------");
    println!("How To Play?");
    sleep(Duration::from_millis(500));
    println!("This is a simple text-based game.");
    sleep(Duration::from_millis(500));
    println!("There are 120 different bosses in the game.");
    sleep(Duration::from_millis(500));
    println!("You will need to find them all and defeat them.");
    sleep(Duration::from_millis(500));
    println!("Notice that, you might need to kill them twice or even more.");
    sleep(Duration::from_millis(500));
    println!("All you need to do is fight with bosses to gain gold and EXP.");
    sleep(Duration::from_millis(500));
    println!("You will need gold to buy buffs: Red Buff, Blue Buff, and Bruiser Buff.");
    sleep(Duration::from_millis(500));
    println!("Red Buff will give you bonus attack.");
    sleep(Duration::from_millis(500));
    println!("Blue Buff will give you bonus defense.");
    sleep(Duration::from_millis(500));
    println!("Bruiser Buff will give you bonus max health.");
    sleep(Duration::from_millis(500));
    println!("You will also need gold to heal yourself after every fight.");
    sleep(Duration::from_millis(500));
    println!("Bosses get stronger after every fight!");
    sleep(Duration::from_millis(500));
    println!("You will need to destroy the boss's shield first, in order to kill them.");
    println!("--------------------------------");
    println!("Option 1: Back to Menu");
    println!("--------------------------------");

    // Read user input 
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read your input, please try again.");

    // Parse user_input as u32
    let user_choice: u32 = user_input.trim().parse().expect("Invalid option. Please choose a valid option.");

    // Match user's choice
    match user_choice {
        1 => main(),
        _ => println!("Invalid option. Please choose a valid option."),
    }
}

// Function: shop
// Description: Displays the shop menu and handles player interactions with the shop.
// Parameters:
//    - player: A mutable reference to the Player struct representing the player's character.
// Returns: None
fn shop(mut player: Player) {
    println!("--------------------------------");
    println!("The Great Old Time");
    println!("--------------------------------");
    println!("Welcome to the shop - Your Gold: ${}", player.gold);
    println!("--------------------------------");
    println!("Option 1: Heal Yourself (${})", (player.level * 2));
    println!("Option 2: Get Stronger With Buff (${})", player.level * 5);
    println!("Option 3: Back");
    println!("--------------------------------");
    
    // Read user input 
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read your input, please try again.");

    // Parse user_input as u32
    let user_choice: u32 = user_input.trim().parse().expect("Invalid option. Please choose a valid option.");

    // Match user's choice
    match user_choice {
        1 => heal_player_health(player.clone()),
        2 => buy_buff(&mut player),
        3 => continue_game(),
        _ => println!("Invalid option. Please choose a valid option."),
    }
}

// Function: buy_buff
// Description: Allows the player to purchase a buff from the in-game shop using their gold.
//              Calculates the cost of the buff based on the player's level and deducts the cost from the player's gold if affordable.
//              Generates a random buff type (attack, defense, or max health) and a random value based on the player's level.
//              Applies the chosen buff effects to the player's stats and provides feedback to the player.
// Parameters:
//    - player: A mutable reference to the Player struct representing the player's character.
// Returns: None
fn buy_buff(player: &mut Player) {
    // Calculate the cost of the buff 
    let buff_cost = player.level * 5;

    // Check if the player has enough gold to buy the buff 
    if player.gold >= buff_cost {
        // Deduct the cost from the player's gold 
        player.gold -= buff_cost;

        // Generate random stat to determine the type of buffs (attack, defense, or health).
        let mut range = rand::thread_rng();
        
        // Let's Specify 1 => Attack, 2 => Defense, 3 => Max Health
        let buff_type = range.gen_range(1..=3);

        // Generate random value based on the player's level
        let buff_value = range.gen_range(1..=player.level + 5);

        // Apply the buff effects based on the random chosen type
        match buff_type {
            1 => {
                player.attack += buff_value;
                println!("--------------------------------");
                println!("You have received a red buff.");
                println!("Red Buff Effects: Attack + {}.\nYour new attack stat: {}.", buff_value, player.attack);
            } 
            2 => {
                player.defense += buff_value;
                println!("--------------------------------");
                println!("You have received a blue buff.");
                println!("Blue Buff Effects: Defense + {}.\nYour new defense stat: {}.", buff_value, player.defense);
            }
            3 => {
                player.max_health += buff_value;
                println!("--------------------------------");
                println!("You have received a bruiser buff.");
                println!("Bruiser Buff Effects: Max Health + {}.\nYour new max health stat: {}.", buff_value, player.max_health);
            }
            _ => {
                // This should not happen, but handle it just in case
                println!("Invalid buff type. Try again.");
            }
        }

        // Display user gold.
        sleep(Duration::from_millis(500));
        println!("--------------------------------");
        println!("Thank you for shopping with us. Here is your receipt.");
        sleep(Duration::from_millis(500));
        println!("--------------------------------");
        println!("Before, you had: ${}.", player.gold + buff_cost);
        sleep(Duration::from_millis(500));
        println!("Item Cost: ${}.", buff_cost);
        sleep(Duration::from_millis(500));
        println!("Remaining Gold: ${}.", player.gold);
        sleep(Duration::from_millis(500));
    } else {
        println!("--------------------------------");
        println!("Oops. You do not have enough gold to buy a buff.");
    }

    // Save player data after update their stat with new buff
    save_player_data(&player);

    // Return to the shop menu
    shop(player.clone());
}

// Function: heal_player_health
// Description: Allows the player to heal their health by 50% in exchange for gold. Checks if the player has enough gold,
//              deducts the cost from their gold, and updates the player's health. Displays healing information, cost,
//              remaining gold, and a receipt. Saves the updated player data.
// Parameters:
//    - player: A mutable reference to the Player struct representing the player's character.
// Returns: None
fn heal_player_health(mut player: Player) {
    // Calculate the cost to heal 50 health
    let cost = player.level * 2; 

    // Check if the player has enough gold to heal 
    if player.gold >= cost {
        // Deduct the cost from the player's gold
        player.gold -= cost;

        // Heal the player's health by 50%
        let healing_amount = player.max_health / 2;
        player.health += healing_amount;

        // Ensure that health is never exceed max health
        if player.health > player.max_health {
            player.health = player.max_health;
        }
        sleep(Duration::from_millis(500));
        println!("--------------------------------");
        println!("You healed for {} health", healing_amount.to_string());
        println!("Your current health: {}.", player.health);
        sleep(Duration::from_millis(500));
        println!("--------------------------------");
        println!("Thank you for shopping with us. Here is your receipt");
        sleep(Duration::from_millis(500));
        println!("--------------------------------");
        println!("Before, you had: ${}.", player.gold + cost);
        sleep(Duration::from_millis(500));
        println!("Item Cost: ${}.", cost);
        sleep(Duration::from_millis(500));
        println!("Remaining Gold: ${}.", player.gold);
        sleep(Duration::from_millis(500));
    } else {
        println!("--------------------------------");
        println!("Oops. You do not have enough gold to heal yourself.");
    }

    // Save player data after update their stat with new buff
    save_player_data(&player);

    // Return to the shop menu
    shop(player.clone());
}

/// # Function: fight_boss_act
///
/// # Description:
/// Initiates a battle with a boss. Reads boss data from the 'bossStats.json' file,
/// displays the story, and prompts the player to fight or escape. Handles the battle
/// sequence, calculating damage, updating health, and providing feedback to the player.
/// Manages critical hits, defends, and calculates gold earned. Checks for player defeat
/// or boss defeat conditions, updates player stats, and generates a new boss if the
/// current boss is defeated.
///
/// # Parameters:
/// - `player`: A mutable reference to the Player struct representing the player's character.
///
/// # Returns:
/// None
fn fight_boss_act(mut player: Player) {
    // Try to open the bossStats.json file 
    let file_result = File::open("./src/bossStats.json");

    match file_result {
        Ok(mut file) => {
            // The file exists, read its contents and deserialize into Boss struct
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to load boss's data.");
        
            // Parsing Boss data
            let mut boss: Boss = serde_json::from_str(&contents).expect("Failed to parse bossStats.json");

            // Display Story
            println!("--------------------------------");
            println!("The Great Old Time");
            println!("------------------");
            println!("Your current level: {}", player.level);
            println!("{} with level {} want to mess up with you.", boss.name, boss.level);
            println!("--------------------------------");
            println!("Option 1: Let's Fight");
            println!("Option 2: Escape From The Forest");
            println!("--------------------------------");

            // Read user input
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Invalid option. Please try again.");

            // Parse user input as u32
            let user_choice: u32 = user_input.trim().parse().expect("Invalid option. Please try again.");
            
            match user_choice { 
                1 => {
                    let mut range = rand::thread_rng();

                    loop {
                        // Calculate damage and update player's health 
                        let boss_damage = boss.attack - player.defense / 2 - player.level * 2; 
                        
                        // Update player.health
                        player.health = player.health.saturating_sub(boss_damage);
                        
                        // Display battle information
                        println!("--------------------------------");
                        sleep(Duration::from_millis(500));
                        println!("{} attacks you for {} damage!", boss.name, boss_damage);
                        sleep(Duration::from_millis(500));
                        println!("Your current health: {}", player.health);
                        sleep(Duration::from_millis(500));
                        println!("{}'s current health: {}", boss.name, boss.health);
                        println!("{}'s current shield: {}", boss.name, boss.defense);
                        save_player_data(&player);

                        // Check if the player is defeated
                        if player.health == 0 {
                            println!("You were defeated by {}!", boss.name);
                            save_player_data(&player);
                            continue_game();
                        }

                        // Prompt user to attack or escape
                        sleep(Duration::from_millis(500));
                        println!("--------------------------------");
                        println!("Option 1: Attack");
                        println!("Option 2: Escape");
                        println!("--------------------------------");

                        // Read user input
                        let mut user_battle_input = String::new();
                        io::stdin().read_line(&mut user_battle_input).expect("Invalid option. Please try again.");

                        // Parse user input as u32
                        let battle_choice: u32 = user_battle_input.trim().parse().expect("Invalid option. Please try again.");

                        match battle_choice {
                            1 => {
                                // Check if the player can do a critical attack 
                                let critical_chance = range.gen_range(1..=10);
                                
                                // 10% to get a critical chance
                                let is_critical = critical_chance == 1;

                                let player_damage = if is_critical {
                                    player.attack * 2
                                } else {
                                    player.attack
                                };

                                if boss.defense > 0 {
                                    boss.defense = boss.defense.saturating_sub(player_damage);

                                    println!("--------------------------------");
                                    sleep(Duration::from_millis(500));
                                    if is_critical {
                                        println!("You dealt a critical damage {:?} to {}!", player_damage, boss.name);
                                    } else {
                                        println!("You dealt {:?} damage to {}!", player_damage, boss.name);
                                    }
                                    sleep(Duration::from_millis(500));
                                    if boss.defense > 0 {
                                        println!("{}'s current defense: {}.", boss.name, boss.defense);
                                    } else {
                                        println!("{}'s shield is destroyed.", boss.name);
                                    }
                                    sleep(Duration::from_millis(500));
                                    println!("Your current health: {}.", player.health);
                                } else {
                                    boss.health = boss.health.saturating_sub(player_damage);
                                    println!("--------------------------------");
                                    sleep(Duration::from_millis(500));
                                    println!("{}'s shield is destroyed.", boss.name);
                                    sleep(Duration::from_millis(500));
                                    if is_critical {
                                        println!("You dealt a critical damage {:?} to {}!", player_damage, boss.name);
                                    } else {
                                        println!("You dealt {:?} damage to {}!", player_damage, boss.name);
                                    }
                                    sleep(Duration::from_millis(500));
                                    println!("{} current health: {}", boss.name, boss.health);
                                    sleep(Duration::from_millis(500));
                                    println!("Your current health: {}", player.health);
                                }

                                // Earn gold based on player's level
                                let gold_earned = range.gen_range(player.level..=player.level*2);
                                player.gold += gold_earned;

                                println!("--------------------------------");
                                sleep(Duration::from_millis(500));
                                println!("{} dropped {} gold for you.", boss.name, gold_earned);

                                // Save boss and player data
                                save_boss_data(&boss); 
                                save_player_data(&player);

                                 // Check if the boss is defeated
                                 if boss.health == 0 {
                                    let exp = range.gen_range(1..=player.level * 2);
                                    println!("--------------------------------");
                                    sleep(Duration::from_millis(500));
                                    println!("Congratulations! You defeated {} and earned {} exp.", boss.name, exp);
                                    if !player.boss_defeated.contains(&boss.name) {
                                        player.boss_defeated.push(boss.name.clone());
                                        sleep(Duration::from_millis(500));
                                        println!("{} is added to your boss defeat collections.", boss.name);
                                        if player.boss_defeated.len() == 120 {
                                            sleep(Duration::from_millis(500));
                                            println!("Congratulations! You have defeated all bosses and emerged victorious!");
                                            sleep(Duration::from_millis(500));
                                            println!("You may keep continue your journey!")
                                        }
                                    }
                                    // Update player's exp
                                    player.current_exp += exp;
                                    if player.current_exp >= player.current_lv_exp {
                                        level_up(player.clone());
                                    } else {
                                        save_player_data(&player);
                                    }

                                    // Generate a new boss for the player
                                    generate_new_boss(&player);
                                }
                            }
                            2 => {
                                println!("You managed to escape from {}.", boss.name);
                                save_player_data(&player);
                                continue_game();
                            }
                            _ => println!("Invalid option. Please choose a valid option."),
                        }
                    }
                },
                2 => continue_game(),
                _ => println!("Invalid option. Please choose a valid option.")
            }
        }
        Err(_) => {
            println!("Failed to load boss data. It may be missing or corrupted.");
        }
    }
}

/// # Function: show_stats
///
/// # Description:
/// Displays the player's statistics and information, including name, level, health, attack,
/// defense, gold, experience, and a list of defeated bosses. Pauses between each stat for better readability.
///
/// # Parameters:
/// - `player`: A Player struct representing the player's character.
///
/// # Returns:
/// None
fn show_stats(player: Player){
    println!("--------------------------------");
    println!("My Stats & Information");
    println!("--------------------------------");
    sleep(Duration::from_millis(200));
    println!("Name: {}", player.name);
    sleep(Duration::from_millis(200));
    println!("Level: {}", player.level);
    sleep(Duration::from_millis(200));
    println!("Health: {}/{}", player.health, player.max_health);
    sleep(Duration::from_millis(200));
    println!("Attack: {}", player.attack);
    sleep(Duration::from_millis(200));
    println!("Defense: {}", player.defense);
    sleep(Duration::from_millis(200));
    println!("Gold: ${}", player.gold);
    sleep(Duration::from_millis(200));
    println!("Exp: {}/{}", player.current_exp, player.current_lv_exp);
    sleep(Duration::from_millis(200));
    if !player.boss_defeated.is_empty() {
        println!("Defeated Bosses:");
        for boss_name in &player.boss_defeated {
            sleep(Duration::from_millis(200));
            println!("  - {}", boss_name);
        }
    } else {
        println!("You haven't defeated any bosses yet.");
    }
    sleep(Duration::from_millis(200));
    continue_game();
}

/// # Function: level_up
///
/// # Description:
/// Increases the player's level, updates their stats (health, attack, defense) accordingly,
/// and saves the new player data. Displays a congratulatory message to the player.
///
/// # Parameters:
/// - `player`: A Player struct representing the player's character before leveling up.
///
/// # Returns:
/// None
fn level_up(player: Player) {
    // Calculate remaining experience after leveling up
    let mut current_exp = 0;
    if player.current_exp != player.current_lv_exp {
        current_exp = player.current_exp - player.current_lv_exp;
    }

    // Create a new Player struct with updated stats
    let new_data = Player {
        name: player.name,
        max_health: player.max_health + player.level * 10,
        health: player.max_health + player.level * 10,
        attack: player.attack + player.level * 2,
        defense: player.defense + player.level * 2,
        level: player.level + 1,
        current_exp: current_exp,
        current_lv_exp: player.current_lv_exp + player.current_lv_exp,
        gold: player.gold,
        boss_defeated: player.boss_defeated
    };

    // Save the updated player data
    save_player_data(&new_data);
    // Display a congratulatory message to the player
    println!("--------------------------------");
    sleep(Duration::from_millis(500));
    println!("Congratulations! You have reached to level {}.", new_data.level);
}

/// # Function: generate_new_boss
///
/// # Description:
/// Generates a new boss with random attributes and a random name from a predefined list.
/// Saves the new boss data and notifies the player about the appearance of the new boss.
///
/// # Parameters:
/// - `player`: A reference to the Player struct representing the player's character.
///
/// # Returns:
/// None
fn generate_new_boss(player: &Player) {
    let mut range = rand::thread_rng();

    // List of possible boss names
    let boss_names = vec![
        "Riftstalker",
        "Spectral Overlord",
        "Dreadfiend",
        "Chromatic Wyrm",
        "Darkmoon Despoiler",
        "Twilight Devastator",
        "Ebonheart Executioner",
        "Mystic Voidwalker",
        "Abyssal Enigma",
        "Crimson Doombringer",
        "Harbinger of Chaos",
        "Shadowflame Phoenix",
        "Frostbite Harvester",
        "Venomous Arachnid",
        "Hellfire Incarnate",
        "Ethereal Nightmare",
        "Necrotic Plagueweaver",
        "Starfall Serpent",
        "Abominable Annihilator",
        "Celestial Obliterator",
        "Doomstorm Conjurer",
        "Malevolent Eclipse",
        "Abyssal Warden",
        "Netherfire Elemental",
        "Blazebringer",
        "Soul Devourer",
        "Noxious Serpent",
        "Eldritch Behemoth",
        "Chaos Conductor",
        "Frostshroud Shadow",
        "Voidheart Magus",
        "Infernal Voidlord",
        "Astral Reckoner",
        "Eclipsed Phantom",
        "Mystic Mindbender",
        "Harmonic Desolation",
        "Thunderous Tyrant",
        "Frostspire Guardian",
        "Cursed Revenant",
        "Typhoon Tempest",
        "Umbral Dominator",
        "Abyssal Arbiter",
        "Doomhammer Titan",
        "Necroshade Lord",
        "Plaguebringer Scourge",
        "Vortex Shifter",
        "Ephemeral Doomcaster",
        "Cryptic Voidstalker",
        "Perdition Ravager",
        "Vortex Sovereign",
        "Astral Vanquisher",
        "Oblivion Harvester",
        "Phantom Monstrosity",
        "Diabolic Obliterator",
        "Eternal Nightshade",
        "Umbral Demolisher",
        "Abominable Overlord",
        "Infernal Malevolence",
        "Mystical Aberration",
        "Doombringer Ascendant",
        "Abyssal Deity",
        "Void Eclipse",
        "Shadow Abysswalker",
        "Dreadlord Xyron",
        "Azure Seraphim",
        "Ignis the Inferno",
        "Lunar Eclipse",
        "Obsidian Gargoyle",
        "Abyssal Dreadnought",
        "Crimson Banshee",
        "Eclipse Devourer",
        "Netherfiend Marauder",
        "Venomous Vindicator",
        "Ethereal Emberfiend",
        "Spectral Scourge",
        "Thunderfury",
        "Soulreaper Xal'thanar",
        "Frostbitten Leviathan",
        "Abyssal Warlock",
        "Doomsday Diviner",
        "Harmonic Horror",
        "Celestial Inferno",
        "Shadowblade Sentinel",
        "Vortexus Maximus",
        "Blizzard Behemoth",
        "Chaos Crawler",
        "Eternal Ember",
        "Netherstorm Nexus",
        "Mystic Maelstrom",
        "Plaguebearer Phantasm",
        "Voidheart Vanguard",
        "Ragnarok Reaper",
        "Doomsayer Dreadlord",
        "Bane of Shadows",
        "Typhoon Tempest",
        "Dreadful Drake",
        "Frostfall Fury",
        "Eldritch Essence",
        "Abyssal Aberration",
        "Crimson Conqueror",
        "Diabolical Dementia",
        "Twilight Tyrant",
        "Oblivion Outlaw",
        "Ephemeral Enchantress",
        "Vengeful Vortex",
        "Malevolent Monolith",
        "Nocturnal Nightshade",
        "Cursed Cataclysm",
        "Perilous Phantasm",
        "Umbral Uprising",
        "Astral Abomination",
        "Void Vanguard",
        "Lich Lord of Doom",
        "Reaper of Ruin",
        "Infernal Invader",
        "Eternal Exile",
        "Mystical Menace",
        "Abyssal Assassin",
        "Shade of Shadows",
        "Soulless Serpent",
        "Dreaded Drifter",
        "Darkened Dragon",
        "Eclipsed Enigma",
        "Vortex Vanisher",
        "Phantom Fury",
        "Diabolic Desolation",
        "Ephemeral Eviscerator",
        "Netherworld Nihilist",
        "Abyssal Alchemist",
        "Eternal Executioner",
        "Crimson Conjurer",
        "Blazing Behemoth",
        "Harmonic Harbinger",
        "Chaos Chimera",
        "Dreadful Deviant",
        "Frostfall Fiend",
        "Umbral Undertaker",
        "Vengeful Void",
        "Nocturnal Nightmare",
        "Infernal Incinerator",
        "Mystic Malefactor",
        "Oblivion Overlord",
        "Typhoon Terror",
        "Astral Anomaly",
        "Doombringer Daemon",
        "Diabolical Demigod",
        "Ephemeral Executioner",
        "Infernal Invoker",
        "Celestial Cataclysm",
        "Harmonic Herald",
        "Crimson Cryptkeeper",
        "Abyssal Adversary",
        "Soulreaver Specter",
        "Necrotic Necromancer",
        "Vengeful Vindicator",
        "Eternal Enchanter",
        "Twilight Tormentor",
        "Void Voyager",
    ];

    // Select a random name from the list
    let random_name = boss_names.choose(&mut range).expect("Failed to choose a random name").to_string();

    // Define the range for boss attributes based on your game's design
    // TODO: Boss Stats may need to adjust for more balance.
    let boss_attack = player.defense + 5;
    let boss_health = player.level * 2 + player.health / 2;
    let boss_defense = player.attack * range.gen_range(2..=10); 

    // Create a new Boss struct with random attributes
    let new_boss = Boss {
        name: random_name.clone(),
        health: range.gen_range(boss_health * 5..=boss_health * 10),
        attack: range.gen_range(boss_attack..=boss_attack + 10), 
        defense: range.gen_range(boss_defense-player.attack..=boss_defense + player.attack),
        level: range.gen_range(player.level.saturating_sub(2)..=player.level + 2), 
    };

    // Save the new boss data
    save_boss_data(&new_boss);
    // Notify the player about the appearance of the new boss
    println!("--------------------------------");
    sleep(Duration::from_millis(500));
    println!("A new boss named {} has appeared!", random_name);
    sleep(Duration::from_millis(500));
    println!("Prepare yourself for the next challenge!");
    sleep(Duration::from_millis(500));
    println!("Back to village for healing...");
    sleep(Duration::from_millis(500));
    //Reset to game menu to start with updated player profile
    continue_game();
}