// src/main.rs
use std::path::PathBuf;

use clap::{Parser,Subcommand};
use anyhow::Result;


use quizzer::quiz::question::Question;
use quizzer::quiz::quiz_manager::QuizManager;

fn main() -> Result<()> {
    let matches = App::new("Quiz Game")
        .subcommand(App::new("enter")
            .about("Enter quiz questions")
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets the output JSON file")
                .takes_value(true)
                .required(true)))
        .subcommand(App::new("play")
            .about("Play the quiz"))
        .get_matches();

    match matches.subcommand() {
        ("enter", Some(enter_matches)) => {
            let output_file = enter_matches.value_of("output").unwrap();
            // Logic for entering quiz questions
            // Use QuizManager::save_questions to save questions to the file
            println!("Entered questions saved to {}", output_file);
        }
        ("play", Some(_play_matches)) => {
            // Logic for playing the quiz
            // Use QuizManager::load_questions to load questions from the file
            println!("Quiz mode activated");
        }
        _ => println!("Invalid command"),
    }

    Ok(())
}
