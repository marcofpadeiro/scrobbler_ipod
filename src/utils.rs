use crate::{Song, SortOrder, get_end_year, get_start_year};
use std::{cmp::Reverse, collections::HashMap};

fn get_time_listened_song(song: &Song) -> u32 {
    (*song).entry.length * (*song).timestamps.len() as u32
}

#[allow(dead_code)]
fn get_plays_song(song: &Song) -> usize {
    song.timestamps.len()
}

pub fn sort_songs_by_plays<'a, T>(entries: &'a [T], order: SortOrder) -> Vec<&'a Song>
where
    T: AsRef<Song>,
{
    let mut refs: Vec<&'a Song> = entries.iter().map(|e| e.as_ref()).collect();

    match order {
        SortOrder::Asc => {
            refs.sort_by_key(|song| song.timestamps.len());
        }
        SortOrder::Desc => {
            refs.sort_by_key(|song| Reverse(song.timestamps.len()));
        }
    }

    refs
}

pub fn sort_map_by_plays<T>(
    entries: &HashMap<String, Vec<&T>>,
    order: SortOrder,
) -> Vec<(String, usize)>
where
    T: AsRef<Song>,
{
    let mut result: Vec<(String, usize)> = entries
        .iter()
        .map(|(key, slice)| {
            let total_plays = slice
                .iter()
                .map(|song_ref| song_ref.as_ref().timestamps.len())
                .sum();
            (key.clone(), total_plays)
        })
        .collect();

    match order {
        SortOrder::Asc => {
            result.sort_by_key(|(_title, plays)| *plays);
        }
        SortOrder::Desc => {
            result.sort_by_key(|(_title, plays)| Reverse(*plays));
        }
    }

    result
}

pub fn filter_songs_played_year<'a>(entries: &'a [Song], year: i32) -> Vec<&'a Song> {
    entries
        .iter()
        .filter(|song| {
            song.timestamps
                .iter()
                .any(|&ts| ts >= get_start_year(year) && ts <= get_end_year(year))
        })
        .collect()
}

pub fn get_minutes_listened<T: AsRef<Song>>(entries: &[T]) -> u32 {
    entries
        .iter()
        .map(|song| get_time_listened_song(song.as_ref()))
        .sum::<u32>()
        / 60
}

pub fn get_plays<T: AsRef<Song>>(entries: &[T]) -> usize {
    entries
        .iter()
        .map(|song| get_plays_song(song.as_ref()))
        .sum::<usize>()
}
