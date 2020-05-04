use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entrant {
    displayname: String,
    place: i32,
    time: i32,
    message: Option<String>,
    statetext: String,
    twitch: String,
    trueskill: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entrants {
    count: String,
    entrants: HashMap<String, Entrant>,
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
    count: String,
    races: Vec<Race>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Race {
    id: String,
    game: Game,
    goal: String,
    time: i32,
    state: i32,
    statetext: String,
    filename: String,
    numentrants: i32,
    entrants: HashMap<String, Entrant>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    id: i32,
    name: String,
    abbrev: String,
    popularity: f32,
    popularityrank: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Games {
    count: String,
    games: Vec<Game>,
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
    name: String,
    toptimes: Vec<TopTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Goals {
    game: Game,
    goals: Vec<Goal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopTime {
    race: i32,
    place: i32,
    player: String,
    time: i32,
    message: String,
    oldtrueskill: i32,
    newtrueskill: i32,
    trueskillchange: i32,
    oldseasontrueskill: i32,
    newseasontrueskill: i32,
    seasontrueskillchange: i32,
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
    id: i32,
    name: String,
    channel: String,
    api: String,
    twitter: String,
    youtube: String,
    country: String,
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
