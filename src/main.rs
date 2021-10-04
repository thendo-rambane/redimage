use anyhow::Result;
use dotenv::dotenv;
use reddit::RedditApi;

fn main() -> Result<(), ()> {
    dotenv().unwrap();
    let mut reddit_api = RedditApi::new();
    reddit_api
        .authenticate()
        .map_err(|error| println!("{}", error))?;
    let karma_list = reddit_api
        .user
        .get_account_karma_breakdown()
        .map_err(|error| println!("{}", error))?;

    // let account = reddit_api
    //     .get_account()
    //     .map_err(|error| println!("{}", error))?;
    let karma_list_json =
        serde_json::to_string(&karma_list).map_err(|error| println!("{}", error))?;
    println!("{}", karma_list_json);
    // let following = reddit_api.get_following(100)?;
    // let following_json = serde_json::to_string(&following)?;
    // println!("{}", following_json);
    Ok(())
}
