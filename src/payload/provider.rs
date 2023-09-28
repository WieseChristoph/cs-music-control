use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Provider {
    pub name: String,
    pub appid: u32,
    pub version: u32,
    #[serde(rename = "steamid")]
    pub steam_id: String,
    pub timestamp: u64,
}
