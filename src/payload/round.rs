use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Round {
    pub phase: RoundPhase,
    pub bomb: Option<BombState>,
    pub win_team: Option<super::team::Team>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RoundPhase {
    Live,
    FreezeTime,
    Over,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BombState {
    Planted,
    Exploded,
    Defused,
}
