use futures::TryFutureExt;
use reqwest::{Client, Version};
// #[macro_use] extern crate rocket;
use reqwest::{self, header::AUTHORIZATION, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json;
use std::time::{Duration, Instant};
use std::{env, vec};
use tokio::time;
// use std::future::;

// #[get("/")]
// fn index() -> &'static str{

//     "Hello, world!"
// }

// #[launch]
// fn rocket() -> _{
//     rocket::build().mount("/", routes![index])
// }

fn get_env(k: &str) -> String {
    match env::var_os(k) {
        Some(val) => val.into_string().unwrap(),
        None => String::from(""),
    }
}

trait Spotify {
    async fn get_data(
        credencial: &mut SfCred,
        route: &'static str,
        id: &'static str,
    ) -> serde_json::Map<std::string::String, serde_json::Value> {
        credencial.get_token().await;
        let url = format!("https://api.spotify.com/v1/{}/{}", route, id);
        let header = format!("Bearer {}", credencial.sf_token.access_token);
        let cli = reqwest::Client::new();
        let res = cli
            .get(url)
            .header(AUTHORIZATION, header)
            .send()
            .await
            .unwrap();
        let a: serde_json::Value = res.json().await.unwrap();
        a.as_object().unwrap().clone()
    }
}

impl Spotify for SpotifyAlbum {}

impl SpotifyAlbum {
    async fn new(cred: &mut SfCred, id: &'static str) -> SpotifyAlbum {
        let data = SpotifyAlbum::get_data(cred, "albums", &id).await;
        SpotifyAlbum {
            id: data.get("id").unwrap().to_string(),
            name: data.get("name").unwrap().to_string(),
            tracks: data
                .get("tracks")
                .unwrap()
                .get("items")
                .unwrap()
                .as_array()
                .unwrap()
                .into_iter()
                .map(|x| x.get("name").unwrap().to_string())
                .collect(),
            total_tracks: data.get("total_tracks").unwrap().as_u64().unwrap(),
            images: vec![String::from("value")],
        }
    }
}
#[derive(Deserialize, Debug)]
struct SpotifyAlbum {
    id: String,
    name: String,
    tracks: Vec<String>,
    total_tracks: u64,
    images: Vec<String> //     artists: Vec<SpotifyArtist>,
                         //     genre: Vec<String>,
                         //     uri: String,
}

#[derive(Deserialize, Debug)]
struct SpotifyArtist {
    id: String,
    name: String,
    genres: Vec<String>,
    images: Vec<String>,
    uri: String,
}
#[derive(Deserialize, Debug)]
struct SpotifyTrack {
    name: String,
    album: SpotifyAlbum,
    artists: Vec<SpotifyArtist>,
    duration_ms: u64,
    popularity: u8,
    preview_url: String,
    track_number: u32,
    uri: String,
}

struct SpotifyPlaylist {
    id: String,
    name: String,
    tracks: Vec<SpotifyTrack>,
    image: String,
    uri: String,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Debug)]
struct SfCred {
    sf_id: String,
    sf_sct: String,
    fecth_in: Instant,
    sf_token: TokenResponse,
}

impl SfCred {
    fn new_cred() -> SfCred {
        SfCred {
            sf_id: get_env("sf_id"),
            sf_sct: get_env("sf_sct"),
            fecth_in: Instant::now(),
            sf_token: TokenResponse {
                access_token: String::from(""),
                expires_in: 0,
            },
        }
    }
    async fn get_token(&mut self) -> &SfCred {
        if self.fecth_in.elapsed() >= Duration::from_secs(self.sf_token.expires_in)
            || self.sf_token.expires_in == 0
        {
            let client = reqwest::Client::new();
            let res = client
                .post("https://accounts.spotify.com/api/token")
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(format!(
                    "grant_type=client_credentials&client_id={}&client_secret={}",
                    self.sf_id, self.sf_sct
                ))
                .send()
                .await
                .unwrap();
            self.sf_token = res.json::<TokenResponse>().await.unwrap();
            self
        } else {
            self
        }
    }
}
#[tokio::main]
async fn main() -> () {
    let mut sf_t = SfCred::new_cred();
    // sf_t.get_token().await;

    // println!("first{:?}\n", sf_t);

    let album = SpotifyAlbum::new(&mut sf_t, "2AoerEEC2T9YEpwSCzYpJG");
    print!("{:?}", album.await);
}
