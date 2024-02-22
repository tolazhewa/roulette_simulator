use std::time::Instant;

use crate::agent::agent::Agent;
use crate::error::Error;
use crate::roulette::{game_configs::GameConfig, roulette_game::RouletteGame};
use tracing::{error, info};

pub async fn run(game_config: GameConfig, agents: Vec<Agent>) -> Result<Vec<RouletteGame>, Error> {
    let start = Instant::now();
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
    let duration = start.elapsed();
    info!("---------------------------");
    info!("Time elapsed: {:?}", duration);
    info!("---------------------------");
    return Ok(results);
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
