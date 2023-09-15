mod agent;
mod bet;
mod board;
mod error;
mod json;
mod roulette;
mod types;

use std::time::Instant;
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

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (file_guard, json_file_guard) = set_up_logging();

    run().await;

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
async fn run() {
    let (game_config, agents) = get_json();
    let mut results: Vec<RouletteGame> = Vec::new();

    run_games_async(game_config.clone(), agents.clone())
        .await
        .unwrap_or_else(|error| {
            panic!(
                "Failed to run games:\n{:?}\n{:?}\n{}",
                game_config, agents, error
            )
        })
        .iter()
        .for_each(|result| match result {
            Ok(r) => results.push(r.clone()),
            Err(e) => error!("Failed to run game: {:?}", e),
        });
    info!(
        "{} games failed to run",
        game_config.number_of_games - results.len() as i32
    );
    let stats: Stats = Stats::from_games(&results);
    println!("{}", stats);
}

async fn run_games_async(
    game_config: GameConfig,
    agents: Vec<Agent>,
) -> Result<Vec<Result<RouletteGame, Error>>, Error> {
    let mut handles = Vec::new();

    for game_number in 1..=game_config.number_of_games {
        let game_agents = agents.clone();
        let number_of_rounds = game_config.number_of_rounds;
        let allow_negative_balance = game_config.allow_negative_balance;

        let handle = tokio::spawn(async move {
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
    let results = futures::future::try_join_all(handles)
        .await
        .map_err(|e| Error::JoinError {
            message: format!("Failed to join thread:\n {:?}", e),
            nested_error: Some(Box::new(e)),
        });
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
