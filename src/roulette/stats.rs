use core::fmt;
use prettytable::{Cell, Row, Table};
use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use std::collections::HashMap;

use super::roulette_game::RouletteGame;
use crate::bet::{bet::Bet, bet_state::BetState};

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Stats {
    average_agent_balances: HashMap<String, i64>,
    average_bet_win_percentage: HashMap<String, HashMap<BetHash, f64>>,
    average_bet_income: HashMap<String, HashMap<BetHash, i64>>,
    longest_loss_streak_pet_bet: HashMap<String, HashMap<BetHash, i64>>,
}
impl serde::Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        let serialized_bet_statistics: HashMap<BetHash, SerializedBetStats> = self
            .average_bet_win_percentage
            .iter()
            .map(|(agent_name, bet_win_percentages)| {
                bet_win_percentages
                    .iter()
                    .map(|(bet_hash, win_percentage)| {
                        (
                            bet_hash.clone(),
                            SerializedBetStats {
                                agent_name: agent_name.clone(),
                                bet_type: bet_hash.bet_type.clone(),
                                bet_value: bet_hash.bet_value.clone(),
                                progression_factor: bet_hash.progression_factor,
                                win_percentage: *win_percentage,
                                average_bet_income: self.average_bet_income[agent_name][bet_hash],
                                longest_loss_streak: self.longest_loss_streak_pet_bet[agent_name]
                                    [bet_hash],
                            },
                        )
                    })
                    .collect::<HashMap<BetHash, SerializedBetStats>>()
            })
            .flatten()
            .collect();
        map.serialize_entry("average_agent_balances", &self.average_agent_balances)?;
        map.serialize_entry("bet_statistics", &serialized_bet_statistics)?;
        map.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str("\nAverage Agent Balances:\n");
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("AGENT"),
            Cell::new("AVERAGE BALANCE"),
        ]));
        self.average_agent_balances
            .iter()
            .for_each(|(agent_name, balance)| {
                table.add_row(Row::new(vec![
                    Cell::new(agent_name),
                    Cell::new(&Self::format_as_dollars_cents(*balance)),
                ]));
            });
        output.push_str(&table.to_string());
        output.push_str("\n\nAverage Bet Win Percentages:\n");
        table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("AGENT"),
            Cell::new("BET TYPE"),
            Cell::new("BET VALUE"),
            Cell::new("WIN PERCENTAGE"),
        ]));
        self.average_bet_win_percentage
            .iter()
            .for_each(|(agent_name, bet_win_percentages)| {
                bet_win_percentages
                    .iter()
                    .for_each(|(bet_hash, win_percentage)| {
                        table.add_row(Row::new(vec![
                            Cell::new(agent_name),
                            Cell::new(bet_hash.bet_type.as_str()),
                            Cell::new(bet_hash.bet_value.as_str()),
                            Cell::new(Self::format_as_percentage(*win_percentage).as_str()),
                        ]));
                    });
            });
        output.push_str(&table.to_string());
        output.push_str("\n\nAverage Bet Income:\n");
        table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("AGENT"),
            Cell::new("BET TYPE"),
            Cell::new("BET VALUE"),
            Cell::new("AVERAGE BET INCOME"),
        ]));
        self.average_bet_income
            .iter()
            .for_each(|(agent_name, bet_incomes)| {
                bet_incomes.iter().for_each(|(bet_hash, income)| {
                    table.add_row(Row::new(vec![
                        Cell::new(agent_name),
                        Cell::new(bet_hash.bet_type.as_str()),
                        Cell::new(bet_hash.bet_value.as_str()),
                        Cell::new(Self::format_as_dollars_cents(*income).as_str()),
                    ]));
                });
            });
        output.push_str(&table.to_string());
        output.push_str("\n\nLongest Loss Streak Per Bet:\n");
        table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("AGENT"),
            Cell::new("BET TYPE"),
            Cell::new("BET VALUE"),
            Cell::new("LONGEST LOSS STREAK"),
        ]));
        self.longest_loss_streak_pet_bet.iter().for_each(
            |(agent_name, bet_longest_loss_streaks)| {
                bet_longest_loss_streaks
                    .iter()
                    .for_each(|(bet_hash, longest_loss_streak)| {
                        table.add_row(Row::new(vec![
                            Cell::new(agent_name),
                            Cell::new(bet_hash.bet_type.as_str()),
                            Cell::new(bet_hash.bet_value.as_str()),
                            Cell::new(longest_loss_streak.to_string().as_str()),
                        ]));
                    });
            },
        );
        output.push_str(&table.to_string());
        return write!(f, "{}", output);
    }
}

impl Stats {
    pub fn from_games(games: &Vec<RouletteGame>) -> Self {
        return Stats {
            average_agent_balances: Self::gen_average_agent_balances(games),
            average_bet_win_percentage: Self::gen_bet_win_percentages(games),
            average_bet_income: Self::gen_average_bet_income(games),
            longest_loss_streak_pet_bet: Self::gen_longest_loss_streak_pet_bet(games),
        };
    }

    fn gen_average_agent_balances(games: &Vec<RouletteGame>) -> HashMap<String, i64> {
        let mut average_agent_balances: HashMap<String, i64> = HashMap::new();
        let number_of_games = games.len() as i32;

        for game in games.iter() {
            for agent in game.agents.iter() {
                average_agent_balances
                    .entry(agent.name.clone())
                    .and_modify(|balance| *balance += agent.balance_cents)
                    .or_insert(agent.balance_cents);
            }
        }

        average_agent_balances.iter_mut().for_each(|(_, balance)| {
            *balance /= number_of_games as i64;
        });

        return average_agent_balances;
    }

    fn gen_bet_win_percentages(
        games: &Vec<RouletteGame>,
    ) -> HashMap<String, HashMap<BetHash, f64>> {
        let mut average_bet_win_percentage: HashMap<String, HashMap<BetHash, f64>> = HashMap::new();
        let number_of_games = games.len() as i32;

        for game in games.iter() {
            for agent in game.agents.iter() {
                for bet in agent.strategic_bets.iter() {
                    let number_of_wins = bet
                        .bet_logs
                        .iter()
                        .filter(|bet_log| bet_log.bet_state == BetState::Won)
                        .count();
                    let number_of_rounds = bet.bet_logs.len();
                    let win_percentage: f64 = number_of_wins as f64 / number_of_rounds as f64;
                    *average_bet_win_percentage
                        .entry(agent.name.clone())
                        .or_insert(HashMap::new())
                        .entry(bet.into())
                        .or_insert(0.0) += win_percentage;
                }
            }
        }

        average_bet_win_percentage
            .iter_mut()
            .for_each(|(_, bet_win_percentages)| {
                bet_win_percentages
                    .iter_mut()
                    .for_each(|(_, win_percentage_sum)| {
                        *win_percentage_sum /= number_of_games as f64;
                    });
            });

        return average_bet_win_percentage;
    }

    fn gen_average_bet_income(games: &Vec<RouletteGame>) -> HashMap<String, HashMap<BetHash, i64>> {
        let mut average_bet_income: HashMap<String, HashMap<BetHash, i64>> = HashMap::new();
        let number_of_games = games.len() as i32;

        for game in games.iter() {
            for agent in game.agents.iter() {
                for bet in agent.strategic_bets.iter() {
                    let mut income_per_bet: i64 = 0;
                    bet.bet_logs.iter().for_each(|bet_log| {
                        if bet_log.bet_state == BetState::Won {
                            income_per_bet += bet_log.amount_cents;
                        } else if bet_log.bet_state == BetState::Lost {
                            income_per_bet -= bet_log.amount_cents;
                        }
                    });
                    *average_bet_income
                        .entry(agent.name.clone())
                        .or_insert(HashMap::new())
                        .entry(bet.into())
                        .or_insert(0) += income_per_bet;
                }
            }
        }

        average_bet_income.iter_mut().for_each(|(_, bet_incomes)| {
            bet_incomes.iter_mut().for_each(|(_, income_sum)| {
                *income_sum /= number_of_games as i64;
            });
        });

        return average_bet_income;
    }

    fn gen_longest_loss_streak_pet_bet(
        games: &Vec<RouletteGame>,
    ) -> HashMap<String, HashMap<BetHash, i64>> {
        let mut longest_loss_streak_pet_bet: HashMap<String, HashMap<BetHash, i64>> =
            HashMap::new();

        for game in games.iter() {
            for agent in game.agents.iter() {
                for bet in agent.strategic_bets.iter() {
                    let mut longest_loss_streak = 0;
                    let mut curr_loss_streak = 0;
                    for bet_log in bet.bet_logs.iter() {
                        if bet_log.bet_state == BetState::Lost {
                            curr_loss_streak += 1;
                        } else {
                            curr_loss_streak = 0;
                        }
                        if curr_loss_streak > longest_loss_streak {
                            longest_loss_streak = curr_loss_streak;
                        }
                    }
                    let _ = *longest_loss_streak_pet_bet
                        .entry(agent.name.clone())
                        .or_insert(HashMap::new())
                        .entry(BetHash::from(bet))
                        .and_modify(|existing_streak| {
                            if longest_loss_streak > *existing_streak {
                                *existing_streak = longest_loss_streak;
                            }
                        })
                        .or_insert(longest_loss_streak);
                }
            }
        }

        return longest_loss_streak_pet_bet;
    }

    fn format_as_percentage(value: f64) -> String {
        return format!("{:.2}%", value * 100.0);
    }

    fn format_as_dollars_cents(value: i64) -> String {
        let dollars = value / 100;
        let cents = value.abs() % 100;

        if value < 0 {
            format!("-${}.{:02}", dollars.abs(), cents)
        } else {
            format!("${}.{:02}", dollars, cents)
        }
    }
}

#[derive(Debug, PartialEq, Hash, Clone, Deserialize, Eq)]
struct BetHash {
    bet_type: String,
    bet_value: String,
    initial_amount_cents: i64,
    progression_factor: i64,
}
impl From<Bet> for BetHash {
    fn from(bet: Bet) -> Self {
        BetHash::from(&bet)
    }
}
impl From<&Bet> for BetHash {
    fn from(bet: &Bet) -> Self {
        BetHash {
            bet_type: bet.bet_value.get_type(),
            bet_value: bet.bet_value.get_value_string(),
            initial_amount_cents: bet.initial_amount_cents,
            progression_factor: bet.progression_factor,
        }
    }
}
impl Serialize for BetHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let key = format!(
            "{}_{}_{}_{}",
            self.bet_type, self.bet_value, self.initial_amount_cents, self.progression_factor
        );
        serializer.serialize_str(&key)
    }
}

#[derive(Serialize)]
struct SerializedBetStats {
    agent_name: String,
    bet_type: String,
    bet_value: String,
    progression_factor: i64,
    win_percentage: f64,
    average_bet_income: i64,
    longest_loss_streak: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::agent::Agent;
    use crate::agent::agent_log::AgentLog;
    use crate::bet::bet_log::BetLog;
    use crate::bet::bet_value::BetValue;
    use crate::board::board::Board;
    use crate::board::slot::Slot;
    use crate::roulette::game_logs::GameLog;
    use crate::roulette::roulette_game::RouletteGame;
    use crate::roulette::roulette_type::RouletteType;
    use crate::types::{
        color::Color, column::Column, dozen::Dozen, even_odd::EvenOdd, half::Half, row::Row,
    };

    const AGENT_1_NAME: &'static str = "BOB";
    const AGENT_2_NAME: &'static str = "ALICE";

    fn create_testing_board() -> Board {
        let mut slots = Vec::new();
        slots.push(Slot {
            number: 0,
            color: Color::Green,
            even_odd: EvenOdd::Zero,
            dozen: Dozen::Zero,
            half: Half::Zero,
            row: Row::Zero,
            column: Column::Zero,
        });
        for i in 1..37 {
            slots.push(Slot {
                number: i,
                color: if i % 2 == 0 { Color::Red } else { Color::Black },
                even_odd: if i % 2 == 0 {
                    EvenOdd::Even
                } else {
                    EvenOdd::Odd
                },
                dozen: Dozen::try_from(i).unwrap(),
                half: Half::try_from(i).unwrap(),
                row: Row::try_from(i).unwrap(),
                column: Column::try_from(i).unwrap(),
            });
        }
        let board = Board { slots };
        return board;
    }

    fn create_games() -> Vec<RouletteGame> {
        let board = create_testing_board();
        let mut number_to_slot = HashMap::new();
        for slot in board.slots.iter() {
            number_to_slot.insert(slot.number, slot.clone());
        }
        let agent1_bet1 = Bet {
            amount_cents: 1000,
            bet_logs: vec![
                BetLog {
                    bet_state: BetState::Lost,
                    amount_cents: 1000,
                    round_number: 1,
                },
                BetLog {
                    bet_state: BetState::Won,
                    amount_cents: 2000,
                    round_number: 2,
                },
                BetLog {
                    bet_state: BetState::Won,
                    amount_cents: 1000,
                    round_number: 3,
                },
                BetLog {
                    bet_state: BetState::Lost,
                    amount_cents: 1000,
                    round_number: 4,
                },
                BetLog {
                    bet_state: BetState::Lost,
                    amount_cents: 2000,
                    round_number: 5,
                },
            ],
            bet_value: BetValue::Color(Color::Red),
            initial_amount_cents: 1000,
            progression_factor: 2,
            bet_state: BetState::Active,
        };
        let agent1_bet2 = Bet {
            amount_cents: 1000,
            bet_logs: vec![
                BetLog {
                    bet_state: BetState::Lost,
                    amount_cents: 1000,
                    round_number: 1,
                },
                BetLog {
                    bet_state: BetState::Lost,
                    amount_cents: 1000,
                    round_number: 2,
                },
                BetLog {
                    bet_state: BetState::Lost,
                    amount_cents: 1000,
                    round_number: 3,
                },
                BetLog {
                    bet_state: BetState::Won,
                    amount_cents: 36000,
                    round_number: 4,
                },
                BetLog {
                    bet_state: BetState::Lost,
                    amount_cents: 1000,
                    round_number: 5,
                },
            ],
            bet_value: BetValue::Number(23),
            initial_amount_cents: 1000,
            progression_factor: 1,
            bet_state: BetState::Active,
        };

        let agent2_bet1 = Bet {
            amount_cents: 1000,
            bet_logs: vec![
                BetLog {
                    bet_state: BetState::Lost,
                    amount_cents: 1000,
                    round_number: 1,
                },
                BetLog {
                    bet_state: BetState::Won,
                    amount_cents: 2000,
                    round_number: 2,
                },
                BetLog {
                    bet_state: BetState::Won,
                    amount_cents: 1000,
                    round_number: 3,
                },
                BetLog {
                    bet_state: BetState::Lost,
                    amount_cents: 1000,
                    round_number: 4,
                },
                BetLog {
                    bet_state: BetState::Lost,
                    amount_cents: 2000,
                    round_number: 5,
                },
            ],
            bet_value: BetValue::Dozen(Dozen::One),
            initial_amount_cents: 1000,
            progression_factor: 2,
            bet_state: BetState::Active,
        };

        let game = RouletteGame {
            game_number: 1,
            board: board.clone(),
            agents: vec![
                Agent {
                    name: AGENT_1_NAME.to_string(),
                    balance_cents: 131000,
                    strategic_bets: vec![agent1_bet1.clone(), agent1_bet2.clone()],
                    agent_logs: vec![
                        AgentLog {
                            round_number: 1,
                            balance_cents: 98000,
                        },
                        AgentLog {
                            round_number: 2,
                            balance_cents: 99000,
                        },
                        AgentLog {
                            round_number: 3,
                            balance_cents: 99000,
                        },
                        AgentLog {
                            round_number: 4,
                            balance_cents: 134000,
                        },
                        AgentLog {
                            round_number: 5,
                            balance_cents: 131000,
                        },
                    ],
                },
                Agent {
                    name: AGENT_2_NAME.to_string(),
                    balance_cents: 99000,
                    strategic_bets: vec![agent2_bet1.clone()],
                    agent_logs: vec![
                        AgentLog {
                            round_number: 1,
                            balance_cents: 99000,
                        },
                        AgentLog {
                            round_number: 2,
                            balance_cents: 101000,
                        },
                        AgentLog {
                            round_number: 3,
                            balance_cents: 102000,
                        },
                        AgentLog {
                            round_number: 4,
                            balance_cents: 101000,
                        },
                        AgentLog {
                            round_number: 5,
                            balance_cents: 99000,
                        },
                    ],
                },
            ],
            number_of_rounds: 10,
            allow_negative_balance: false,
            game_logs: vec![
                GameLog {
                    round_number: 1,
                    winning_slot: number_to_slot[&13].clone(),
                },
                GameLog {
                    round_number: 2,
                    winning_slot: number_to_slot[&12].clone(),
                },
                GameLog {
                    round_number: 3,
                    winning_slot: number_to_slot[&4].clone(),
                },
                GameLog {
                    round_number: 4,
                    winning_slot: number_to_slot[&23].clone(),
                },
                GameLog {
                    round_number: 5,
                    winning_slot: number_to_slot[&33].clone(),
                },
            ],
            roulette_type: RouletteType::European,
        };
        return vec![game];
    }

    #[test]
    fn test_average_bet_win_percentage() {
        let games = create_games();
        let stats = Stats::from_games(&games);
        let agent1_bet1_hash = &BetHash::from(&games[0].agents[0].strategic_bets[0]);
        let agent1_bet2_hash = &BetHash::from(&games[0].agents[0].strategic_bets[1]);
        let agent2_bet1_hash = &BetHash::from(&games[0].agents[1].strategic_bets[0]);
        assert_eq!(
            stats.average_bet_win_percentage[AGENT_1_NAME].len(),
            2,
            "{} should have 2 bet types",
            AGENT_1_NAME
        );
        assert_eq!(
            stats.average_bet_win_percentage[AGENT_2_NAME].len(),
            1,
            "{} should have 1 bet type",
            AGENT_2_NAME
        );
        assert_eq!(
            stats.average_bet_win_percentage[AGENT_1_NAME][agent1_bet1_hash], 0.4 as f64,
            "{}-Bet1 average win percentage has not been calculated correctly",
            AGENT_1_NAME
        );
        assert_eq!(
            stats.average_bet_win_percentage[AGENT_1_NAME][agent1_bet2_hash], 0.2 as f64,
            "{}-Bet2 average win percentage has not been calculated correctly",
            AGENT_1_NAME
        );
        assert_eq!(
            stats.average_bet_win_percentage[AGENT_2_NAME][agent2_bet1_hash], 0.4 as f64,
            "{}-Bet1 average win percentage has not been calculated correctly",
            AGENT_2_NAME
        );
    }

    #[test]
    fn test_average_bet_income() {
        let games = create_games();
        let stats = Stats::from_games(&games);
        let agent1_bet1_hash = &BetHash::from(&games[0].agents[0].strategic_bets[0]);
        let agent1_bet2_hash = &BetHash::from(&games[0].agents[0].strategic_bets[1]);
        let agent2_bet1_hash = &BetHash::from(&games[0].agents[1].strategic_bets[0]);
        assert_eq!(
            stats.average_bet_income[AGENT_1_NAME].len(),
            2,
            "{} should have 2 bet types",
            AGENT_1_NAME
        );
        assert_eq!(
            stats.average_bet_income[AGENT_2_NAME].len(),
            1,
            "{} should have 1 bet type",
            AGENT_2_NAME
        );
        assert_eq!(
            stats.average_bet_income[AGENT_1_NAME][agent1_bet1_hash], -1000,
            "{}-Bet1 average bet income is calculated incorrectly",
            AGENT_1_NAME
        );
        assert_eq!(
            stats.average_bet_income[AGENT_1_NAME][agent1_bet2_hash], 32000,
            "{}-Bet2 average bet income is calculated incorrectly",
            AGENT_1_NAME,
        );
        assert_eq!(
            stats.average_bet_income[AGENT_2_NAME][agent2_bet1_hash], -1000,
            "{}-Bet2 average bet income is calculated incorrectly",
            AGENT_2_NAME
        );
    }

    #[test]
    fn test_longest_loss_streak_pet_bet() {
        let games = create_games();
        let stats = Stats::from_games(&games);
        let agent1_bet1_hash = &BetHash::from(&games[0].agents[0].strategic_bets[0]);
        let agent1_bet2_hash = &BetHash::from(&games[0].agents[0].strategic_bets[1]);
        let agent2_bet1_hash = &BetHash::from(&games[0].agents[1].strategic_bets[0]);

        assert_eq!(
            stats.longest_loss_streak_pet_bet[AGENT_1_NAME].len(),
            2,
            "{} should have 2 bet types",
            AGENT_1_NAME
        );
        assert_eq!(
            stats.longest_loss_streak_pet_bet[AGENT_2_NAME].len(),
            1,
            "{} should have 1 bet type1",
            AGENT_2_NAME
        );
        assert_eq!(
            stats.longest_loss_streak_pet_bet[AGENT_1_NAME][agent1_bet1_hash], 2,
            "{}-Bet1 longests loss streak is calculated incorrectly",
            AGENT_1_NAME
        );
        assert_eq!(
            stats.longest_loss_streak_pet_bet[AGENT_1_NAME][agent1_bet2_hash], 3,
            "{}-Bet2 longests loss streak is calculated incorrectly",
            AGENT_1_NAME
        );
        assert_eq!(
            stats.longest_loss_streak_pet_bet[AGENT_2_NAME][agent2_bet1_hash], 2,
            "{}-Bet1 longests loss streak is calculated incorrectly",
            AGENT_2_NAME
        );
    }

    #[test]
    fn test_average_agent_balances() {
        let games = create_games();
        let stats = Stats::from_games(&games);

        assert_eq!(
            stats.average_agent_balances[AGENT_1_NAME], 131000,
            "average agent balance for {} was miscalculated",
            AGENT_1_NAME
        );
        assert_eq!(
            stats.average_agent_balances[AGENT_2_NAME], 99000,
            "average agent balance for {} was miscalculated",
            AGENT_1_NAME
        );
    }
}
