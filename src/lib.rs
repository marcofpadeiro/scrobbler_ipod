use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

use chrono::{TimeZone, Utc};
use entry::Entry;

mod entry;

#[derive(Clone)]
pub struct Song {
    pub entry: Entry,
    pub timestamps: Vec<u32>,
}

pub fn parse_file(file: &str) -> Result<Vec<Song>, Box<dyn Error>> {
    let mut songs_map: HashMap<Entry, Vec<u32>> = HashMap::new();

    read_to_string(file)?
        .lines()
        .skip(4)
        .filter_map(|line| Entry::parse_entry(line).ok())
        .for_each(|(entry, timestamp)| {
            songs_map
                .entry(entry)
                .or_insert_with(Vec::new)
                .push(timestamp);
        });

    let songs = songs_map
        .into_iter()
        .map(|(entry, timestamps)| Song { entry, timestamps })
        .collect();

    Ok(songs)
}

pub fn get_start_year(year: i32) -> u32 {
    Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0)
        .unwrap()
        .timestamp() as u32
}

pub fn get_end_year(year: i32) -> u32 {
    Utc.with_ymd_and_hms(year, 12, 31, 23, 59, 59)
        .unwrap()
        .timestamp() as u32
}

pub fn organize_per_album(entries: &Vec<Song>) -> HashMap<String, Vec<Song>> {
    let mut per_album: HashMap<String, Vec<Song>> = HashMap::new();

    for entry in entries {
        per_album
            .entry(entry.entry.album.clone())
            .or_insert_with(Vec::new)
            .push(entry.clone());
    }
    per_album
}

pub fn organize_per_artist(entries: &Vec<Song>) -> HashMap<String, Vec<Song>> {
    let mut per_artist: HashMap<String, Vec<Song>> = HashMap::new();

    entries.iter().for_each(|entry| {
        entry.entry.artists.iter().for_each(|artist| {
            per_artist
                .entry(artist.clone())
                .or_insert_with(Vec::new)
                .push(entry.clone());
        });
    });

    per_artist
}
