use std::error::Error;
use std::fmt;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Entry {
    pub artists: Vec<String>,
    pub album: String,
    pub title: String,
    pub track_num: u32,
    pub length: u32,
    pub rating: char,
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

impl Entry {
    #[allow(dead_code)]
    pub fn parse_entry(str: &str) -> Result<(Entry, u32), Box<dyn Error>> {
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
        let timestamp: u32 = temp[6]
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
}
