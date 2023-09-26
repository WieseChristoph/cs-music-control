use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Player {
    #[serde(rename = "steamid")]
    pub steam_id: String,
    pub clan: Option<String>,
    pub name: String,
    pub observer_slot: Option<u8>,
    pub team: Option<super::team::Team>,
    pub activity: Activity,
    pub state: Option<State>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Activity {
    Menu,
    Playing,
    TextInput,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub health: u8,
    pub armor: u8,
    pub helmet: bool,
    pub flashed: u8,
    pub smoked: u8,
    pub burning: u8,
    pub money: u16,
    pub round_kills: u8,
    pub round_killhs: u8,
    pub round_totaldmg: Option<u16>,
    pub equip_value: u16,
    #[serde(rename = "defusekit")]
    pub defuse_kit: Option<bool>,
}
