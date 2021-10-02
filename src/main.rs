use std::error::Error;

use anyhow::Result;
use dotenv::dotenv;
use reddit::RedditApi;

fn main() -> Result<(), ()> {
    dotenv().unwrap();
    let mut reddit_api = RedditApi::new();
    reddit_api
        .authenticate()
        .map_err(|error| println!("{}", error))?;
    let account = reddit_api
        .get_account()
        .map_err(|error| println!("{}", error))?;
    let account_json = serde_json::to_string(&account).map_err(|error| println!("{}", error))?;
    println!("{}", account_json);
    // let following = reddit_api.get_following(100)?;
    // let following_json = serde_json::to_string(&following)?;
    // println!("{}", following_json);
    Ok(())
}
