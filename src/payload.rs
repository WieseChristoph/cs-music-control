pub mod map;
pub mod player;
pub mod provider;
pub mod round;
pub mod team;

use map::Map;
use player::Player;
use provider::Provider;
use round::Round;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Payload {
    pub provider: Provider,
    pub map: Map,
    pub round: Round,
    pub player: Player,
}
