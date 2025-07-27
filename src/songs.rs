use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub type SongsJson = HashMap<String, i32>;

pub fn load_songs() {
    let songsjson = Path::new("./songs.json");
    if !songsjson.exists() {
        fs::write(
            songsjson,
            &serde_json::to_string_pretty(&SongsJson::new()).unwrap(),
        )
        .unwrap();
    }
}

pub fn add_song(song: String) {
    let index = File::open("./songs.json").unwrap();
    let mut songs: SongsJson = serde_json::from_reader(index).unwrap();
    let notmutsongs = songs.clone();
    let our_song = notmutsongs.get(&song);
    if our_song.is_none() {
        songs.insert(song.clone(), 1);
    }
    if our_song.is_some() {
        songs.insert(song, our_song.unwrap() + 1);
    }
    let mut index = File::options().write(true).open("./songs.json").unwrap();
    File::write(
        &mut index,
        serde_json::to_string_pretty(&sort_songs(songs))
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
}

pub fn sort_songs(songs: SongsJson) -> SongsJson {
    let mut song_vec: Vec<(String, i32)> = songs.into_iter().collect();
    song_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let sorted_songs: SongsJson = song_vec.into_iter().collect();
    sorted_songs
}
