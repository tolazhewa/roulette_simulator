mod agent;
mod bet;
mod board;
mod error;
mod json;
mod roulette;
mod types;

use std::{thread, time::Instant};

use error::Error;

use crate::agent::agent::Agent;
use crate::json::json_reader::JsonReader;
use crate::roulette::stats::Stats;
use crate::roulette::{game_configs::GameConfig, roulette_game::RouletteGame};

const GAME_CONFIG_FILENAME: &str = "./res/game.json";
const AGENTS_FILENAME: &str = "./res/agents.json";
const THREAD_BATCH_SIZE: i32 = 100;

#[derive(Debug)]
struct BatchParams {
    batch_number: i32,
    thread_batch_size: i32,
    agents: Vec<Agent>,
    number_of_rounds: i32,
    allow_negative_balance: bool,
}

fn main() {
    let start = Instant::now();
    let (mut game_config, mut agents) = get_json();
    let results = run(&mut game_config, &mut agents).unwrap_or_else(|error| {
        panic!(
            "Failed to run game:\n{:?}\n{:?}\n{}",
            game_config, agents, error
        )
    });
    let stats: Stats = Stats::from_games(&results);
    println!("{}", stats);
    let duration = start.elapsed();
    println!("---------------------------");
    println!("Time elapsed: {:?}", duration);
    println!("---------------------------");
}

fn get_json() -> (GameConfig, Vec<Agent>) {
    let game_config = JsonReader::read_game_json(GAME_CONFIG_FILENAME).unwrap_or_else(|error| {
        panic!(
            "Failed to read game config from {}\n{}",
            GAME_CONFIG_FILENAME, error
        )
    });
    let agents = JsonReader::read_agents_json(AGENTS_FILENAME).unwrap_or_else(|error| {
        panic!("Failed to read agents from {}\n{}", AGENTS_FILENAME, error)
    });
    return (game_config, agents);
}

fn run(game_config: &mut GameConfig, agents: &mut Vec<Agent>) -> Result<Vec<RouletteGame>, Error> {
    let mut game_results: Vec<RouletteGame> = Vec::new();
    let thread_batch_size = if THREAD_BATCH_SIZE < game_config.number_of_games {
        THREAD_BATCH_SIZE
    } else {
        game_config.number_of_games
    };
    let number_of_batches = game_config.number_of_games / thread_batch_size;
    let mut remaining_games = game_config.number_of_games % thread_batch_size;
    for batch_number in 0..number_of_batches {
        let batch_results = run_batch(BatchParams {
            batch_number,
            thread_batch_size,
            agents: agents.clone(),
            number_of_rounds: game_config.number_of_rounds,
            allow_negative_balance: game_config.allow_negative_balance,
        });
        game_results.extend(batch_results.clone());
        remaining_games += thread_batch_size - batch_results.len() as i32;
    }
    game_results.extend(run_batch(BatchParams {
        batch_number: number_of_batches,
        thread_batch_size: remaining_games,
        agents: agents.clone(),
        number_of_rounds: game_config.number_of_rounds,
        allow_negative_balance: game_config.allow_negative_balance,
    }));
    return Ok(game_results);
}

fn run_batch(params: BatchParams) -> Vec<RouletteGame> {
    let mut results = Vec::new();
    let BatchParams {
        batch_number,
        thread_batch_size,
        ref agents,
        number_of_rounds,
        allow_negative_balance,
    } = params;
    let mut handles = Vec::new();
    for thread_number in 1..=thread_batch_size {
        let game_agents: Vec<Agent> = agents.clone();
        let number_of_rounds = number_of_rounds;
        let allow_negative_balance = allow_negative_balance;
        let game_number = (batch_number * thread_batch_size) + thread_number;

        let handle: thread::JoinHandle<Result<RouletteGame, Error>> = thread::spawn(move || {
            let mut game: RouletteGame = RouletteGame::new(
                game_number,
                game_agents,
                number_of_rounds,
                allow_negative_balance,
                None,
            )?;
            game.play()?;
            return Ok(game);
        });
        handles.push(handle);
    }
    for handle in handles.into_iter() {
        let thread_res = handle.join().map_err(|e| Error::ThreadJoinError {
            message: format!("Failed to join thread:\n {:?}", e),
            nested_error: Some(e),
        });

        match thread_res {
            Ok(game_res) => match game_res {
                Ok(game) => results.push(game),
                Err(e) => {
                    println!("Failed to run game: {:?}\n{}", params, e);
                }
            },
            Err(e) => {
                println!("Failed to join thread: {:?}\n{}", params, e);
            }
        }
    }
    return results;
}
