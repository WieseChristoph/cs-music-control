use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub enum Team {
    CT,
    T,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TeamInfo {
    pub score: u8,
    pub consecutive_round_losses: u8,
    pub timeouts_remaining: u8,
    pub matches_won_this_series: u8,
}
