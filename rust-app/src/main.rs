use postgres::{Client, NoTls};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::time::Instant;

mod creds;

#[derive(Debug)]
struct Track {
    track_id: String,
    track_name: String,
    track_duration: i64, 
    danceability: f64,
    energy: f64,
    key: i64,
    loudness: f64, 
    mode: i64,
    speechiness: f64,
    acousticness: f64,
    instrumentalness: f64,
    liveness: f64,
    valence: f64,
    tempo: f64,
    time_signature: i64
}

#[derive(Debug)]
struct Song {
    track_id: String,
    track_duration: f64, 
    danceability: f64,
    energy: f64,
    key: f64,
    loudness: f64, 
    mode: f64,
    speechiness: f64,
    acousticness: f64,
    instrumentalness: f64,
    liveness: f64,
    valence: f64,
    tempo: f64,
    time_signature: f64,
    default: bool,
}

impl Song {
    fn default_init() -> Self {
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

    fn get_diff(&self, s: &Song) -> f64 {
        return f64::powi(self.track_duration-s.track_duration,2)+f64::powi(self.danceability-s.danceability,2)+f64::powi(self.energy-s.energy,2)+f64::powi(self.key-s.key,2)+f64::powi(self.loudness-s.loudness,2)+f64::powi(self.mode-s.mode,2)+f64::powi(self.speechiness-s.speechiness,2)+f64::powi(self.acousticness-s.acousticness,2)+f64::powi(self.instrumentalness-s.instrumentalness,2)+f64::powi(self.liveness-s.liveness,2)+f64::powi(self.valence-s.valence,2)+f64::powi(self.tempo-s.tempo,2)+f64::powi(self.time_signature-s.time_signature,2);
    }
}

fn main() {

    let host = crate::creds::creds::gethost();
    let user = crate::creds::creds::getuser();
    let passwd = crate::creds::creds::getpasswd();
    let db_name = crate::creds::creds::getdbname();
    let port = crate::creds::creds::getport();

    let start = Instant::now();
    let mut client = Client::connect(
        &format!("postgresql://{user}:{passwd}@{host}:{port}/{db_name}"),
        NoTls,
    ).unwrap();
    let mut duration = start.elapsed();
    println!("time taken for db connection {:?}",duration);
    let mut track: Song;

    let track_id = "2f9NLCoIaiIn7rZnH9mdir";
    let cluster = get_cluster(track_id, &mut client);
    duration = start.elapsed() - duration;
    println!("time taken to get cluster {:?}",duration);
    track = get_song(track_id, &mut client);
    println!("{:#?}",track);
    duration = start.elapsed() - duration;
    println!("time taken to get track {:?}",duration);
    let searchSpace: Vec<Song> = get_searchspace(track_id, &mut client, cluster, true);
    duration = start.elapsed() - duration;
    println!("time taken to get searchspace {:?}",duration);
    let mut scores: Vec<(String, f64)> = score_on_searchspace(&searchSpace, track,1.0,1.0,1.0);
    duration = start.elapsed() - duration;
    println!("time taken to score on searchspace {:?}",duration);
    let mut ans = get_unscaled_features(scores, 25, &mut client);
    println!("closest songs unscaled features!");
    println!("{:#?}",ans);
    duration = start.elapsed() - duration;
    println!("time taken to get unscaled features of tracks {:?}",duration);
    println!("total time {:?}",start.elapsed());
    println!("Hello, world!");
}

fn get_cluster(track_id: &str, client: &mut Client) -> i64 {
    let query = format!("select * from trackcluster where track_id='{track_id}'");
    let mut cluster: i64 = 0;
    if let Ok(res) = client.query(&query, &[]) {
        let row = &res[0];
        cluster = row.get::<&str,i64>("cluster");
    }
    return cluster;
}

fn get_song(track_id: &str, client: &mut Client) -> Song {
    let query = format!("select * from trackfeatures where track_id='{track_id}'");
    if let Ok(res) = client.query(&query, &[]) {
        let row = &res[0];
        return form_song(row);
    } else { return Song::default_init();}
}

fn form_song(row: &postgres::Row) -> Song{
    //println!("{:?}",row);
    let song = Song {
        track_id: row.get::<&str,String>("track_id"),
        track_duration: row.get::<&str,f64>("track_duration"), 
        danceability: row.get::<&str,f64>("danceability"),
        energy: row.get::<&str,f64>("energy"),
        key: row.get::<&str,f64>("key"),
        loudness: row.get::<&str,f64>("loudness"), 
        mode: row.get::<&str,f64>("mode"),
        speechiness: row.get::<&str,f64>("speechiness"),
        acousticness: row.get::<&str,f64>("acousticness"),
        instrumentalness: row.get::<&str,f64>("instrumentalness"),
        liveness: row.get::<&str,f64>("liveness"),
        valence: row.get::<&str,f64>("valence"),
        tempo: row.get::<&str,f64>("tempo"),
        time_signature: row.get::<&str,f64>("time_signature"),
        default: false
    };
    return song;
}

fn form_track(row: &postgres::Row) -> Track {
    let track = Track {
        track_id: row.get::<&str,String>("track_id"),
        track_name: row.get::<&str,String>("track_name"),
        track_duration: row.get::<&str,i64>("track_duration"), 
        danceability: row.get::<&str,f64>("danceability"),
        energy: row.get::<&str,f64>("energy"),
        key: row.get::<&str,i64>("key"),
        loudness: row.get::<&str,f64>("loudness"), 
        mode: row.get::<&str,i64>("mode"),
        speechiness: row.get::<&str,f64>("speechiness"),
        acousticness: row.get::<&str,f64>("acousticness"),
        instrumentalness: row.get::<&str,f64>("instrumentalness"),
        liveness: row.get::<&str,f64>("liveness"),
        valence: row.get::<&str,f64>("valence"),
        tempo: row.get::<&str,f64>("tempo"),
        time_signature: row.get::<&str,i64>("time_signature")
    };
    return track;
}

fn get_searchspace(track_id: &str, client: &mut Client, cluster: i64, include_explicit: bool) -> Vec<Song> {
    let cluster = cluster.to_string();
    let mut ans = vec![Song::default_init()];
    let mut query = format!("select * from trackfeatures where track_id in (select distinct(track_id) from trackplaylist where pid in (select distinct(pid) from trackcluster where cluster = {cluster}))");
    if !include_explicit {
        query = format!("{query} and explicit=False");
    }
    if let Ok(res) = client.query(&query, &[]) {
        for row in &res {
            ans.push(form_song(row));
        }
    }
    return ans;
}


fn score_on_searchspace(searchSpace: &Vec<Song>, input_song: Song, loudness: f64, tempo: f64, danceability: f64) -> Vec<(String, f64)> {
    let mut scores: Vec<(String, f64)> = Vec::new();
    for song in searchSpace {
        scores.push((String::from(&song.track_id), input_song.get_diff(&song)));
    }
    scores.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
    return scores;
}

fn get_unscaled_features(scores: Vec<(String, f64)>, num_songs: usize, client: &mut Client) -> Vec<Track> {
    let mut tracklist = String::from("(");
    let mut closest_songs: Vec<&str> = Vec::new();
    for x in &scores[..num_songs] {
        //println!("{}",x.0);
        tracklist.push_str("'");
        tracklist.push_str(&x.0);
        tracklist.push_str("',");
    }
    tracklist.pop();
    tracklist.push_str(")");
    //println!("{:?}",tracklist);
    let mut query = "select * from trackftsunscaled where track_id in ";
    let binding = query.to_owned()+&tracklist;
    query = &binding;
    //println!("{query}");
    let mut ans: Vec<Track> = Vec::new();
    let res = client.query(query, &[]); 
    //println!("result!");
    //println!("result {:?}",res);
    if let Ok(res) = client.query(query, &[]) {
        for row in &res {
            ans.push(form_track(row));
        }
    }
    return ans;
}