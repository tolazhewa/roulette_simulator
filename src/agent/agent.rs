use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::bet::{bet::Bet, bet_state::BetState};

use super::agent_log::AgentLog;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct Agent {
    pub balance_cents: i64,
    pub strategic_bets: Vec<Bet>,
    pub name: String,
    pub agent_logs: Vec<AgentLog>,
}

impl Agent {
    pub fn consolidate_bets(&mut self) {
        let mut bets: HashMap<String, HashMap<String, Bet>> = HashMap::new();
        let mut consolidated_bets: Vec<Bet> = Vec::new();

        for bet in self.strategic_bets.iter_mut() {
            let bet_type_string = bet.bet_value.get_type();
            bets.entry(bet_type_string)
                .and_modify(|type_to_bets| {
                    type_to_bets
                        .entry(bet_hash(bet))
                        .and_modify(|existing_bet| {
                            existing_bet.initial_amount_cents += bet.initial_amount_cents;
                            existing_bet.amount_cents += bet.amount_cents;
                        })
                        .or_insert(bet.clone());
                })
                .or_insert_with(|| {
                    let mut type_to_bets: HashMap<String, Bet> = HashMap::new();
                    type_to_bets.insert(bet_hash(bet), bet.clone());
                    return type_to_bets;
                });
        }
        for (_, type_to_bet) in bets {
            for (_, bet) in type_to_bet {
                consolidated_bets.push(bet);
            }
        }
        self.strategic_bets = consolidated_bets;
    }

    pub fn allow_all_bets(&mut self) {
        self.strategic_bets.iter_mut().for_each(|bet| {
            bet.bet_state = BetState::Active;
        });
    }

    pub fn determine_affordable_bets(&mut self) {
        let mut total_bet_value = 0;
        for bet in self.strategic_bets.iter_mut() {
            total_bet_value += bet.amount_cents;
            if total_bet_value <= self.balance_cents {
                bet.bet_state = BetState::Active;
            } else {
                bet.bet_state = BetState::Inactive;
            }
        }
    }

    pub fn play_strategy(&mut self) {
        for bet in self.strategic_bets.iter_mut() {
            if bet.bet_state == BetState::Lost {
                bet.amount_cents *= bet.progression_factor;
            } else if bet.bet_state == BetState::Won {
                bet.amount_cents = bet.initial_amount_cents;
            }
        }
    }
}

fn bet_hash(bet: &Bet) -> String {
    return format!("{:?} {:?}", bet.bet_value, bet.progression_factor);
}

#[cfg(test)]
mod test {
    use crate::{bet::bet_value::BetValue, types::color::Color};

    use super::*;
    #[test]
    fn test_consolidate_bets() {
        let mut agent = Agent {
            balance_cents: 100000,
            name: String::from("Test Agent"),
            strategic_bets: vec![
                Bet {
                    amount_cents: 1000,
                    bet_logs: Vec::new(),
                    bet_state: BetState::Active,
                    bet_value: BetValue::Number(1),
                    initial_amount_cents: 1000,
                    progression_factor: 2,
                },
                Bet {
                    amount_cents: 1000,
                    bet_logs: Vec::new(),
                    bet_state: BetState::Active,
                    bet_value: BetValue::Number(1),
                    initial_amount_cents: 1000,
                    progression_factor: 2,
                },
            ],
            agent_logs: Vec::new(),
        };

        agent.consolidate_bets();
        assert_eq!(agent.strategic_bets.len(), 1);
        assert_eq!(agent.strategic_bets[0].amount_cents, 2000);
    }

    #[test]
    fn test_allow_all_bets() {
        let mut agent = Agent {
            balance_cents: 100000,
            name: String::from("Test Agent"),
            strategic_bets: vec![
                Bet {
                    amount_cents: 1000,
                    bet_logs: Vec::new(),
                    bet_state: BetState::Inactive,
                    bet_value: BetValue::Number(1),
                    initial_amount_cents: 1000,
                    progression_factor: 2,
                },
                Bet {
                    amount_cents: 1000,
                    bet_logs: Vec::new(),
                    bet_state: BetState::Active,
                    bet_value: BetValue::Color(Color::Black),
                    initial_amount_cents: 1000,
                    progression_factor: 2,
                },
            ],
            agent_logs: Vec::new(),
        };

        agent.allow_all_bets();
        assert_eq!(agent.strategic_bets.len(), 2);
        assert_eq!(agent.strategic_bets[0].bet_state, BetState::Active);
        assert_eq!(agent.strategic_bets[1].bet_state, BetState::Active);
    }

    #[test]
    fn test_determine_affordable_bets() {
        let mut agent = Agent {
            balance_cents: 100000,
            name: String::from("Test Agent"),
            strategic_bets: vec![
                Bet {
                    amount_cents: 1000,
                    bet_logs: Vec::new(),
                    bet_state: BetState::Inactive,
                    bet_value: BetValue::Number(1),
                    initial_amount_cents: 1000,
                    progression_factor: 2,
                },
                Bet {
                    amount_cents: 1000,
                    bet_logs: Vec::new(),
                    bet_state: BetState::Active,
                    bet_value: BetValue::Color(Color::Black),
                    initial_amount_cents: 1000,
                    progression_factor: 2,
                },
            ],
            agent_logs: Vec::new(),
        };

        agent.allow_all_bets();
        assert_eq!(agent.strategic_bets.len(), 2);
        assert_eq!(agent.strategic_bets[0].bet_state, BetState::Active);
        assert_eq!(agent.strategic_bets[1].bet_state, BetState::Active);
    }

    #[test]
    fn test_play_strategy() {
        let mut agent = Agent {
            balance_cents: 100000,
            name: String::from("Test Agent"),
            strategic_bets: vec![
                Bet {
                    amount_cents: 3500,
                    bet_logs: Vec::new(),
                    bet_state: BetState::Won,
                    bet_value: BetValue::Number(1),
                    initial_amount_cents: 3500,
                    progression_factor: 2,
                },
                Bet {
                    amount_cents: 5000,
                    bet_logs: Vec::new(),
                    bet_state: BetState::Lost,
                    bet_value: BetValue::Color(Color::Black),
                    initial_amount_cents: 5000,
                    progression_factor: 2,
                },
            ],
            agent_logs: Vec::new(),
        };

        agent.play_strategy();
        assert_eq!(agent.strategic_bets.len(), 2);
        let sum_of_amounts = agent
            .strategic_bets
            .iter()
            .map(|bet| bet.amount_cents)
            .sum::<i64>();
        assert_eq!(sum_of_amounts, 13500);
    }
}
