use crate::json::json_reader::JsonReader;
use crate::roulette::stats::Stats;

use actix_web::{get, HttpResponse, Responder};
use tracing::{error, info};

use crate::roulette::game_runner::run;

const GAME_CONFIG_FILENAME: &str = "./res/game.json";
const AGENTS_FILENAME: &str = "./res/agents.json";

#[get("/test")]
async fn run_with_files() -> impl Responder {
    let game_config_res = JsonReader::read_game_json(GAME_CONFIG_FILENAME);
    let agents_res = JsonReader::read_agents_json(AGENTS_FILENAME);
    let game_config;
    let agents;
    if game_config_res.is_ok() {
        game_config = game_config_res.unwrap();
    } else {
        error!(
            "Failed to read game config: {}, path: {}",
            game_config_res.err().unwrap(),
            GAME_CONFIG_FILENAME
        );
        return HttpResponse::InternalServerError().body("Failed to read game config");
    }
    if agents_res.is_ok() {
        agents = agents_res.unwrap();
    } else {
        error!(
            "Failed to read agents: {}, path: {}",
            agents_res.err().unwrap(),
            AGENTS_FILENAME
        );
        return HttpResponse::InternalServerError().body("Failed to read agents");
    }
    match run(game_config, agents).await {
        Ok(result) => {
            let stats: Stats = Stats::from_games(&result);
            match serde_json::to_string(&stats) {
                Ok(json_string) => {
                    info!("Stats: {}", stats);
                    HttpResponse::Ok().body(json_string)
                }
                Err(err) => {
                    error!("Failed to convert stats to json: {}", err);
                    HttpResponse::InternalServerError().body("Failed to convert stats to json")
                }
            }
        }
        Err(err) => {
            error!("Failed to run with files: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to run based off jsons")
        }
    }
}
