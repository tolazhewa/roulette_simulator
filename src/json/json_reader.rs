use crate::error::Error;
use std::{fs::File, io::Read};

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
    pub fn read_game_json(game_config_path: &str) -> Result<GameConfig, Error> {
        let mut file = File::open(game_config_path).map_err(|e| Error::IOError(e))?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        let mut game_config: GameConfig =
            serde_json::from_str(&contents).map_err(|e| Error::DeserializatonError {
                message: format!("Failed to deserialize game config json file: {}", e),
                de_str: Some(contents.clone()),
                value: None,
                nested_error: Some(Box::new(e)),
            })?;
        if game_config.roulette_type.is_none() {
            game_config.roulette_type = Some(RouletteType::European);
        }
        return Ok(game_config);
    }

    pub fn read_agents_json(agents_path: &str) -> Result<Vec<Agent>, Error> {
        let mut file = File::open(agents_path).map_err(|e| Error::IOError(e))?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        let agent_entries: Vec<Value> =
            serde_json::from_str(&contents).map_err(|e| Error::DeserializatonError {
                message: format!("Failed to deserialize agents json file: {}", e),
                de_str: Some(contents.clone()),
                value: None,
                nested_error: Some(Box::new(e)),
            })?;
        let mut agents: Vec<Agent> = Vec::new();
        let mut agent_number: i32 = 1;

        for agent_entry in agent_entries {
            let agent_logs: Vec<AgentLog> = Vec::new();
            let balance_cents: i64 =
                agent_entry["balance_cents"]
                    .as_i64()
                    .ok_or(Error::DeserializatonError {
                        message: format!(
                            "Failed to deserialize balance cents from {}",
                            agent_entry
                        ),
                        de_str: None,
                        value: Some(agent_entry.clone()),
                        nested_error: None,
                    })?;
            let strategic_bets_data: &Vec<Value> =
                agent_entry["strategic_bets"]
                    .as_array()
                    .ok_or(Error::DeserializatonError {
                        message: format!(
                            "Failed to deserialize strategic bets from {}",
                            agent_entry
                        ),
                        de_str: None,
                        value: Some(agent_entry.clone()),
                        nested_error: None,
                    })?;
            let mut strategic_bets: Vec<Bet> = Vec::new();
            let name_entry: Option<&str> = agent_entry["name"].as_str();
            let name: String = name_entry
                .unwrap_or(&format!("Agent {}", agent_number))
                .to_string();

            for strategic_bet_data in strategic_bets_data {
                let bet_value: BetValue = strategic_bet_data["bet_value"].clone().try_into()?;
                let amount_cents: i64 = strategic_bet_data["amount_cents"].as_i64().ok_or(
                    Error::DeserializatonError {
                        message: format!(
                            "Failed to deserialize amount cents from {}",
                            strategic_bet_data
                        ),
                        de_str: None,
                        value: Some(strategic_bet_data.clone()),
                        nested_error: None,
                    },
                )?;
                let initial_amount_cents: i64 = amount_cents;
                let progression_factor: i64 = strategic_bet_data["progression_factor"]
                    .as_i64()
                    .ok_or(Error::DeserializatonError {
                        message: format!(
                            "Failed to deserialize progression factor from {}",
                            strategic_bet_data
                        ),
                        de_str: None,
                        value: Some(strategic_bet_data.clone()),
                        nested_error: None,
                    })?;
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
