#![allow(non_snake_case)]

use {rand::Rng, std::io::Write, std::process::exit};

fn main() {
    println!("Welcome to RPS! Type 'q' or 'Q' to quit.");

    let mut wins: u64 = 0;
    let mut loses: u64 = 0;
    let mut draws: u64 = 0;

    loop {
        println!("make your move");
        std::io::stdout().flush().unwrap();
        let mut player = String::new();
        std::io::stdin().read_line(&mut player).unwrap();
        player = player.trim().to_lowercase();

        if player != "rock" && player != "paper" && player != "scissors" {
            if player == "q" {
                break;
            }
            if player.chars().count() >= 10 || player.is_empty() {
                println!("[bad string] is not a valid option!");
            } else {
                println!("'{player}' is not a valid option!");
            }
            println!("");
            continue;
        }

        let rand_number = rand::thread_rng().gen_range(1..3);
        let computer;

        if rand_number == 0 {
            computer = "rock";
        } else if rand_number == 1 {
            computer = "paper";
        } else {
            computer = "scissors";
        }

        print!("You chose {player}, and computer chose {computer}. ");

        if player == "rock" {
            if computer == "rock" {
                // rock:rock
                println!("It's a draw!");
                std::io::stdout().flush().unwrap();
                draws += 1;
            } else if computer == "paper" {
                // rock:paper
                println!("You've lost!");
                std::io::stdout().flush().unwrap();
                loses += 1;
            } else {
                // rock:scissors
                println!("You've won!");
                std::io::stdout().flush().unwrap();
                wins += 1;
            }
        } else if player == "paper" {
            if computer == "rock" {
                // paper:rock
                println!("You've won!");
                std::io::stdout().flush().unwrap();
                wins += 1;
            } else if computer == "paper" {
                // paper:paper
                println!("It's a draw!");
                std::io::stdout().flush().unwrap();
                draws += 1;
            } else {
                // paper:scissors
                println!("You've lost!");
                std::io::stdout().flush().unwrap();
                loses += 1;
            }
        } else {
            if computer == "rock" {
                // scissors:rock
                println!("You've lost!");
                std::io::stdout().flush().unwrap();
                loses += 1;
            } else if computer == "paper" {
                // scissors:paper
                println!("You've won!");
                std::io::stdout().flush().unwrap();
                wins += 1;
            } else {
                // scissors:scissors
                println!("It's a draw!");
                std::io::stdout().flush().unwrap();
                draws += 1
            }
        }
        println!(""); // Insert a newline
    }
    println!("You scored ");
    println!("{wins} wins, ");
    println!("{loses} loses, ");
    println!("{draws} draws, ");
    std::io::stdout().flush().unwrap();

    println!("\nThanks for playing!");
    print!("Press any key to continue...");

    std::io::stdout().flush().unwrap();
    let mut temp: String = String::new();
    std::io::stdin().read_line(&mut temp).unwrap();
    drop(temp);

    exit(0);
}
