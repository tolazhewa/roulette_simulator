mod agent;
mod bet;
mod board;
mod error;
mod json;
mod roulette;
mod types;

use std::{thread, time::Instant};
use tracing::{error, info};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, registry::Registry};

use crate::agent::agent::Agent;
use crate::json::json_reader::JsonReader;
use crate::roulette::stats::Stats;
use crate::roulette::{game_configs::GameConfig, roulette_game::RouletteGame};
use error::Error;

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
    let (file_guard, json_file_guard) = set_up_logging();
    let (game_config, agents) = get_json();
    let results = run(&game_config, &agents).unwrap_or_else(|error| {
        panic!(
            "Failed to run game:\n{:?}\n{:?}\n{}",
            game_config, agents, error
        )
    });
    let stats: Stats = Stats::from_games(&results);
    println!("{}", stats);
    let duration = start.elapsed();
    info!("---------------------------");
    info!("Time elapsed: {:?}", duration);
    info!("---------------------------");
    drop(file_guard);
    drop(json_file_guard);
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

fn run(game_config: &GameConfig, agents: &Vec<Agent>) -> Result<Vec<RouletteGame>, Error> {
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
                    error!("Failed to run game: {:?}\n{}", params, e);
                }
            },
            Err(e) => {
                error!("Failed to join thread: {:?}\n{}", params, e);
            }
        }
    }
    return results;
}

fn set_up_logging() -> (WorkerGuard, WorkerGuard) {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "./logs", "app.log");
    let json_file_appender = RollingFileAppender::new(Rotation::DAILY, "./logs", "app.json");
    let (non_blocking_file, file_guard) = tracing_appender::non_blocking(file_appender);
    let (non_blocking_json_file, json_file_guard) =
        tracing_appender::non_blocking(json_file_appender);

    let stdout_layer = fmt::layer().with_writer(|| std::io::stdout());
    let file_layer = fmt::layer().with_writer(move || non_blocking_file.clone());
    let json_file_layer = fmt::layer()
        .json()
        .with_writer(move || non_blocking_json_file.clone());

    let subscriber = Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .with(json_file_layer)
        .with(LevelFilter::TRACE);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Unable to set global tracing subscriber");

    return (file_guard, json_file_guard);
}
