use scrobbler::{get_end_year, get_start_year, Song};

fn get_time_listened_song(song: &Song) -> u32 {
    song.entry.length * song.timestamps.len() as u32
}

fn filter_song_daterange(song: &mut Song, start: u32, end: u32) {
    song.timestamps.retain(|&ts| ts >= start && ts <= end);
}

pub fn filter_songs_played_year(entries: &mut Vec<Song>, year: i32) {
    entries.iter_mut().for_each(|song| filter_song_daterange(song, get_start_year(year), get_end_year(year)));

    entries.retain(|song| !song.timestamps.is_empty());
}
pub fn get_minutes_listened(entries: &Vec<Song>) -> u32 {
    entries
        .iter()
        .map(|song| get_time_listened_song(song))
        .sum::<u32>()
        / 60
}

