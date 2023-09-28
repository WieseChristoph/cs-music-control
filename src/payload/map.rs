use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Map {
    pub mode: Mode,
    pub name: String,
    pub phase: GeneralPhase,
    pub round: u8,
    pub team_ct: super::team::TeamInfo,
    pub team_t: super::team::TeamInfo,
    pub num_matches_to_win_series: u8,
    pub current_spectators: Option<u8>,
    pub souvenirs_total: Option<u8>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    #[serde(rename = "gungameprogressive")]
    ArmsRace,
    Competitive,
    Casual,
    Custom,
    Deathmatch,
    #[serde(rename = "gungametrbomb")]
    Demolition,
    Survival,
    Training,
    #[serde(rename = "scrimcomp2v2")]
    Wingman,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GeneralPhase {
    Warmup,
    Live,
    Intermission,
    GameOver,
}
