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
        self.consolidate_bets();
        self.validate_bets();
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
                .for_each(|bet| bet.validate(Some(&self.roulette_type)))
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

#[cfg(test)]
mod test {
    use crate::{
        agent::agent::Agent,
        bet::{bet::Bet, bet_state::BetState, bet_value::BetValue},
        json::deserializable::I64Deserializable,
        roulette::roulette_type::RouletteType,
        types::{
            adjacent_numbers::AdjacentNumbers, color::Color, column::Column,
            double_column::DoubleColumn, dozen::Dozen, even_odd::EvenOdd, half::Half, row::Row,
        },
    };

    use super::RouletteGame;
    fn create_game(agents_opt: Option<Vec<Agent>>) -> RouletteGame {
        let agents = match agents_opt {
            Some(agents_list) => agents_list,
            None => {
                let strategic_bets_1 = vec![Bet {
                    amount_cents: 1000,
                    bet_logs: vec![],
                    bet_state: BetState::Active,
                    bet_value: BetValue::Color(Color::Red),
                    initial_amount_cents: 1000,
                    progression_factor: 2,
                }];
                let strategic_bets_2 = vec![Bet {
                    amount_cents: 1000,
                    bet_logs: vec![],
                    bet_state: BetState::Active,
                    bet_value: BetValue::Number(17),
                    initial_amount_cents: 1000,
                    progression_factor: 2,
                }];
                vec![
                    Agent {
                        balance_cents: 100000,
                        strategic_bets: strategic_bets_1,
                        name: String::from("AGENT1"),
                        agent_logs: Vec::new(),
                    },
                    Agent {
                        balance_cents: 100000,
                        strategic_bets: strategic_bets_2,
                        name: String::from("AGENT2"),
                        agent_logs: Vec::new(),
                    },
                ]
            }
        };
        let res = RouletteGame::new(1, agents, 10, false, Some(RouletteType::European));
        assert!(res.is_ok());
        return res.unwrap();
    }

    fn assign_agents(game: &mut RouletteGame, bet_values: Vec<BetValue>) {
        game.agents = {
            let agents = bet_values
                .iter()
                .enumerate()
                .map(|(index, bet_value)| Agent {
                    balance_cents: 100000,
                    strategic_bets: vec![Bet {
                        amount_cents: 1000,
                        bet_logs: vec![],
                        bet_state: BetState::Active,
                        bet_value: bet_value.clone(),
                        initial_amount_cents: 1000,
                        progression_factor: 2,
                    }],
                    name: format!("AGENT-{}", index + 1),
                    agent_logs: Vec::new(),
                })
                .collect();
            agents
        }
    }

    #[test]
    fn test_log_round() {
        let mut game = create_game(None);
        let slot = game.board.slots[0].clone();
        game.log_round(1, &slot);
        assert_eq!(game.game_logs.len(), 1);
        assert_eq!(game.game_logs[0].round_number, 1);
        assert_eq!(game.game_logs[0].winning_slot, slot);
        assert_eq!(game.agents[0].agent_logs.len(), 1);
        assert_eq!(game.agents[0].agent_logs[0].round_number, 1);
        assert_eq!(game.agents[0].strategic_bets[0].bet_logs.len(), 1);
        assert_eq!(game.agents[0].strategic_bets[0].bet_logs[0].round_number, 1);
        assert_eq!(
            game.agents[0].strategic_bets[0].bet_logs[0].amount_cents,
            1000
        );
        assert_eq!(
            game.agents[0].strategic_bets[0].bet_logs[0].bet_state,
            BetState::Active
        );
    }

    #[test]
    fn test_determine_bet_results_color_win() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        assign_agents(&mut game, vec![BetValue::Color(slot.color)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Won);
        assert_eq!(game.agents[0].balance_cents, 102000);
    }

    #[test]
    fn test_determine_bet_results_color_lose() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        let color = match slot.color {
            Color::Red => Color::Black,
            _ => Color::Red,
        };
        assign_agents(&mut game, vec![BetValue::Color(color)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Lost);
        assert_eq!(game.agents[0].balance_cents, 100000);
    }

    #[test]
    fn test_determine_bet_results_number_win() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        assign_agents(&mut game, vec![BetValue::Number(slot.number)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Won);
        assert_eq!(game.agents[0].balance_cents, 136000);
    }

    #[test]
    fn test_determine_bet_results_number_lose() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        let num = match slot.number {
            36 => slot.number - 1,
            _ => slot.number + 1,
        };
        assign_agents(&mut game, vec![BetValue::Number(num)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Lost);
        assert_eq!(game.agents[0].balance_cents, 100000);
    }

    #[test]
    fn test_determine_bet_results_column_win() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        assign_agents(&mut game, vec![BetValue::Column(slot.column)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Won);
        assert_eq!(game.agents[0].balance_cents, 112000);
    }

    #[test]
    fn test_determine_bet_results_column_lose() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        let column = match slot.column {
            Column::Twelve => Column::One,
            _ => Column::Twelve,
        };
        assign_agents(&mut game, vec![BetValue::Column(column)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Lost);
        assert_eq!(game.agents[0].balance_cents, 100000);
    }

    #[test]
    fn test_determine_bet_results_dozen_win() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        assign_agents(&mut game, vec![BetValue::Dozen(slot.dozen)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Won);
        assert_eq!(game.agents[0].balance_cents, 103000);
    }

    #[test]
    fn test_determine_bet_results_dozen_lose() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        let dozen = match slot.dozen {
            Dozen::Three => Dozen::Two,
            _ => Dozen::Three,
        };
        assign_agents(&mut game, vec![BetValue::Dozen(dozen)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Lost);
        assert_eq!(game.agents[0].balance_cents, 100000);
    }

    #[test]
    fn test_determine_bet_results_even_odd_win() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        assign_agents(&mut game, vec![BetValue::EvenOdd(slot.even_odd)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Won);
        assert_eq!(game.agents[0].balance_cents, 102000);
    }

    #[test]
    fn test_determine_bet_results_even_odd_lose() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        let even_odd = match slot.even_odd {
            EvenOdd::Even => EvenOdd::Odd,
            _ => EvenOdd::Even,
        };
        assign_agents(&mut game, vec![BetValue::EvenOdd(even_odd)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Lost);
        assert_eq!(game.agents[0].balance_cents, 100000);
    }

    #[test]
    fn test_determine_bet_results_half_win() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        assign_agents(&mut game, vec![BetValue::Half(slot.half)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Won);
        assert_eq!(game.agents[0].balance_cents, 102000);
    }

    #[test]
    fn test_determine_bet_results_half_lose() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        let half = match slot.half {
            Half::One => Half::Two,
            _ => Half::One,
        };
        assign_agents(&mut game, vec![BetValue::Half(half)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Lost);
        assert_eq!(game.agents[0].balance_cents, 100000);
    }
    #[test]
    fn test_determine_bet_results_row_win() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        assign_agents(&mut game, vec![BetValue::Row(slot.row)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Won);
        assert_eq!(game.agents[0].balance_cents, 103000);
    }

    #[test]
    fn test_determine_bet_results_row_lose() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        let row = match slot.row {
            Row::Three => Row::Two,
            _ => Row::Three,
        };
        assign_agents(&mut game, vec![BetValue::Row(row)]);
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Lost);
        assert_eq!(game.agents[0].balance_cents, 100000);
    }

    #[test]
    fn test_determine_bet_results_double_column_win() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        let column1 = slot.column;
        let column2 = match column1 {
            Column::Twelve => Column::from_number((slot.column.value() - 1) as i64).unwrap(),
            _ => Column::from_number((slot.column.value() + 1) as i64).unwrap(),
        };
        assign_agents(
            &mut game,
            vec![BetValue::DoubleColumn(DoubleColumn {
                columns: [column1, column2],
            })],
        );
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Won);
        assert_eq!(game.agents[0].balance_cents, 106000);
    }

    #[test]
    fn test_determine_bet_results_double_column_lose() {
        let mut game = create_game(None);
        let slot = game.board.slots[1].clone();
        let (column1, column2) = match slot.column {
            Column::Twelve => (Column::One, Column::Two),
            Column::Eleven => (Column::One, Column::Two),
            _ => (Column::Eleven, Column::Twelve),
        };
        assign_agents(
            &mut game,
            vec![BetValue::DoubleColumn(DoubleColumn {
                columns: [column1, column2],
            })],
        );
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Lost);
        assert_eq!(game.agents[0].balance_cents, 100000);
    }

    #[test]
    fn test_determine_bet_results_two_adjacent_numbers_win() {
        let mut game = create_game(None);
        let slot = game
            .board
            .slots
            .iter()
            .filter(|slot| slot.number == 1)
            .next()
            .unwrap()
            .clone();

        assign_agents(
            &mut game,
            vec![BetValue::AdjacentNumbers(AdjacentNumbers {
                numbers: vec![1, 2],
            })],
        );
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Won);
        assert_eq!(game.agents[0].balance_cents, 118000);
    }

    #[test]
    fn test_determine_bet_results_two_adjacent_numbers_lose() {
        let mut game = create_game(None);
        let slot = game
            .board
            .slots
            .iter()
            .filter(|slot| slot.number == 1)
            .next()
            .unwrap()
            .clone();

        assign_agents(
            &mut game,
            vec![BetValue::AdjacentNumbers(AdjacentNumbers {
                numbers: vec![2, 3],
            })],
        );
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Lost);
        assert_eq!(game.agents[0].balance_cents, 100000);
    }

    #[test]
    fn test_determine_bet_results_three_adjacent_numbers_win() {
        let mut game = create_game(None);
        let slot = game
            .board
            .slots
            .iter()
            .filter(|slot| slot.number == 1)
            .next()
            .unwrap()
            .clone();
        assign_agents(
            &mut game,
            vec![BetValue::AdjacentNumbers(AdjacentNumbers {
                numbers: vec![0, 1, 2],
            })],
        );
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Won);
        assert_eq!(game.agents[0].balance_cents, 112000);
    }

    #[test]
    fn test_determine_bet_results_three_adjacent_numbers_lose() {
        let mut game = create_game(None);
        let slot = game
            .board
            .slots
            .iter()
            .filter(|slot| slot.number == 29)
            .next()
            .unwrap()
            .clone();
        assign_agents(
            &mut game,
            vec![BetValue::AdjacentNumbers(AdjacentNumbers {
                numbers: vec![0, 2, 3],
            })],
        );
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Lost);
        assert_eq!(game.agents[0].balance_cents, 100000);
    }

    #[test]
    fn test_determine_bet_results_four_adjacent_numbers_win() {
        let mut game = create_game(None);
        let slot = game
            .board
            .slots
            .iter()
            .filter(|slot| slot.number == 1)
            .next()
            .unwrap()
            .clone();
        assign_agents(
            &mut game,
            vec![BetValue::AdjacentNumbers(AdjacentNumbers {
                numbers: vec![1, 2, 4, 5],
            })],
        );
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Won);
        assert_eq!(game.agents[0].balance_cents, 109000);
    }

    #[test]
    fn test_determine_bet_results_four_adjacent_numbers_lose() {
        let mut game = create_game(None);
        let slot = game
            .board
            .slots
            .iter()
            .filter(|slot| slot.number == 1)
            .next()
            .unwrap()
            .clone();
        assign_agents(
            &mut game,
            vec![BetValue::AdjacentNumbers(AdjacentNumbers {
                numbers: vec![2, 3, 5, 6],
            })],
        );
        game.determine_bet_results(&slot);
        assert_eq!(game.agents[0].strategic_bets[0].bet_state, BetState::Lost);
        assert_eq!(game.agents[0].balance_cents, 100000);
    }

    #[test]
    fn test_collect_bets_active() {
        let mut game = create_game(None);
        game.collect_bets();
        assert_eq!(game.agents[0].balance_cents, 99000);
        assert_eq!(game.agents[1].balance_cents, 99000);
    }

    #[test]
    fn test_collect_bets_inactive() {
        let mut game = create_game(None);
        game.agents[0].strategic_bets[0].bet_state = BetState::Inactive;
        game.collect_bets();
        assert_eq!(game.agents[0].balance_cents, 100000);
    }

    #[test]
    fn test_spin() {
        let mut game = create_game(None);
        let res = game.spin();
        assert!(res.is_ok());
        assert_eq!(game.board.slots.contains(&res.unwrap()), true);
    }

    #[test]
    fn test_ensure_agent_funds() {
        let mut game = create_game(None);
        game.agents[0].balance_cents = 1000;
        game.agents[0].strategic_bets[0].amount_cents = 1001;
        game.ensure_agent_funds();
        assert_eq!(
            game.agents[0].strategic_bets[0].bet_state,
            BetState::Inactive
        );
    }
}
