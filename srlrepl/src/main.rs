use srl_http::*;
use rustyline::Editor;
use rustyline::error::ReadlineError;

#[tokio::main]
async fn main() {
    let prompt = "srl> ";
    let mut rl = Editor::<()>::new();
    let mut url: String = "http://api.speedrunslive.com:81/".to_owned();
    loop {
        let readline = rl.readline(prompt);
        match readline {
            Ok(s) if s == "" => continue,
            Ok(s) => {
                rl.add_history_entry(s.clone());
                match s {
                    s if s == "exit" => break,
                    s if s == "url" => {
                        println!("url = {}", url);
                    },
                    s if s == "races" => {
                        let races = races(&url).await;
                        println!("{:#?}", races);
                    }
                    s if s == "games" => {
                        let games = games(&url).await;
                        println!("{:#?}", games);
                    }
                    s if s.starts_with("seturl") => {
                        let (_, rest) = s.split_at("seturl".len()+1);
                        url = rest.to_owned();
                    },
                    s if s.starts_with("startrace") => {
                        let (_, rest) = s.split_at("startrace".len()+1);
                        let game = rest.to_owned();
                        let resp = create_race(&url, &game).await;
                        println!("{:#?}", resp);

                    },
                    s if s.starts_with("entrants") => {
                        let (_, raceid) = s.split_at("entrants".len()+1);
                        let entrants = entrants(&url, &raceid).await;
                        println!("{:#?}", entrants);
                    },
                    s if s.starts_with("race") => {
                        let (_, raceid) = s.split_at("race".len()+1);
                        let race = race(&url, &raceid).await;
                        println!("{:#?}", race);
                    },
                    s if s.starts_with("game") => {
                        let (_, raceid) = s.split_at("game".len()+1);
                        let game = game(&url, &raceid).await;
                        println!("{:#?}", game);
                    },
                    s if s.starts_with("goals") => {
                        let (_, abbrev) = s.split_at("goals".len()+1);
                        let goals = goals(&url, &abbrev).await;
                        println!("{:#?}", goals);
                    },
                    s if s.starts_with("player") => {
                        let (_, playerid) = s.split_at("player".len()+1);
                        let player = player(&url, &playerid).await;
                        println!("{:#?}", player);
                    },
                    s => println!("{}", s),
                };

            },
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted by user");
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            },
        }
    }
}
