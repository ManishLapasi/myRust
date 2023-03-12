use postgres::{Client, NoTls};
use std::time::Instant;
use crate::rec_funcs::song_structs::{Song, Track};
use crate::rec_funcs::creds::creds::{gethost, getuser, getpasswd, getdbname, getport};

pub mod creds;
pub mod song_structs;

pub fn get_recomms(track_id: String) {

    let host = gethost();
    let user = getuser();
    let passwd = getpasswd();
    let db_name = getdbname();
    let port = getport();

    let start = Instant::now();
    let mut client = Client::connect(
        &format!("postgresql://{user}:{passwd}@{host}:{port}/{db_name}"),
        NoTls,
    ).unwrap();
    let mut duration = start.elapsed();
    println!("time taken for db connection {:?}",duration);
    let track: Song;

    let cluster = get_cluster(&track_id, &mut client);
    duration = start.elapsed() - duration;
    println!("time taken to get cluster {:?}",duration);
    track = get_song(&track_id, &mut client);
    println!("{:#?}",track);
    duration = start.elapsed() - duration;
    println!("time taken to get track {:?}",duration);
    let searchSpace: Vec<Song> = get_searchspace(&track_id, &mut client, cluster, true);
    duration = start.elapsed() - duration;
    println!("time taken to get searchspace {:?}",duration);
    let scores: Vec<(String, f64)> = score_on_searchspace(&searchSpace, track,1.0,1.0,1.0);
    duration = start.elapsed() - duration;
    println!("time taken to score on searchspace {:?}",duration);
    let ans = get_unscaled_features(scores, 25, &mut client);
    println!("closest songs unscaled features!");
    println!("{:#?}",ans);
    duration = start.elapsed() - duration;
    println!("time taken to get unscaled features of tracks {:?}",duration);
    println!("total time {:?}",start.elapsed());
    println!("Hello, world!");
}

pub fn get_cluster(track_id: &str, client: &mut Client) -> i64 {
    let query = format!("select * from trackcluster where track_id='{track_id}'");
    let mut cluster: i64 = 0;
    if let Ok(res) = client.query(&query, &[]) {
        let row = &res[0];
        cluster = row.get::<&str,i64>("cluster");
    }
    return cluster;
}

pub fn get_song(track_id: &str, client: &mut Client) -> Song {
    let query = format!("select * from trackfeatures where track_id='{track_id}'");
    if let Ok(res) = client.query(&query, &[]) {
        let row = &res[0];
        return form_song(row);
    } else { return Song::default_init();}
}

pub fn form_song(row: &postgres::Row) -> Song{
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

pub fn form_track(row: &postgres::Row) -> Track {
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

pub fn get_searchspace(track_id: &str, client: &mut Client, cluster: i64, include_explicit: bool) -> Vec<Song> {
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


pub fn score_on_searchspace(searchSpace: &Vec<Song>, input_song: Song, loudness: f64, tempo: f64, danceability: f64) -> Vec<(String, f64)> {
    let mut scores: Vec<(String, f64)> = Vec::new();
    for song in searchSpace {
        scores.push((String::from(&song.track_id), input_song.get_diff(&song)));
    }
    scores.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
    return scores;
}

pub fn get_unscaled_features(scores: Vec<(String, f64)>, num_songs: usize, client: &mut Client) -> Vec<Track> {
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