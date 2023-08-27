use core::fmt;
use prettytable::{Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::roulette_game::RouletteGame;
use crate::bet::{bet::Bet, bet_state::BetState};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Stats {
    average_agent_balances: HashMap<String, i64>,
    average_bet_win_percentage: HashMap<String, HashMap<BetHash, f64>>,
    average_bet_income: HashMap<String, HashMap<BetHash, i64>>,
    longest_loss_streak_pet_bet: HashMap<String, HashMap<BetHash, i64>>,
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
                        .entry(BetHash::from(bet))
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
                        .entry(BetHash::from(bet))
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
                    bet.bet_logs.iter().for_each(|bet_log| {
                        if bet_log.bet_state == BetState::Lost {
                            curr_loss_streak += 1;
                        } else {
                            if curr_loss_streak > longest_loss_streak {
                                longest_loss_streak = curr_loss_streak;
                            }
                            curr_loss_streak = 0;
                        }
                    });
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

#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize, Eq)]
struct BetHash {
    bet_type: String,
    bet_value: String,
    initial_amount_cents: i64,
    progression_factor: i64,
}
impl From<&Bet> for BetHash {
    fn from(bet: &Bet) -> Self {
        return Self {
            bet_type: bet.bet_value.get_type(),
            bet_value: bet.bet_value.get_value_string(),
            initial_amount_cents: bet.initial_amount_cents,
            progression_factor: bet.progression_factor,
        };
    }
}
