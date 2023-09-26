mod payload;

use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use chrono::Local;
use clap::Parser;
use enigo::{Enigo, Key, KeyboardControllable};
use indoc::formatdoc;
use payload::{round::RoundPhase, Payload};
use std::{
    fs::File,
    io::{self, Write},
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::RwLock;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        value_name = "CS_FOLDER_PATH",
        help = "Path to CS installation folder",
        num_args = 0..
    )]
    generate_config: Option<Vec<String>>,
}

#[derive(Debug)]
struct AppState {
    enigo: Enigo,
    is_playing: bool,
}

#[tokio::main]
async fn main() {
    let port = 3000;

    let args = Args::parse();

    if args.generate_config.is_some() {
        println!("Generating config...");
        match generate_config(&args.generate_config.unwrap().join(" "), port) {
            Ok(_) => println!("Config generated!"),
            Err(e) => eprintln!("Error generating config: {}", e),
        }
        return;
    }

    let state = Arc::new(RwLock::new(AppState {
        enigo: Enigo::new(),
        is_playing: true,
    }));

    // initialize tracing
    tracing_subscriber::fmt::init();

    println!("This program expects that your music is already playing!");

    let app = Router::new()
        // `POST /` goes to `handle_payload`
        .route("/", post(handle_payload))
        .layer(Extension(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::debug!("listening on {}", addr);
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn generate_config(cs_folder_path: &str, port: u16) -> io::Result<()> {
    let gsi_config_name = "gamestate_integration_music_control.cfg";
    let gsi_config = formatdoc! {r#"
        "Music Control v.1"
        {{
            "uri" "http://127.0.0.1:{port}"
            "timeout" "5.0"
            "buffer"  "0.1"
            "throttle" "0.5"
            "heartbeat" "60.0"
            "data"
            {{
                "provider"            "1"      // general info about client being listened to: game name, appid, client steamid, etc.
                "map"                 "1"      // map, gamemode, and current match phase ('warmup', 'intermission', 'gameover', 'live') and current score
                "round"               "1"      // round phase ('freezetime', 'over', 'live'), bomb state ('planted', 'exploded', 'defused'), and round winner (if any)
                "player_id"           "1"      // player name, clan tag, observer slot (ie key to press to observe this player) and team
                "player_state"        "1"      // player state for this current round such as health, armor, kills this round, etc.
            }}
        }}"#, port = port};

    let mut file = File::create(format!("{}/csgo/cfg/{}", cs_folder_path, gsi_config_name))?;

    file.write_all(gsi_config.as_bytes())?;

    Ok(())
}

async fn handle_payload(
    Extension(state): Extension<Arc<RwLock<AppState>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `Payload` type
    Json(payload): Json<Payload>,
) -> StatusCode {
    match payload.round.phase {
        RoundPhase::FreezeTime | RoundPhase::Over => {
            play_pause(state, false).await;
        }
        RoundPhase::Live => {
            play_pause(state, is_alive(payload.clone())).await;
        }
    }

    // this will be converted into a JSON response
    // with a status code of `200 OK`
    StatusCode::OK
}

// check if player has 0 health or is spectating
fn is_alive(payload: Payload) -> bool {
    payload.provider.steam_id == payload.player.steam_id && payload.player.state.unwrap().health > 0
}

async fn play_pause(state: Arc<RwLock<AppState>>, pause: bool) {
    let r_state = state.read().await;
    if (!pause && !r_state.is_playing) || (pause && r_state.is_playing) {
        drop(r_state);

        println!(
            "[{}] {}",
            Local::now().format("%H:%M:%S"),
            if pause { "Pausing..." } else { "Playing..." }
        );

        let mut w_state = state.write().await;
        w_state.enigo.key_click(Key::MediaPlayPause);
        w_state.is_playing = !pause;

        drop(w_state);
    } else {
        drop(r_state);
    }
}
