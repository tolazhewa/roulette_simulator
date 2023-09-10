use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::agent::{agent::Agent, agent_log::AgentLog};
use crate::bet::{bet_log::BetLog, bet_state::BetState, bet_value::BetValue};
use crate::board::{board::Board, slot::Slot};
use crate::error::Error;

use super::{game_logs::GameLog, roulette_type::RouletteType};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct RouletteGame {
    pub game_number: i32,
    pub board: Board,
    pub agents: Vec<Agent>,
    pub number_of_rounds: i32,
    pub allow_negative_balance: bool,
    pub game_logs: Vec<GameLog>,
    pub roulette_type: RouletteType,
}

impl RouletteGame {
    pub fn new(
        game_number: i32,
        agents: Vec<Agent>,
        number_of_rounds: i32,
        allow_negative_balance: bool,
        roulette_type: Option<RouletteType>,
    ) -> Result<Self, Error> {
        let roulette_type = roulette_type.unwrap_or(RouletteType::European);
        let board = Board::generate(&roulette_type)?;
        return Ok(RouletteGame {
            game_number,
            board,
            agents,
            number_of_rounds,
            allow_negative_balance,
            game_logs: Vec::new(),
            roulette_type,
        });
    }
    pub fn play(&mut self) -> Result<(), Error> {
        self.validate_bets();
        self.consolidate_bets();
        for round_number in 1..=self.number_of_rounds {
            self.play_round(round_number)?;
        }
        return Ok(());
    }
    fn play_round(&mut self, round_number: i32) -> Result<(), Error> {
        if self.allow_negative_balance {
            self.allow_all_bets();
        } else {
            self.ensure_agent_funds();
        }
        self.collect_bets();
        let winning_slot = self.spin()?;
        self.determine_bet_results(&winning_slot);
        self.log_round(round_number, &winning_slot);
        self.play_agent_strategies();
        return Ok(());
    }
    fn log_round(&mut self, round_number: i32, winning_slot: &Slot) {
        self.game_logs.push(GameLog {
            round_number,
            winning_slot: winning_slot.clone(),
        });

        for agent in self.agents.iter_mut() {
            agent.agent_logs.push(AgentLog {
                round_number,
                balance_cents: agent.balance_cents,
            });
            for bet in agent
                .strategic_bets
                .iter_mut()
                .filter(|bet| bet.bet_state != BetState::Inactive)
            {
                bet.bet_logs.push(BetLog {
                    round_number,
                    bet_state: bet.bet_state,
                    amount_cents: bet.amount_cents,
                });
            }
        }
    }
    fn determine_bet_results(&mut self, winning_slot: &Slot) {
        self.agents.iter_mut().for_each(|agent| {
            agent
                .strategic_bets
                .iter_mut()
                .filter(|bet| bet.bet_state == BetState::Active)
                .for_each(|bet| match &bet.bet_value {
                    BetValue::AdjacentNumbers(adjacent_numbers) => {
                        if adjacent_numbers.numbers.contains(&winning_slot.number) {
                            let num_count = adjacent_numbers.numbers.len();
                            bet.bet_state = BetState::Won;
                            if num_count == 2 {
                                agent.balance_cents += bet.amount_cents * 18;
                            } else if num_count == 3 {
                                agent.balance_cents += bet.amount_cents * 12;
                            } else if num_count == 4 {
                                agent.balance_cents += bet.amount_cents * 9;
                            }
                        } else {
                            bet.bet_state = BetState::Lost;
                        }
                    }
                    BetValue::Color(color) => {
                        if *color == winning_slot.color {
                            agent.balance_cents += bet.amount_cents * 2;
                            bet.bet_state = BetState::Won;
                        } else {
                            bet.bet_state = BetState::Lost;
                        }
                    }
                    BetValue::Column(column) => {
                        if *column == winning_slot.column {
                            agent.balance_cents += bet.amount_cents * 12;
                            bet.bet_state = BetState::Won;
                        } else {
                            bet.bet_state = BetState::Lost;
                        }
                    }
                    BetValue::Dozen(dozen) => {
                        if *dozen == winning_slot.dozen {
                            agent.balance_cents += bet.amount_cents * 3;
                            bet.bet_state = BetState::Won;
                        } else {
                            bet.bet_state = BetState::Lost;
                        }
                    }
                    BetValue::EvenOdd(even_odd) => {
                        if *even_odd == winning_slot.even_odd {
                            agent.balance_cents += bet.amount_cents * 2;
                            bet.bet_state = BetState::Won;
                        } else {
                            bet.bet_state = BetState::Lost;
                        }
                    }
                    BetValue::Half(half) => {
                        if *half == winning_slot.half {
                            agent.balance_cents += bet.amount_cents * 2;
                            bet.bet_state = BetState::Won;
                        } else {
                            bet.bet_state = BetState::Lost;
                        }
                    }
                    BetValue::Number(number) => {
                        if *number == winning_slot.number {
                            agent.balance_cents += bet.amount_cents * 36;
                            bet.bet_state = BetState::Won;
                        } else {
                            bet.bet_state = BetState::Lost;
                        }
                    }
                    BetValue::Row(row) => {
                        if *row == winning_slot.row {
                            agent.balance_cents += bet.amount_cents * 3;
                            bet.bet_state = BetState::Won;
                        } else {
                            bet.bet_state = BetState::Lost;
                        }
                    }
                    BetValue::DoubleColumn(double_column) => {
                        if double_column.columns.contains(&winning_slot.column) {
                            agent.balance_cents += bet.amount_cents * 6;
                            bet.bet_state = BetState::Won;
                        } else {
                            bet.bet_state = BetState::Lost;
                        }
                    }
                });
        });
    }
    fn collect_bets(&mut self) {
        self.agents.iter_mut().for_each(|agent: &mut Agent| {
            agent
                .strategic_bets
                .iter()
                .filter(|bet| bet.bet_state == BetState::Active)
                .for_each(|bet| {
                    agent.balance_cents -= bet.amount_cents;
                });
        });
    }
    fn spin(&mut self) -> Result<Slot, Error> {
        let mut rng = rand::thread_rng();
        return self
            .board
            .slots
            .choose(&mut rng)
            .ok_or(Error::GenericError {
                message: format!(
                    "Unable to choose a random slot from the board: {:?}",
                    self.board,
                ),
                nested_error: None,
            })
            .cloned();
    }
    fn consolidate_bets(&mut self) {
        self.agents
            .iter_mut()
            .for_each(|agent| agent.consolidate_bets());
    }
    fn validate_bets(&mut self) {
        self.agents.iter_mut().for_each(|agent| {
            agent
                .strategic_bets
                .iter_mut()
                .filter(|bet| bet.bet_state == BetState::Active)
                .for_each(|bet| bet.validate_bet(&self.roulette_type))
        });
    }
    fn allow_all_bets(&mut self) {
        self.agents
            .iter_mut()
            .for_each(|agent| agent.allow_all_bets());
    }
    fn ensure_agent_funds(&mut self) {
        self.agents
            .iter_mut()
            .for_each(|agent| agent.determine_affordable_bets());
    }
    fn play_agent_strategies(&mut self) {
        self.agents.iter_mut().for_each(|agent: &mut Agent| {
            agent.play_strategy();
        });
    }
}
