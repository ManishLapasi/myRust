use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Song {
    pub track_id: String,
    pub track_duration: f64, 
    pub danceability: f64,
    pub energy: f64,
    pub key: f64,
    pub loudness: f64, 
    pub mode: f64,
    pub speechiness: f64,
    pub acousticness: f64,
    pub instrumentalness: f64,
    pub liveness: f64,
    pub valence: f64,
    pub tempo: f64,
    pub time_signature: f64,
    pub default: bool,
}

impl Song {
    pub fn default_init() -> Self {
        Self {
            track_id: String::from("default_track"),
            track_duration: -1.0, 
            danceability: -1.0,
            energy: -1.0,
            key: -1.0,
            loudness: -1.0, 
            mode: -1.0,
            speechiness: -1.0,
            acousticness: -1.0,
            instrumentalness: -1.0,
            liveness: -1.0,
            valence: -1.0,
            tempo: -1.0,
            time_signature: -1.0,
            default: true,
        }
    }

    pub fn get_diff(&self, s: &Song) -> f64 {
        return f64::powi(self.track_duration-s.track_duration,2)+f64::powi(self.danceability-s.danceability,2)+f64::powi(self.energy-s.energy,2)+f64::powi(self.key-s.key,2)+f64::powi(self.loudness-s.loudness,2)+f64::powi(self.mode-s.mode,2)+f64::powi(self.speechiness-s.speechiness,2)+f64::powi(self.acousticness-s.acousticness,2)+f64::powi(self.instrumentalness-s.instrumentalness,2)+f64::powi(self.liveness-s.liveness,2)+f64::powi(self.valence-s.valence,2)+f64::powi(self.tempo-s.tempo,2)+f64::powi(self.time_signature-s.time_signature,2);
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Track {
    pub track_id: String,
    pub track_name: String,
    pub track_duration: i64, 
    pub danceability: f64,
    pub energy: f64,
    pub key: i64,
    pub loudness: f64, 
    pub mode: i64,
    pub speechiness: f64,
    pub acousticness: f64,
    pub instrumentalness: f64,
    pub liveness: f64,
    pub valence: f64,
    pub tempo: f64,
    pub time_signature: i64
}