use std::error::Error;

use serde::Deserialize;
use tracing::{info, warn};
use tracing_subscriber::FmtSubscriber;
use tungstenite::{Message, connect};

use crate::songs::{add_song, load_songs};

mod songs;

#[derive(Debug, Deserialize)]
struct Response {
    artist: String,
    song: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    load_songs();
    let (mut socket, response) =
        connect("ws://api.streamabc.net/metadata/channel/atsw_zk5wvrrbjb_e9zy")
            .expect("Can't connect");

    warn!("Connected to the server");
    warn!("Response HTTP code: {}", response.status());
    warn!("Response contains the following headers:");
    for (header, _value) in response.headers() {
        warn!("* {header}");
    }

    loop {
        let msg = socket.read().expect("Error reading message");
        if !matches!(msg, Message::Ping(_)) {
            let msg_string = msg.into_text().unwrap();
            let msg: Response = serde_json::from_str(&msg_string).unwrap();
            info!("Received: {} - {}", msg.artist, msg.song);
            add_song(format!("{} by {}", msg.song, msg.artist));
        }
    }
    // socket.close(None);
}
