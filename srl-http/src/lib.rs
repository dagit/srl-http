use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entrant {
    pub displayname: String,
    pub place: i32,
    pub time: i32,
    pub message: Option<String>,
    pub statetext: String,
    pub twitch: String,
    pub trueskill: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entrants {
    pub count: String,
    pub entrants: HashMap<String, Entrant>,
}

pub async fn entrants(url: &str, raceid: &str) -> reqwest::Result<Entrants> {
    let path = format!("{}entrants/{}", url, raceid);
    let entrants: Entrants = reqwest::get(&path)
        .await?
        .json()
        .await?;
    Ok(entrants)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Races {
    pub count: String,
    pub races: Vec<Race>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Race {
    pub id: String,
    pub game: Game,
    pub goal: String,
    pub time: i32,
    pub state: i32,
    pub statetext: String,
    pub filename: String,
    pub numentrants: i32,
    pub entrants: HashMap<String, Entrant>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub abbrev: String,
    pub popularity: f32,
    pub popularityrank: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Games {
    pub count: String,
    pub games: Vec<Game>,
}


pub async fn races(url: &str) -> reqwest::Result<Races> {
    let path = format!("{}races", url);
    let races: Races = reqwest::get(&path)
        .await?
        .json()
        .await?;
    Ok(races)
}

pub async fn race(url: &str, raceid: &str) -> reqwest::Result<Race> {
    let path = format!("{}races/{}", url, raceid);
    let race: Race = reqwest::get(&path)
        .await?
        .json()
        .await?;
    Ok(race)
}

/** Note: This requires authorization */
pub async fn create_race(url: &str, game: &str) -> reqwest::Result<reqwest::Response> {
    let client = reqwest::Client::new();
    let path = format!("{}races", url);
    let mut map = HashMap::new();
    map.insert("game", game.to_owned());
    let res = client.post(&path)
        .json(&map)
        .send()
        .await?;
    Ok(res)
}

pub async fn games(url: &str) -> reqwest::Result<Games> {
    let path = format!("{}games", url);
    let games: Games = reqwest::get(&path)
        .await?
        .json()
        .await?;
    Ok(games)
}

pub async fn game(url: &str, raceid: &str) -> reqwest::Result<Game> {
    let path = format!("{}games/{}", url, raceid);
    let game: Game = reqwest::get(&path)
        .await?
        .json()
        .await?;
    Ok(game)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Goal {
    pub name: String,
    pub toptimes: Vec<TopTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Goals {
    pub game: Game,
    pub goals: Vec<Goal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopTime {
    pub race: i32,
    pub place: i32,
    pub player: String,
    pub time: i32,
    pub message: String,
    pub oldtrueskill: i32,
    pub newtrueskill: i32,
    pub trueskillchange: i32,
    pub oldseasontrueskill: i32,
    pub newseasontrueskill: i32,
    pub seasontrueskillchange: i32,
}

pub async fn goals(url: &str, abbrev: &str) -> reqwest::Result<Goals> {
    let path = format!("{}goals/{}", url, abbrev);
    let goals: Goals = reqwest::get(&path)
        .await?
        .json()
        .await?;
    Ok(goals)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub channel: String,
    pub api: String,
    pub twitter: String,
    pub youtube: String,
    pub country: String,
}

pub async fn player(url: &str, playerid: &str) -> reqwest::Result<Player> {
    let path = format!("{}players/{}", url, playerid);
    let player: Player = reqwest::get(&path)
        .await?
        .json()
        .await?;
    Ok(player)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
