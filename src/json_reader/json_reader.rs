use std::{
    fs::File,
    io::{self, Read},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    agent::{agent::Agent, agent_log::AgentLog},
    bet::{bet::Bet, bet_log::BetLog, bet_state::BetState, bet_value::BetValue},
    roulette::{game_configs::GameConfig, roulette_type::RouletteType},
};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Deserialize, Serialize)]
pub struct JsonReader {}

impl JsonReader {
    pub fn read_game_json(game_config_path: &str) -> Result<GameConfig, io::Error> {
        let mut file = File::open(game_config_path)?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        let mut game_config: GameConfig = serde_json::from_str(&contents)?;
        if game_config.roulette_type.is_none() {
            game_config.roulette_type = Some(RouletteType::European);
        }
        return Ok(game_config);
    }

    pub fn read_agents_json(agents_path: &str) -> Result<Vec<Agent>, io::Error> {
        let mut file = File::open(agents_path)?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        let agent_entries: Vec<Value> = serde_json::from_str(&contents)?;
        let mut agents: Vec<Agent> = Vec::new();
        let mut agent_number: i32 = 1;

        for agent_entry in agent_entries {
            let agent_logs: Vec<AgentLog> = Vec::new();
            let balance_cents: i64 = agent_entry["balance_cents"].as_i64().unwrap();
            let strategic_bets_data: &Vec<Value> =
                agent_entry["strategic_bets"].as_array().unwrap();
            let mut strategic_bets: Vec<Bet> = Vec::new();
            let name_entry: Option<&str> = agent_entry["name"].as_str();
            let name: String;
            if name_entry.is_some() {
                name = name_entry.unwrap().to_string();
            } else {
                name = format!("Agent {}", agent_number);
            }

            for strategic_bet_data in strategic_bets_data {
                let bet_value: BetValue =
                    strategic_bet_data["bet_value"].clone().try_into().unwrap();
                let amount_cents: i64 = strategic_bet_data["amount_cents"].as_i64().unwrap();
                let initial_amount_cents: i64 = amount_cents;
                let progression_factor: i64 =
                    strategic_bet_data["progression_factor"].as_i64().unwrap();
                let bet_state: BetState = BetState::Active;
                let bet_logs: Vec<BetLog> = Vec::new();
                strategic_bets.push(Bet {
                    bet_state,
                    bet_value,
                    amount_cents,
                    initial_amount_cents,
                    progression_factor,
                    bet_logs,
                });
            }
            agents.push(Agent {
                balance_cents,
                strategic_bets,
                name,
                agent_logs,
            });
            agent_number += 1;
        }
        return Ok(agents);
    }
}
