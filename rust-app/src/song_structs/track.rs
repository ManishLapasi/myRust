#[derive(Debug)]
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