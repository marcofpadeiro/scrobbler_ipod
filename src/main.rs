use std::{collections::HashMap, error::Error, fmt, fs::read_to_string};

const FILE_PATH: &str = ".scrobbler.log";

#[derive(Eq, Hash, PartialEq, Clone)]
struct Entry {
    artists: Vec<String>,
    album: String,
    title: String,
    track_num: u32,
    length: u32,
    rating: char,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {} {} {} {} {}",
            self.artists, self.album, self.title, self.track_num, self.length, self.rating,
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let entries = parse_file(FILE_PATH)?;

    Ok(())
}

fn filter_per_album(entries: HashMap<Entry, Vec<u64>>) -> HashMap<String, Vec<(Entry, Vec<u64>)>> {
    let mut per_album: HashMap<String, Vec<(Entry, Vec<u64>)>> = HashMap::new();

    for (entry, timestamps) in entries {
        per_album
            .entry(entry.album.clone())
            .or_insert_with(Vec::new)
            .push((entry, timestamps));
    }
    per_album
}

fn filter_per_artist(entries: HashMap<Entry, Vec<u64>>) -> HashMap<String, Vec<(Entry, Vec<u64>)>> {
    let mut per_artist: HashMap<String, Vec<(Entry, Vec<u64>)>> = HashMap::new();

    for (entry, timestamps) in entries {
        for artist in &entry.artists {
            per_artist
                .entry(artist.clone())
                .or_insert_with(Vec::new)
                .push((entry.clone(), timestamps.clone()));
        }
    }

    per_artist
}

fn parse_file(file: &str) -> Result<HashMap<Entry, Vec<u64>>, Box<dyn Error>> {
    let mut entries: HashMap<Entry, Vec<u64>> = HashMap::new();

    let lines: Vec<String> = read_to_string(file)?
        .lines()
        .map(|line| line.to_string())
        .collect();

    for line in lines.iter().skip(4) {
        let (line, timestamp) = parse_entry(line)?;
        let entry = entries.entry(line).or_insert(vec![]);

        entry.push(timestamp);
    }

    Ok(entries)
}

fn parse_entry(str: &str) -> Result<(Entry, u64), Box<dyn Error>> {
    let temp: Vec<&str> = str.split('\t').collect();

    let artists = temp[0].split(';').map(|s| s.trim().to_string()).collect();
    let album = temp[1].to_string();
    let title = temp[2].to_string();

    let track_num = temp[3]
        .parse()
        .map_err(|_| format!("Failed to parse track number: {}", temp[3]))?;
    let length = temp[4]
        .parse()
        .map_err(|_| format!("Failed to parse length: {}", temp[4]))?;
    let rating = temp[5]
        .chars()
        .next()
        .ok_or_else(|| format!("Failed to extract rating from: {}", temp[5]))?;
    let timestamp: u64 = temp[6]
        .parse()
        .map_err(|_| format!("Failed to parse timestamp: {}", temp[6]))?;
    Ok((
        Entry {
            artists,
            album,
            title,
            track_num,
            length,
            rating,
        },
        timestamp,
    ))
}
