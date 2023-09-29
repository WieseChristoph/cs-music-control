mod payload;

use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use chrono::Local;
use clap::Parser;
use enigo::{Enigo, Key, KeyboardControllable};
use indoc::formatdoc;
use keyvalues_parser::Vdf;
use payload::{round::RoundPhase, Payload};
use std::path::Path;
use std::{fs::File, io::Write, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use winreg::{enums::*, RegKey};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "OPTIONAL_CS_INSTALLATION_PATH", help = "Generate config file for CS", num_args = 0..)]
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

    // generate config and exit if flag is present
    if args.generate_config.is_some() {
        println!("Generating config...");
        let cs_folder_path = match args.generate_config.unwrap() {
            path if path.is_empty() => get_cs_folder_path().expect("Couldn't get CS folder path"),
            path => path.join(" "),
        };
        match generate_config(port, cs_folder_path) {
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

fn generate_config(port: u16, cs_folder_path: String) -> Result<(), Box<dyn std::error::Error>> {
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

    let config_path = format!(
        "{}\\game\\{}\\cfg\\{}",
        cs_folder_path,
        if cs_folder_path.contains("Counter-Strike Global Offensive") {
            "csgo"
        } else {
            "cs2"
        },
        gsi_config_name
    );
    println!("Creating config at: {}", config_path);
    let mut file = File::create(config_path)?;
    file.write_all(gsi_config.as_bytes())?;

    Ok(())
}

fn get_cs_folder_path() -> Result<String, Box<dyn std::error::Error>> {
    let steam_folder_path = get_steam_folder_path();

    // extract library folders from libraryfolders.vdf
    let library_folders_vdf_path = format!("{}/steamapps/libraryfolders.vdf", steam_folder_path);
    let library_folders_vdf_text = std::fs::read_to_string(library_folders_vdf_path)?;
    let library_folders_vdf = Vdf::parse(&library_folders_vdf_text)?;
    let library_folders = library_folders_vdf
        .value
        .get_obj()
        .ok_or("Couldn't get library folders object")?;

    // iterate through library folders and find CS folder path
    for (_key, value) in library_folders {
        let library_folder = value[0]
            .get_obj()
            .ok_or("Couldn't get library folder object")?;
        let apps = library_folder.get("apps").ok_or("Couldn't get apps")?[0]
            .get_obj()
            .ok_or("Couldn't get apps object")?;

        // check if CS is installed in this library folder
        if apps.contains_key("730") {
            let library_folder_path = library_folder
                .get("path")
                .ok_or("Couldn't get library path")?[0]
                .get_str()
                .ok_or("Couldn't get library path string")?;

            // check for both CS:GO and CS 2 folders
            for dir_name in ["Counter-Strike Global Offensive", "Counter-Strike 2"] {
                let cs_folder_path =
                    format!("{}\\steamapps\\common\\{}", library_folder_path, dir_name);
                if Path::new(&cs_folder_path).exists() {
                    return Ok(cs_folder_path);
                }
            }
        }
    }

    Err("Couldn't find CS folder path".into())
}

fn get_steam_folder_path() -> String {
    // get Steam folder path from registry
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let steam_key = hkcu
        .open_subkey("SOFTWARE\\Valve\\Steam")
        .expect("Couldn't open Steam registry key");
    let steam_path: String = steam_key
        .get_value("SteamPath")
        .expect("Couldn't get SteamPath");

    return steam_path;
}

async fn handle_payload(
    Extension(state): Extension<Arc<RwLock<AppState>>>,
    // this argument tells axum to parse the request body
    // as JSON into a `Payload` type
    Json(payload): Json<Payload>,
) -> StatusCode {
    // play/pause music based on round phase and player state
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

fn is_alive(payload: Payload) -> bool {
    // check if player is not spectating and is alive
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
        // simulate media play/pause key press
        w_state.enigo.key_click(Key::MediaPlayPause);
        w_state.is_playing = !pause;

        drop(w_state);
    } else {
        drop(r_state);
    }
}
