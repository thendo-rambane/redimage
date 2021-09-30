use std::error::Error;

use anyhow::Result;
use dotenv::dotenv;
use reddit::RedditApi;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().unwrap();
    let mut reddit_api = RedditApi::new();
    reddit_api.authenticate()?;
    // println!("{:?}", serde_json::to_string(&reddit_api)?);
    // let account = reddit_api.get_account()?;
    // let account_json = serde_json::to_string(&account)?;
    let following = reddit_api.get_following(100)?;
    let following_json = serde_json::to_string(&following)?;
    println!("{}", following_json);
    Ok(())
}
