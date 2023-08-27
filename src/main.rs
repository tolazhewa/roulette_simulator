mod agent;
mod bet;
mod board;
mod json_reader;
mod roulette;
mod types;

use std::{thread, time::Instant};

use crate::agent::agent::Agent;
use crate::json_reader::json_reader::JsonReader;
use crate::roulette::stats::Stats;
use crate::roulette::{game_configs::GameConfig, roulette_game::RouletteGame};

const GAME_CONFIG_FILENAME: &str = "./res/game.json";
const AGENTS_FILENAME: &str = "./res/agents.json";
const NUMBER_OF_THREADS: i32 = 100;

fn main() {
    let start = Instant::now();
    let (mut game_config, mut agents) = get_json();
    let results: Vec<RouletteGame> = run(&mut game_config, &mut agents);
    let stats: Stats = Stats::from_games(&results);
    println!("{}", stats);
    let duration = start.elapsed();
    println!("---------------------------");
    println!("Time elapsed: {:?}", duration);
    println!("---------------------------");
}

fn get_json() -> (GameConfig, Vec<Agent>) {
    let game_config = JsonReader::read_game_json(GAME_CONFIG_FILENAME).unwrap();
    let agents = JsonReader::read_agents_json(AGENTS_FILENAME).unwrap();
    return (game_config, agents);
}

fn run(game_config: &mut GameConfig, agents: &mut Vec<Agent>) -> Vec<RouletteGame> {
    let mut game_results: Vec<RouletteGame> = Vec::new();
    let threads_at_a_time = if NUMBER_OF_THREADS < game_config.number_of_games {
        NUMBER_OF_THREADS
    } else {
        game_config.number_of_games
    };
    for threads_number in 0..(game_config.number_of_games / threads_at_a_time) {
        let mut handles: Vec<thread::JoinHandle<RouletteGame>> = Vec::new();
        for game_number in 1..=threads_at_a_time {
            let game_agents: Vec<Agent> = agents.clone();
            let number_of_rounds = game_config.number_of_rounds;
            let allow_negative_balance = game_config.allow_negative_balance;

            let handle: thread::JoinHandle<RouletteGame> = thread::spawn(move || {
                let mut game: RouletteGame = RouletteGame::new(
                    game_number + (threads_number * threads_at_a_time),
                    game_agents,
                    number_of_rounds,
                    allow_negative_balance,
                    None,
                );
                game.play();
                return game;
            });
            handles.push(handle);
        }
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .for_each(|game| game_results.push(game));
    }
    return game_results;
}
