use std::error::Error;

use chrono::{Datelike, Utc};
use scrobbler::{organize_per_album, organize_per_artist, parse_file};
use utils::{filter_songs_played_year, get_minutes_listened};

const FILE_PATH: &str = ".scrobbler.log";

mod entry;
mod utils;

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn Error>> {
    let mut songs = parse_file(FILE_PATH)?;
    filter_songs_played_year(&mut songs, Utc::now().year());

    let albums = organize_per_album(&songs);
    let artists = organize_per_artist(&songs);

    println!(
        "{} minutes listened so far this year",
        get_minutes_listened(&artists["Radiohead"])
    );

    Ok(())
}
