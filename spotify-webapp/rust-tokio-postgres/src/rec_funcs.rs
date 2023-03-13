use tokio_postgres::{Client};
use serde_json::Result;
use serde::{Deserialize, Serialize};
use self::songstructs::Song;
use self::songstructs::Track;

pub mod songstructs;

pub async fn getres(client: &Client, track_id: String) -> String{
    let cluster = get_cluster(&track_id, &client).await;
    let song = get_song(&track_id, &client).await;
    let searchSpace: Vec<Song> = get_searchspace(&track_id, &client, cluster, true).await;
    let scores: Vec<(String, f64)> = score_on_searchspace(&searchSpace, &song,1.0,1.0,1.0);
    let ans = get_unscaled_features(scores, 25, &client).await;
    return serde_json::to_string(&ans).unwrap();
}

async fn get_cluster(track_id: &str, client: &Client) -> i64 {
    let query = format!("select * from trackcluster where track_id='{track_id}'");
    let res = client.query(&query, &[]).await.unwrap();
    let cluster = (&res[0]).get::<&str,i64>("cluster");
    return cluster;
}

async fn get_song(track_id: &str, client: &Client) -> Song {
    let query = format!("select * from trackfeatures where track_id='{track_id}'");
    let res = client.query(&query, &[]).await.unwrap();
    let row = &res[0];
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

async fn get_searchspace(track_id: &str, client: &Client, cluster: i64, include_explicit: bool) -> Vec<Song> {
    let cluster = cluster.to_string();
    let mut ans: Vec<Song> = Vec::new();
    let mut query = format!("select * from trackfeatures where track_id in (select distinct(track_id) from trackplaylist where pid in (select distinct(pid) from trackcluster where cluster = {cluster}))");
    if !include_explicit {
        query = format!("{query} and explicit=False");
    }
    let res = client.query(&query, &[]).await.unwrap();
    for row in &res {
        ans.push(Song {
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
        });
    }
    return ans;
}

fn score_on_searchspace(searchSpace: &Vec<Song>, input_song: &Song, loudness: f64, tempo: f64, danceability: f64) -> Vec<(String, f64)> {
    let mut scores: Vec<(String, f64)> = Vec::new();
    for song in searchSpace {
        scores.push((String::from(&song.track_id), input_song.get_diff(&song)));
    }
    scores.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
    return scores;
}

async fn get_unscaled_features(scores: Vec<(String, f64)>, num_songs: usize, client: &Client) -> Vec<Track> {
    let mut tracklist = String::from("(");
    let mut closest_songs: Vec<&str> = Vec::new();
    for x in &scores[..num_songs] {
        tracklist.push_str("'");
        tracklist.push_str(&x.0);
        tracklist.push_str("',");
    }
    tracklist.pop();
    tracklist.push_str(")");
    let mut query = "select * from trackftsunscaled where track_id in ";
    let binding = query.to_owned()+&tracklist;
    query = &binding;
    let mut ans: Vec<Track> = Vec::new();
    let res = client.query(query, &[]).await.unwrap();
    for row in &res {
        ans.push(Track {
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
        });
    }
    return ans;
}
