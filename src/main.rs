use std::error::Error;

use chrono::{Datelike, Utc};
use scrobbler::{organize_per_album, organize_per_artist, organize_per_artist_albums, parse_file};
use utils::filter_songs_played_year;

const FILE_PATH: &str = ".scrobbler.log";

mod entry;
mod utils;

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn Error>> {
    let songs = parse_file(FILE_PATH)?;
    let this_year_songs = filter_songs_played_year(&songs, Utc::now().year());

    let albums = organize_per_album(&songs);
    let artists = organize_per_artist(&songs);
    let artists_albums = organize_per_artist_albums(&artists);

    Ok(())
}
