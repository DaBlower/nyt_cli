use serde::Deserialize;
use std::env;
use std::process;
use std::error::Error;

#[derive(Deserialize)]
struct MostPopular {
    status: String,
    results: Vec<Article>
}

#[derive(Deserialize)]
struct Article {
    title: String,
    section: String,
    url: String
}

macro_rules! MOST_POPULAR_URL {
    () => {
        "https://api.nytimes.com/svc/mostpopular/v2/viewed/1.json?api-key={}"
    };
}

fn main() -> Result<(), Box<dyn Error>>{
    dotenvy::dotenv().expect("envs not loaded!");

    let nyt_api: String = env::var("NYT_API").expect("API_KEY must be set in .env");

    println!("Most popular NYT stories\n");

    let client = reqwest::blocking::Client::new();
    
        let url = format!(MOST_POPULAR_URL!(), nyt_api);

        let pop: MostPopular = client
            .get(&url)
            .send()
            .expect("Failed to fetch story")
            .json()
            .expect("Failed to parse story");

        if pop.status != "OK" {
            println!("API request failed");
            process::exit(1);
        }

        for article in pop.results.iter().take(10) {
            let Article { title, section, url } = article;
            println!("{section}, {title}\n{url}\n");
        }
    Ok(())

}